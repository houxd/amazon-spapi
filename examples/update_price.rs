use std::collections::HashMap;

use amazon_spapi::client::{SpapiClient, SpapiConfig};
use amazon_spapi::models::listings_items_2021_08_01::patch_operation::Op;
use amazon_spapi::models::listings_items_2021_08_01::{ListingsItemPatchRequest, PatchOperation};
use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let config = SpapiConfig::from_env()?;
    let client = SpapiClient::new(config.clone())?;

    let seller_id = "YOUR_SELLER_ID";
    let sku = "YOUR_SKU";
    let marketplace_ids = vec!["ATVPDKIKX0DER".to_string()]; // Amazon US Marketplace ID
    let new_price = 42.0; // New price to set

    log::info!(
        "Updating price for SKU: {} in marketplace: {:?}",
        sku,
        marketplace_ids
    );
    match update_listing_price(&client, seller_id, sku, &marketplace_ids, new_price).await {
        Ok(response) => {
            println!(
                "Price update request submitted successfully! {:#?}",
                response
            );
            println!("Submission ID: {}", response.submission_id);
            if let Some(issues) = response.issues {
                if !issues.is_empty() {
                    println!("Issues found:");
                    for issue in issues {
                        println!("  - {}: {}", issue.code, issue.message);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to update price: {}", e);
        }
    }

    let item = get_current_listing(&client, seller_id, sku, &marketplace_ids).await?;
    println!("Current listing information: {:#?}", item);

    Ok(())
}

/// Update the listing price
async fn update_listing_price(
    client: &SpapiClient,
    seller_id: &str,
    sku: &str,
    marketplace_ids: &[String],
    new_price: f64,
) -> Result<amazon_spapi::models::listings_items_2021_08_01::ListingsItemSubmissionResponse> {
    let mut purchasable_offer = HashMap::new();
    purchasable_offer.insert("audience".to_string(), json!("ALL"));
    purchasable_offer.insert("currency".to_string(), json!("USD"));
    purchasable_offer.insert("marketplace_id".to_string(), json!(marketplace_ids[0]));
    purchasable_offer.insert(
        "our_price".to_string(),
        json!([{
            "schedule": [{
                "value_with_tax": new_price
            }]
        }]),
    );

    let mut list_price = HashMap::new();
    list_price.insert("marketplace_id".to_string(), json!(marketplace_ids[0]));
    list_price.insert("currency".to_string(), json!("USD"));
    list_price.insert("value".to_string(), json!(new_price));

    let purchasable_offer_patch = PatchOperation {
        op: Op::Replace,
        path: "/attributes/purchasable_offer".to_string(),
        value: Some(vec![purchasable_offer]),
    };

    let list_price_patch = PatchOperation {
        op: Op::Replace,
        path: "/attributes/list_price".to_string(),
        value: Some(vec![list_price]),
    };

    let patch_request = ListingsItemPatchRequest {
        product_type: "PRODUCT".to_string(),
        patches: vec![purchasable_offer_patch, list_price_patch],
    };

    println!("{:#?}", patch_request);

    client
        .patch_listings_item(
            seller_id,
            sku,
            marketplace_ids.to_vec(),
            patch_request,
            None, // included_data
            None, // Some("VALIDATION_PREVIEW"), // mode - Can use "VALIDATION_PREVIEW" to validate first
            None, // issue_locale
        )
        .await
}

/// Fetch the current listing information for a given SKU
async fn get_current_listing(
    client: &SpapiClient,
    seller_id: &str,
    sku: &str,
    marketplace_ids: &[String],
) -> Result<amazon_spapi::models::listings_items_2021_08_01::Item> {
    client
        .get_listings_item(
            seller_id,
            sku,
            marketplace_ids.to_vec(),
            None,                                                          // issue_locale
            Some(vec!["summaries".to_string(), "attributes".to_string()]), // included_data
        )
        .await
}
