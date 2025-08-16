use amazon_spapi::client::{SpapiClient, SpapiConfig};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SpapiClient::new(SpapiConfig::from_env()?)?;

    // let fee = client.get_fee_for_asin("B0DGJC52FP", 999.0, true).await?;
    // println!("Fee for ASIN B0DGJC52FP: ${:.2}", fee);

    let current_local_time = chrono::Local::now();
    println!("Current local time: {}", current_local_time);
    {
        let fees = client
            .get_fees_for_asins(vec![("B0DGJC52FP".to_string(), 999.0)], true)
            .await?;
        println!("Fees for ASINs: {:?}", fees);
    }
    let current_local_time = chrono::Local::now();
    println!("Current local time: {}", current_local_time);
    {
        let fees = client
            .get_fees_for_asins(vec![("B0DGJC52FP".to_string(), 999.0)], true)
            .await?;
        println!("Fees for ASINs: {:?}", fees);
    }
    let current_local_time = chrono::Local::now();
    println!("Current local time: {}", current_local_time);
    {
        let fees = client
            .get_fees_for_asins(vec![("B0DGJC52FP".to_string(), 999.0)], true)
            .await?;
        println!("Fees for ASINs: {:?}", fees);
    }
    let current_local_time = chrono::Local::now();
    println!("Current local time: {}", current_local_time);

    Ok(())
}
