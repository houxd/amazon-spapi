use amazon_spapi::client::rate_limiter::RateLimiter;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    println!("Testing rate limiter with safety factor");

    // Create rate limiter with default safety factor (1.05 = 5% buffer)
    let rate_limiter = RateLimiter::new();
    println!("Default safety factor: {}", rate_limiter.get_safety_factor());

    // Create rate limiter with custom safety factor (1.2 = 20% buffer)
    let mut conservative_limiter = RateLimiter::new_with_safety_factor(1.2);
    println!("Conservative safety factor: {}", conservative_limiter.get_safety_factor());

    // Test changing safety factor
    conservative_limiter.set_safety_factor(1.15);
    println!("Updated safety factor: {}", conservative_limiter.get_safety_factor());

    // Simulate API calls with timing measurements
    let endpoint = "test_endpoint";
    let rate = 2.0; // 2 requests per second
    let burst = 3;

    println!("\nTesting with endpoint: {}, rate: {}/s, burst: {}", endpoint, rate, burst);

    for i in 1..=5 {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        
        // Wait for token
        let guard = conservative_limiter.wait(endpoint, rate, burst).await?;
        
        let after_wait = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        
        // Simulate API call duration
        sleep(Duration::from_millis(50)).await;
        
        // Mark response received
        guard.mark_response().await;
        
        let after_response = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        
        println!(
            "Request {}: wait_time={}ms, api_call={}ms, total={}ms", 
            i,
            after_wait - start,
            after_response - after_wait,
            after_response - start
        );
    }

    // Show current token status
    let status = conservative_limiter.get_token_status().await?;
    for (endpoint, (tokens, rate, burst)) in status {
        println!("Endpoint {}: {:.2} tokens, rate: {}/s, burst: {}", endpoint, tokens, rate, burst);
    }

    Ok(())
}
