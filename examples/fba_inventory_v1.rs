use amazon_spapi::{
    client::{SpapiClient, SpapiConfig},
    models::fba_inventory::InventorySummary,
};
use anyhow::Result;

async fn get_inventory_summaries_all(
    spapi: &SpapiClient,
    details: Option<bool>,
) -> Result<Vec<InventorySummary>> {
    let mut res: Vec<InventorySummary> = Vec::new();
    let mut next_token: Option<String> = None;

    loop {
        let granularity_type = "Marketplace";
        let granularity_id = spapi.get_marketplace_id();
        let marketplace_ids = vec![spapi.get_marketplace_id().to_string()];

        let inventory = spapi
            .get_inventory_summaries(
                granularity_type,
                granularity_id,
                marketplace_ids,
                details,
                None,
                None,
                None,
                next_token.as_deref(),
            )
            .await?;
        log::debug!("Fetched inventory summaries successfully: {:?}", inventory);

        if let Some(payload) = inventory.payload {
            res.extend(payload.inventory_summaries);
        }

        if let Some(pagination) = inventory.pagination {
            next_token = pagination.next_token;
        } else {
            break;
        }
    }

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = SpapiClient::new(SpapiConfig::from_env()?)?;

    let inventory_summaries = get_inventory_summaries_all(&client, Some(false)).await?;
    log::info!("Fetched {} inventory summaries", inventory_summaries.len());
    for summary in inventory_summaries {
        log::info!("Inventory Summary: {:?}", summary);
    }

    Ok(())
}
