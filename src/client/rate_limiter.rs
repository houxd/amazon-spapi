use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// RAII guard that automatically records response when dropped
#[must_use = "RateLimitGuard must be held until the API response is received"]
pub struct RateLimitGuard {
    rate_limiter: Arc<Mutex<HashMap<String, TokenBucketState>>>,
    identifier: String,
    auto_record: bool,
}

impl RateLimitGuard {
    fn new(
        rate_limiter: Arc<Mutex<HashMap<String, TokenBucketState>>>,
        identifier: String,
        auto_record: bool,
    ) -> Self {
        Self {
            rate_limiter,
            identifier,
            auto_record,
        }
    }

    /// Manually mark that the API response was received
    /// This will record the response time and prevent automatic recording on drop
    pub async fn mark_response(mut self) {
        if self.auto_record {
            let mut buckets = self.rate_limiter.lock().await;
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            if let Some(bucket) = buckets.get_mut(&self.identifier) {
                bucket.last_response_time = Some(now);
                log::trace!(
                    "Manually recorded response time for {}: {}",
                    self.identifier,
                    now
                );
            }
        }

        // Prevent automatic recording on drop
        self.auto_record = false;
    }
}

impl Drop for RateLimitGuard {
    fn drop(&mut self) {
        if !self.auto_record {
            return;
        }

        let rate_limiter = self.rate_limiter.clone();
        let identifier = self.identifier.clone();

        // Spawn a task to record the response since Drop can't be async
        tokio::spawn(async move {
            let mut buckets = rate_limiter.lock().await;
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            if let Some(bucket) = buckets.get_mut(&identifier) {
                // Always record the response time to ensure accurate rate limiting
                bucket.last_response_time = Some(now);
                log::trace!("Auto-recorded response time for {}: {}", identifier, now);
            } else {
                log::warn!(
                    "Attempted to auto-record response for unknown endpoint: {}",
                    identifier
                );
            }
        });
    }
}

/// State of a token bucket for rate limiting
#[derive(Debug, Clone)]
pub struct TokenBucketState {
    pub tokens: f64,
    pub last_refill: u64,                // Unix timestamp in seconds
    pub last_response_time: Option<u64>, // Unix timestamp in seconds when last response was received
    pub rate: f64,                       // requests per second
    pub burst: u32,                      // maximum burst capacity
    pub initial_burst_used: bool,        // Track if initial burst has been used
}

/// In-memory rate limiter that manages token buckets for different endpoints
/// Thread-safe but not cross-process
pub struct RateLimiter {
    buckets: Arc<Mutex<HashMap<String, TokenBucketState>>>,
    /// Safety factor to add buffer time between requests (1.0 = no buffer, 1.1 = 10% buffer)
    /// Default is 1.05 (5% buffer) to avoid 429 errors due to timing inconsistencies
    safety_factor: f64,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            safety_factor: 1.10, // Default 10% safety buffer
        }
    }

    /// Create a new rate limiter with a custom safety factor
    /// Safety factor > 1.0 adds buffer time between requests
    /// For example, 1.1 means 10% longer wait times
    pub fn new_with_safety_factor(safety_factor: f64) -> Self {
        let factor = if safety_factor < 1.0 {
            log::warn!("Safety factor {} is less than 1.0, using 1.0 instead", safety_factor);
            1.0
        } else {
            safety_factor
        };
        
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            safety_factor: factor,
        }
    }

    /// Set the safety factor for rate limiting
    /// This adds a buffer to prevent 429 errors due to timing inconsistencies
    pub fn set_safety_factor(&mut self, safety_factor: f64) {
        if safety_factor < 1.0 {
            log::warn!("Safety factor {} is less than 1.0, using 1.0 instead", safety_factor);
            self.safety_factor = 1.0;
        } else {
            self.safety_factor = safety_factor;
            log::info!("Rate limiter safety factor set to {}", safety_factor);
        }
    }

    /// Get the current safety factor
    pub fn get_safety_factor(&self) -> f64 {
        self.safety_factor
    }

    /// Wait for a token to become available for the given endpoint and return a guard
    /// When the guard is dropped, record_response will be called automatically
    #[must_use = "The returned guard must be held until the API response is received"]
    pub async fn wait(&self, identifier: &str, rate: f64, burst: u32) -> Result<RateLimitGuard> {
        self._wait_for_token(identifier, rate, burst).await?;

        Ok(RateLimitGuard::new(
            self.buckets.clone(),
            identifier.to_string(),
            true, // auto_record enabled
        ))
    }

    async fn _wait_for_token(&self, identifier: &str, rate: f64, burst: u32) -> Result<()> {
        loop {
            let mut buckets = self.buckets.lock().await;
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

            // Get or create bucket for this endpoint
            let bucket = buckets.entry(identifier.to_string()).or_insert_with(|| {
                log::debug!("Creating new token bucket for endpoint: {}", identifier);
                TokenBucketState {
                    tokens: burst as f64,
                    last_refill: now,
                    last_response_time: None,
                    rate: rate,
                    burst: burst,
                    initial_burst_used: false,
                }
            });

            // Update bucket configuration if changed
            if (bucket.rate - rate).abs() > f64::EPSILON || bucket.burst != burst {
                log::info!(
                    "Updating rate limit for {}: rate {} -> {}, burst {} -> {}",
                    identifier,
                    bucket.rate,
                    rate,
                    bucket.burst,
                    burst
                );
                bucket.rate = rate;
                bucket.burst = burst;
            }

            // Calculate token refill based on appropriate time reference
            let refill_from_time = if bucket.initial_burst_used {
                // After initial burst, use response time for consistent interval enforcement
                bucket.last_response_time.unwrap_or(bucket.last_refill)
            } else {
                // During initial burst, use last_refill
                bucket.last_refill
            };

            let time_passed = now.saturating_sub(refill_from_time) as f64;
            let tokens_to_add = time_passed * bucket.rate;
            let new_tokens = (bucket.tokens + tokens_to_add).min(bucket.burst as f64);

            // Reset burst state if bucket refilled to near capacity
            if new_tokens >= (bucket.burst as f64 - 0.5) && bucket.initial_burst_used {
                bucket.initial_burst_used = false;
                bucket.last_response_time = None;
                log::debug!(
                    "Bucket {} refilled to near capacity ({:.1}), resetting burst state",
                    identifier,
                    new_tokens
                );
            }

            bucket.tokens = new_tokens;
            bucket.last_refill = now;

            // Enforce minimum interval after initial burst
            if bucket.initial_burst_used {
                if let Some(last_response_time) = bucket.last_response_time {
                    let base_minimum_interval = 1.0 / bucket.rate;
                    // Apply safety factor to add buffer time
                    let minimum_interval = base_minimum_interval * self.safety_factor;
                    let time_since_response = now.saturating_sub(last_response_time) as f64;

                    if time_since_response < minimum_interval {
                        let wait_seconds = minimum_interval - time_since_response;
                        log::debug!(
                            "Enforcing minimum interval for {} (safety factor: {:.2}): waiting {:.3}s since last response",
                            identifier,
                            self.safety_factor,
                            wait_seconds
                        );

                        drop(buckets);
                        sleep(Duration::from_secs_f64(wait_seconds)).await;
                        continue;
                    }
                }
            }

            log::debug!(
            "Endpoint {}: {:.1} tokens available, rate: {}/s, burst: {}, initial_burst_used: {}",
            identifier,
            bucket.tokens,
            bucket.rate,
            bucket.burst,
            bucket.initial_burst_used
        );

            // Check if we have a token available
            if bucket.tokens >= 1.0 {
                bucket.tokens -= 1.0;

                // For burst=1, immediately mark initial_burst_used
                if bucket.burst == 1 || bucket.tokens <= 0.0 {
                    bucket.initial_burst_used = true;
                    log::debug!(
                        "Initial burst capacity exhausted for {} (tokens: {:.1}, burst: {})",
                        identifier,
                        bucket.tokens,
                        bucket.burst
                    );
                }

                log::debug!(
                    "Token consumed for {}, {:.1} tokens remaining, initial_burst_used: {}",
                    identifier,
                    bucket.tokens,
                    bucket.initial_burst_used
                );

                return Ok(());
            }

            // Calculate wait time for next token
            let tokens_needed = 1.0 - bucket.tokens;
            let base_wait_seconds = tokens_needed / bucket.rate;
            // Apply safety factor to wait time
            let wait_seconds = base_wait_seconds * self.safety_factor;
            log::debug!(
                "Rate limit reached for {}, need to wait {:.2}s (base: {:.2}s, safety factor: {:.2}) for next token",
                identifier,
                wait_seconds,
                base_wait_seconds,
                self.safety_factor
            );

            // Mark initial burst as exhausted (if not already marked)
            if !bucket.initial_burst_used {
                bucket.initial_burst_used = true;
                log::debug!(
                    "Marking initial burst as used for {} due to token shortage",
                    identifier
                );
            }

            let wait_duration = Duration::from_secs_f64(wait_seconds.max(0.1));

            drop(buckets);
            sleep(wait_duration).await;
        }
    }

    /// Check if a token is available without consuming it
    pub async fn check_token_availability(&self, identifier: &str) -> Result<bool> {
        let mut buckets = self.buckets.lock().await;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        if let Some(bucket) = buckets.get_mut(identifier) {
            // Refill tokens
            let time_passed = now.saturating_sub(bucket.last_refill) as f64;
            let tokens_to_add = time_passed * bucket.rate;
            bucket.tokens = (bucket.tokens + tokens_to_add).min(bucket.burst as f64);
            bucket.last_refill = now;

            Ok(bucket.tokens >= 1.0)
        } else {
            // No bucket exists, so we can create one with full capacity
            Ok(true)
        }
    }

    /// Get current token status for all endpoints
    /// Returns (tokens, rate, burst) for each endpoint
    pub async fn get_token_status(&self) -> Result<HashMap<String, (f64, f64, u32)>> {
        let mut buckets = self.buckets.lock().await;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut status = HashMap::new();

        for (endpoint_key, bucket) in buckets.iter_mut() {
            // Refill tokens before reporting status
            let time_passed = now.saturating_sub(bucket.last_refill) as f64;
            let tokens_to_add = time_passed * bucket.rate;
            bucket.tokens = (bucket.tokens + tokens_to_add).min(bucket.burst as f64);
            bucket.last_refill = now;

            status.insert(
                endpoint_key.clone(),
                (bucket.tokens, bucket.rate, bucket.burst),
            );
        }

        Ok(status)
    }

    /// Reset all rate limiting state (useful for testing)
    pub async fn reset(&self) {
        let mut buckets = self.buckets.lock().await;
        buckets.clear();
        log::debug!("Rate limiter state reset");
    }

    /// Get the number of active buckets
    pub async fn active_buckets_count(&self) -> usize {
        let buckets = self.buckets.lock().await;
        buckets.len()
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
