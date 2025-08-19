use anyhow::Result;
use amazon_spapi::client::{SpapiClient, SpapiConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let config = SpapiConfig::from_env()?;
    let client = SpapiClient::new(config.clone())?;

    let marketplace = client.get_marketplace_participations().await?;
    println!("Marketplace: {:#?}", marketplace);

    Ok(())
}
