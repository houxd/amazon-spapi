use amazon_spapi::client::{SpapiClient, SpapiConfig};
use anyhow::{anyhow, Result};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestockRecommendation {
    #[serde(rename = "Country")]
    pub country: String,

    #[serde(rename = "Product Name")]
    pub product_name: String,

    #[serde(rename = "FNSKU")]
    pub fnsku: String,

    #[serde(rename = "Merchant SKU")]
    pub merchant_sku: String,

    #[serde(rename = "ASIN")]
    pub asin: String,

    #[serde(rename = "Condition")]
    pub condition: String,

    #[serde(rename = "Supplier")]
    pub supplier: String,

    #[serde(rename = "Supplier part no.")]
    pub supplier_part_no: String,

    #[serde(rename = "Currency code")]
    pub currency_code: String,

    #[serde(rename = "Price", deserialize_with = "deserialize_f64")]
    pub price: f64,

    #[serde(rename = "Sales last 30 days", deserialize_with = "deserialize_f64")]
    pub sales_last_30_days: f64,

    #[serde(
        rename = "Units Sold Last 30 Days",
        deserialize_with = "deserialize_u32"
    )]
    pub units_sold_last_30_days: u32,

    #[serde(rename = "Total Units", deserialize_with = "deserialize_u32")]
    pub total_units: u32,

    #[serde(rename = "Inbound", deserialize_with = "deserialize_u32")]
    pub inbound: u32,

    #[serde(rename = "Available", deserialize_with = "deserialize_u32")]
    pub available: u32,

    #[serde(rename = "FC transfer", deserialize_with = "deserialize_u32")]
    pub fc_transfer: u32,

    #[serde(rename = "FC Processing", deserialize_with = "deserialize_u32")]
    pub fc_processing: u32,

    #[serde(rename = "Customer Order", deserialize_with = "deserialize_u32")]
    pub customer_order: u32,

    #[serde(rename = "Unfulfillable", deserialize_with = "deserialize_u32")]
    pub unfulfillable: u32,

    #[serde(rename = "Working", deserialize_with = "deserialize_u32")]
    pub working: u32,

    #[serde(rename = "Shipped", deserialize_with = "deserialize_u32")]
    pub shipped: u32,

    #[serde(rename = "Receiving", deserialize_with = "deserialize_u32")]
    pub receiving: u32,

    #[serde(rename = "Fulfilled by")]
    pub fulfilled_by: String,

    #[serde(
        rename = "Total Days of Supply (including units from open shipments)",
        deserialize_with = "deserialize_u32"
    )]
    pub total_days_of_supply: u32,

    #[serde(
        rename = "Days of Supply at Amazon Fulfillment Network",
        deserialize_with = "deserialize_u32"
    )]
    pub days_of_supply_at_afn: u32,

    #[serde(rename = "Alert")]
    pub alert: String,

    #[serde(
        rename = "Recommended replenishment qty",
        deserialize_with = "deserialize_u32"
    )]
    pub recommended_replenishment_qty: u32,

    #[serde(rename = "Recommended ship date")]
    pub recommended_ship_date: String,

    #[serde(rename = "Recommended action")]
    pub recommended_action: String,

    #[serde(rename = "Unit storage size")]
    pub unit_storage_size: String,
}

fn deserialize_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let cleaned = s.trim();
    if cleaned.is_empty() {
        return Ok(0.0);
    }

    let cleaned = cleaned.trim_end_matches('+');
    cleaned
        .parse::<f64>()
        .map_err(|_| anyhow!("Failed to parse number: '{}'", s))
        .map_err(serde::de::Error::custom)
}

fn deserialize_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let cleaned = s.trim();
    if cleaned.is_empty() {
        return Ok(0);
    }

    let cleaned = if cleaned.ends_with('+') {
        cleaned.trim_end_matches('+')
    } else {
        cleaned
    };

    let result = cleaned.parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Warning: Could not parse integer '{}', using 0", s);
        0
    });

    Ok(result)
}

pub fn parse_restock_report(csv_content: &str) -> Result<Vec<RestockRecommendation>> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    let mut recommendations = Vec::new();

    for (line_num, result) in reader.deserialize().enumerate() {
        match result {
            Ok(record) => {
                recommendations.push(record);
            }
            Err(e) => {
                return Err(anyhow!("Failed to parse CSV at line {}: {}", line_num + 2, e));
            }
        }
    }

    println!(
        "Successfully parsed {} recommendations",
        recommendations.len()
    );
    Ok(recommendations)
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let spapi_config = SpapiConfig::from_env()?;
    let client = SpapiClient::new(spapi_config.clone())?;
    let marketplace_ids = vec!["ATVPDKIKX0DER".to_string()]; // Amazon US Marketplace ID

    // create report specification
    let report_type = "GET_RESTOCK_INVENTORY_RECOMMENDATIONS_REPORT";
    let report_content = client
        .fetch_report(
            report_type,
            marketplace_ids.clone(),
            None,
            Some(|attempt, status| {
                println!("Attempt get report {}: {:?}", attempt, status);
            }),
        )
        .await?;

    // save report content to a file
    let report_file_path = "/tmp/restock_inventory_report.txt";
    std::fs::write(report_file_path, &report_content)
        .expect("Unable to write report content to file");

    // // load report content from the file
    // let report_file_path = "/tmp/restock_inventory_report.txt";
    // let report_content =
    //     std::fs::read_to_string(report_file_path).expect("Unable to read report content from file");

    let restocks = parse_restock_report(&report_content);

    println!("Restock inventory report generated successfully.");
    println!("{:#?}", restocks);

    Ok(())
}
