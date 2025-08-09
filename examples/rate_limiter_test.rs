use std::{thread::sleep, time::Duration};

use amazon_spapi::client::rate_limiter;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let rate_limiter = rate_limiter::RateLimiter::new();

    for i in 1..=5 {
        let _ = rate_limiter.wait("func1", 1.0, 15).await?;
        println!("call {}, time: {}", i, chrono::Local::now());
    }

    sleep(Duration::from_secs(5));

    for i in 1..=20 {
        let _ = rate_limiter.wait("func1", 1.0, 15).await?;
        println!("call {}, time: {}", i, chrono::Local::now());
    }

    sleep(Duration::from_secs(15));


    for i in 1..=20 {
        let _ = rate_limiter.wait("func1", 1.0, 15).await?;
        println!("call {}, time: {}", i, chrono::Local::now());
    }

    Ok(())
}
