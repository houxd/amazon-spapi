use anyhow::Result;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::client::ApiEndpoint;

/// State of a token bucket for rate limiting
#[derive(Debug, Clone)]
pub struct TokenBucketState {
    pub tokens: f64,
    pub last_refill: u64, // Unix timestamp in seconds
    pub last_response_time: Option<u64>, // Unix timestamp in seconds when last response was received
    pub rate: f64,        // requests per second
    pub burst: u32,       // maximum burst capacity
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

    /// Wait for a token to become available for the given endpoint
    /// This method will block until a token is available
    pub async fn wait_for_token(&self, identifier: &str, rate: f64, burst: u32) -> Result<()> {
        loop {
            {
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
                    }
                });

                // Update bucket configuration if endpoint configuration changed
                if (bucket.rate - rate).abs() > f64::EPSILON || bucket.burst != burst {
                    log::info!("Updating rate limit for {}: rate {} -> {}, burst {} -> {}", 
                        identifier, bucket.rate, rate, bucket.burst, burst);
                    bucket.rate = rate;
                    bucket.burst = burst;
                }

                // Refill tokens based on time passed
                let time_passed = now.saturating_sub(bucket.last_refill) as f64;
                let tokens_to_add = time_passed * bucket.rate; // rate tokens per second
                bucket.tokens = (bucket.tokens + tokens_to_add).min(bucket.burst as f64);
                bucket.last_refill = now;

                // Check if we need to wait based on the minimum interval since last response
                if let Some(last_response_time) = bucket.last_response_time {
                    let minimum_interval = 1.0 / bucket.rate; // minimum seconds between requests
                    let time_since_response = now.saturating_sub(last_response_time) as f64;
                    
                    if time_since_response < minimum_interval {
                        let wait_seconds = minimum_interval - time_since_response;
                        log::debug!("Enforcing minimum interval for {}: waiting {:.3}s since last response", 
                            identifier, wait_seconds);
                        
                        // Release lock and wait
                        drop(buckets);
                        sleep(Duration::from_secs_f64(wait_seconds)).await;
                        continue; // Retry after waiting
                    }
                }

                log::trace!("Endpoint {}: {:.2} tokens available, rate: {}/s, burst: {}", 
                    identifier, bucket.tokens, bucket.rate, bucket.burst);

                // Check if we have a token available
                if bucket.tokens >= 1.0 {
                    bucket.tokens -= 1.0;
                    
                    log::debug!("Token consumed for {}, {:.2} tokens remaining", 
                        identifier, bucket.tokens);
                    
                    return Ok(());
                }

                // Calculate wait time for next token
                let wait_time = Duration::from_secs_f64(1.0 / bucket.rate);
                log::debug!("Rate limit reached for {}, waiting {:?}", 
                    identifier, wait_time);
            } // Release lock here

            // Sleep outside the lock to allow other tasks to proceed
            sleep(Duration::from_millis(100)).await; // Check every 100ms
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

            status.insert(endpoint_key.clone(), (bucket.tokens, bucket.rate, bucket.burst));
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
    pub async fn record_response(&self, identifier: &str) -> Result<()> {
        let mut buckets = self.buckets.lock().await;
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        
        if let Some(bucket) = buckets.get_mut(identifier) {
            bucket.last_response_time = Some(now);
            log::trace!("Recorded response time for {}: {}", identifier, now);
        } else {
            log::warn!("Attempted to record response for unknown endpoint: {}", identifier);
        }
        
        Ok(())
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
