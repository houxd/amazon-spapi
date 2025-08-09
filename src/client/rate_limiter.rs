use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::client::ApiEndpoint;

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
                // Only record response time after initial burst is exhausted
                if bucket.initial_burst_used || bucket.tokens < (bucket.burst as f64 * 0.5) {
                    bucket.last_response_time = Some(now);
                    log::trace!("Auto-recorded response time for {}: {}", identifier, now);
                } else {
                    log::trace!(
                        "Skipping response time recording for {} (still in initial burst)",
                        identifier
                    );
                }
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
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
        }
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

    /// Wait for a token to become available for the given endpoint
    /// This method will block until a token is available
    #[deprecated(since = "0.1.4", note = "Use `wait()` instead.")]
    pub async fn wait_for_token(&self, identifier: &str, rate: f64, burst: u32) -> Result<()> {
        self._wait_for_token(identifier, rate, burst).await
    }

    async fn _wait_for_token(&self, identifier: &str, rate: f64, burst: u32) -> Result<()> {
        loop {
            let mut buckets = self.buckets.lock().await;
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

            // Get or create bucket for this endpoint
            let bucket = buckets.entry(identifier.to_string()).or_insert_with(|| {
                log::debug!("Creating new token bucket for endpoint: {}", identifier);
                TokenBucketState {
                    tokens: burst as f64, // Start with full burst capacity
                    last_refill: now,
                    last_response_time: None, // No response received yet
                    rate: rate,
                    burst: burst,
                    initial_burst_used: false, // Mark that we haven't used the initial burst yet
                }
            });

            // Update bucket configuration if endpoint configuration changed
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

            // 始终进行token refill计算
            let time_passed = now.saturating_sub(bucket.last_refill) as f64;
            let tokens_to_add = time_passed * bucket.rate;
            let new_tokens = (bucket.tokens + tokens_to_add).min(bucket.burst as f64);

            // 如果token恢复到了接近满容量，重置initial_burst_used标志
            if new_tokens >= (bucket.burst as f64 - 0.5) && bucket.initial_burst_used {
                bucket.initial_burst_used = false;
                bucket.last_response_time = None; // 清除响应时间记录
                log::debug!(
                    "Bucket {} refilled to near capacity ({:.1}), resetting burst state",
                    identifier,
                    new_tokens
                );
            }

            bucket.tokens = new_tokens;
            bucket.last_refill = now;

            // Only check minimum interval if initial burst has been used AND we have recent response time
            if bucket.initial_burst_used {
                if let Some(last_response_time) = bucket.last_response_time {
                    let minimum_interval = 1.0 / bucket.rate;
                    let time_since_response = now.saturating_sub(last_response_time) as f64;

                    if time_since_response < minimum_interval {
                        let wait_seconds = minimum_interval - time_since_response;
                        log::debug!(
                            "Enforcing minimum interval for {}: waiting {:.3}s since last response",
                            identifier,
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

                // 只有当token数量降到很低时才标记initial_burst已用完
                if bucket.tokens <= 1.0 && !bucket.initial_burst_used {
                    bucket.initial_burst_used = true;
                    log::debug!(
                        "Initial burst capacity exhausted for {} (tokens: {:.1})",
                        identifier,
                        bucket.tokens
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
            let wait_seconds = tokens_needed / bucket.rate;
            log::debug!(
                "Rate limit reached for {}, need to wait {:.2}s for next token",
                identifier,
                wait_seconds
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

    /// Record that a response was received for the given endpoint
    /// This updates the last_response_time used for enforcing minimum intervals
    #[deprecated(since = "0.1.4", note = "Use `wait()` instead.")]
    pub async fn record_response(&self, identifier: &str) -> Result<()> {
        let mut buckets = self.buckets.lock().await;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        if let Some(bucket) = buckets.get_mut(identifier) {
            bucket.last_response_time = Some(now);
            bucket.initial_burst_used = true; // Mark that we've moved past initial burst
            log::trace!("Recorded response time for {}: {}", identifier, now);
        } else {
            log::warn!(
                "Attempted to record response for unknown endpoint: {}",
                identifier
            );
        }

        Ok(())
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
