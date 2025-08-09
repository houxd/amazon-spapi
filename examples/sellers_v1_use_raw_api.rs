use amazon_spapi::apis::sellers_api::get_marketplace_participations;
use amazon_spapi::client::{SpapiClient, SpapiConfig};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let spapi_config = SpapiConfig::from_env()?;
    let spapi_client = SpapiClient::new(spapi_config.clone())?;
    {
        // Internally refresh the access token and create a configuration
        // Configuration must be created for each API call
        let configuration = spapi_client.create_configuration().await?;

        // Wait for rate limit before making the API call
        // When _guard is dropped, the rate limiter will mark the api call as having received a response
        let _guard = spapi_client
            .limiter()
            .wait("get_marketplace_participations", 0.016, 15)
            .await?;

        // Call the API to get marketplace participations
        let res = get_marketplace_participations(&configuration).await?;
        
        println!("Marketplace Participations: {:#?}", res);
    }
    Ok(())
}
