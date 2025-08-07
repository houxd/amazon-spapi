use amazon_spapi::client::rate_limiter;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rate_limiter = rate_limiter::RateLimiter::new(); // 5 requests per second
    {
        let _ = rate_limiter.wait("func1", 0.5, 1).await?;
        println!("Function 1 executed once");
    }
    {
        let _ = rate_limiter.wait("func1", 0.5, 1).await?;
        println!("Function 1 executed twice");
    }
    {
        let _ = rate_limiter.wait("func1", 0.5, 1).await?;
        println!("Function 1 executed thrice");
    }
    {
        let _ = rate_limiter.wait("func1", 0.5, 1).await?;
        println!("Function 1 executed four times");
    }

    Ok(())
}
