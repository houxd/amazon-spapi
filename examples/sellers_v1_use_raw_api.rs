use amazon_spapi::apis::sellers_api::get_marketplace_participations;
use amazon_spapi::client::{SpapiClient, SpapiConfig};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let spapi_config = SpapiConfig::from_env()?;
    let client = SpapiClient::new(spapi_config.clone())?;
    let configuration = client.create_configuration().await?;
    client
        .limiter()
        .wait_for_token("get_marketplace_participations", 0.016, 15)
        .await?;
    let res = get_marketplace_participations(&configuration).await;
    client.limiter()
        .record_response("get_marketplace_participations")
        .await?;
    println!("Marketplace Participations: {:#?}", res);
    Ok(())
}
