use amazon_spapi::client::RateLimiter;
use anyhow::Result;
use std::time::Duration;

async fn call_api_with_error(
    i: usize,
    rate_limiter: &RateLimiter,
    should_fail: bool,
) -> Result<()> {
    let guard = rate_limiter.wait("func1", 0.1, 1).await?;

    println!("API call {} start at: {}", i, chrono::Local::now());

    if should_fail {
        return Err(anyhow::anyhow!("API call failed"));
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("API call {} end at  : {}", i, chrono::Local::now());

    guard.mark_response().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let rate_limiter = RateLimiter::new();

    if let Err(e) = call_api_with_error(1, &rate_limiter, false).await {
        println!("Call 1 failed: {}", e);
    }

    if let Err(e) = call_api_with_error(2, &rate_limiter, true).await {
        println!("Call 2 failed: {}", e);
    }

    if let Err(e) = call_api_with_error(3, &rate_limiter, false).await {
        println!("Call 3 failed: {}", e);
    }

    Ok(())
}
