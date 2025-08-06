use anyhow::Result;
use amazon_spapi::client::{SpapiClient, SpapiConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let client = SpapiClient::new(SpapiConfig::from_env()?)?;

    let fee = client.get_fee_for_asin("B0DGJC52FP", 999.0, true).await?;
    println!("Fee for ASIN B0DGJC52FP: ${:.2}", fee);

    Ok(())
}
