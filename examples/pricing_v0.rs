use anyhow::Result;
use amazon_spapi::{client::{SpapiClient, SpapiConfig}, marketplace_ids};

#[tokio::main]
async fn main() -> Result<()> {
    let client = SpapiClient::new(SpapiConfig::from_env()?)?;

    // let res = client
    //     .get_item_offers(client.get_marketplace_id(), "B0DGJC52FP", "New", None)
    //     .await?;
    // println!("Item offers: {:?}", res);

    let res = client
        .get_item_offers_batch_by_asins(vec!["B0DGJC52FP", "B0BN72FYFG"], marketplace_ids::US)
        .await?;
    println!("Batch item offers: {:?}", res);

    // let res = client
    //     .get_listing_offers_batch_by_skus(vec!["YOU_SKU1", "YOU_SKU2"])
    //     .await?;
    // println!("Batch listing offers: {:?}", res);
    

    Ok(())
}
