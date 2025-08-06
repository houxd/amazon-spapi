use anyhow::Result;

use crate::{
    apis::fba_inventory_api::get_inventory_summaries,
    client::{ApiEndpoint, ApiMethod, SpapiClient},
    models::fba_inventory::{GetInventorySummariesResponse, InventoryItem},
};

impl SpapiClient {
    /// Get inventory summaries for FBA inventory
    ///
    /// Returns a list of inventory summaries. The summaries returned depend on the presence of the
    /// startDateTime and sellerSkus parameters.
    pub async fn get_inventory_summaries(
        &self,
        granularity_type: &str,
        granularity_id: &str,
        marketplace_ids: Vec<String>,
        details: Option<bool>,
        start_date_time: Option<String>,
        seller_skus: Option<Vec<String>>,
        seller_sku: Option<&str>,
        next_token: Option<&str>,
    ) -> Result<GetInventorySummariesResponse> {
        let configuration = self.create_configuration().await?;

        self.limiter()
            .wait_for_token("/fba/inventory/v1/summaries", 2.0, 2)
            .await?;

        let res = get_inventory_summaries(
            &configuration,
            granularity_type,
            granularity_id,
            marketplace_ids,
            details,
            start_date_time,
            seller_skus,
            seller_sku,
            next_token,
        )
        .await;

        self.limiter()
            .record_response("/fba/inventory/v1/summaries")
            .await?;

        Ok(res?)
        // let endpoint = ApiEndpoint {
        //     version: "fba_inventory_v1",
        //     path: "/fba/inventory/v1/summaries",
        //     path_params: None,
        //     method: ApiMethod::Get,
        //     rate: 2.0, // SP-API documentation shows 2 requests per second
        //     burst: 2,
        // };

        // let mut query_params = vec![];
        // if let Some(details) = query.details {
        //     query_params.push(("details".to_string(), details.to_string()));
        // }
        // query_params.push((
        //     "granularityType".to_string(),
        //     query.granularity_type.clone(),
        // ));
        // query_params.push(("granularityId".to_string(), query.granularity_id.clone()));
        // if let Some(start_date_time) = query.start_date_time {
        //     query_params.push(("startDateTime".to_string(), start_date_time));
        // }
        // if let Some(seller_skus) = query.seller_skus {
        //     query_params.push(("sellerSkus".to_string(), seller_skus.join(",")));
        // }
        // if let Some(seller_sku) = query.seller_sku {
        //     query_params.push(("sellerSku".to_string(), seller_sku));
        // }
        // if let Some(next_token) = query.next_token {
        //     query_params.push(("nextToken".to_string(), next_token));
        // }
        // query_params.push((
        //     "marketplaceIds".to_string(),
        //     query.marketplace_ids.join(","),
        // ));

        // let response = self
        //     .make_request(&endpoint, Some(query_params), None, None)
        //     .await?;

        // if response.status().is_success() {
        //     let result: GetInventorySummariesResponse = response.json().await?;
        //     Ok(result)
        // } else {
        //     let status = response.status();
        //     let error_text = response.text().await?;
        //     Err(anyhow::anyhow!(
        //         "Failed to get inventory summaries: {} - Response: {}",
        //         status,
        //         error_text
        //     ))
        // }
    }

    // /// Get inventory summaries for specific seller SKUs
    // ///
    // /// Convenience method to get inventory summaries for a list of seller SKUs
    // pub async fn get_inventory_summaries_by_skus(
    //     &self,
    //     marketplace_id: &str,
    //     seller_skus: &[&str],
    //     details: Option<bool>,
    // ) -> Result<GetInventorySummariesResponse> {
    //     self.get_inventory_summaries(GetInventorySummariesQuery {
    //         details,
    //         granularity_type: "Marketplace".to_string(),
    //         granularity_id: marketplace_id.to_string(),
    //         start_date_time: None,
    //         seller_skus: Some(seller_skus.iter().map(|&s| s.to_string()).collect()),
    //         seller_sku: None,
    //         next_token: None,
    //         marketplace_ids: vec![marketplace_id.to_string()],
    //     })
    //     .await
    // }

    // /// Get all inventory summaries for a marketplace
    // ///
    // /// Convenience method to get all inventory summaries for a marketplace
    // pub async fn get_all_inventory_summaries(
    //     &self,
    //     marketplace_id: &str,
    //     details: Option<bool>,
    //     start_date_time: Option<&str>,
    // ) -> Result<GetInventorySummariesResponse> {
    //     self.get_inventory_summaries(GetInventorySummariesQuery {
    //         details,
    //         granularity_type: "Marketplace".to_string(),
    //         granularity_id: marketplace_id.to_string(),
    //         start_date_time: start_date_time.map(|s| s.to_string()),
    //         seller_skus: None,
    //         seller_sku: None,
    //         next_token: None,
    //         marketplace_ids: vec![marketplace_id.to_string()],
    //     })
    //     .await
    // }

    // // Sandbox-specific operations

    // /// Add inventory items to the sandbox (sandbox-only operation)
    // ///
    // /// Requests that Amazon add items to the Sandbox Inventory with desired amount of quantity
    // /// in the sandbox environment. This is a sandbox-only operation and must be directed to a sandbox endpoint.
    // pub async fn add_inventory(
    //     &self,
    //     idempotency_token: &str,
    //     inventory_items: Vec<InventoryItem>,
    // ) -> Result<AddInventoryResponse> {
    //     let endpoint = ApiEndpoint {
    //         version: "fba_inventory_v1",
    //         path: "/fba/inventory/v1/items/inventory",
    //         path_params: None,
    //         method: HttpMethod::Post,
    //         rate: 2.0,
    //         burst: 10,
    //     };

    //     let request = AddInventoryRequest {
    //         inventory_items: Some(inventory_items),
    //     };

    //     let mut headers = vec![];
    //     headers.push(("X-Amzn-Idempotency-Token", idempotency_token.to_string()));

    //     let body = serde_json::to_string(&request)?;

    //     let response = self
    //         .make_request(&endpoint, None, Some(headers), Some(&body))
    //         .await?;

    //     if response.status().is_success() {
    //         let result: AddInventoryResponse = response.json().await?;
    //         Ok(result)
    //     } else {
    //         let status = response.status();
    //         let error_text = response.text().await?;
    //         Err(anyhow::anyhow!(
    //             "Failed to add inventory: {} - Response: {}",
    //             status,
    //             error_text
    //         ))
    //     }
    // }

    // /// Create an inventory item in the sandbox (sandbox-only operation)
    // ///
    // /// Requests that Amazon create an item in the sandbox environment
    // pub async fn create_inventory_item(
    //     &self,
    //     seller_sku: &str,
    //     marketplace_id: &str,
    //     product_name: &str,
    // ) -> Result<CreateInventoryItemResponse> {
    //     let endpoint = ApiEndpoint {
    //         version: "fba_inventory_v1",
    //         path: "/fba/inventory/v1/items/{sellerSku}",
    //         path_params: Some(vec![("sellerSku", seller_sku.to_string())]),
    //         method: HttpMethod::Put,
    //         rate: 2.0,
    //         burst: 10,
    //     };

    //     let request = CreateInventoryItemRequest {
    //         seller_sku: seller_sku.to_string(),
    //         marketplace_id: marketplace_id.to_string(),
    //         product_name: product_name.to_string(),
    //     };

    //     let body = serde_json::to_string(&request)?;

    //     let response = self
    //         .make_request(&endpoint, None, None, Some(&body))
    //         .await?;

    //     if response.status().is_success() {
    //         let result: CreateInventoryItemResponse = response.json().await?;
    //         Ok(result)
    //     } else {
    //         let status = response.status();
    //         let error_text = response.text().await?;
    //         Err(anyhow::anyhow!(
    //             "Failed to create inventory item {}: {} - Response: {}",
    //             seller_sku,
    //             status,
    //             error_text
    //         ))
    //     }
    // }

    // /// Delete an inventory item from the sandbox (sandbox-only operation)
    // ///
    // /// Requests that Amazon delete an item from the sandbox environment
    // pub async fn delete_inventory_item(
    //     &self,
    //     seller_sku: &str,
    //     marketplace_id: &str,
    // ) -> Result<DeleteInventoryItemResponse> {
    //     let endpoint = ApiEndpoint {
    //         version: "fba_inventory_v1",
    //         path: "/fba/inventory/v1/items/{sellerSku}",
    //         path_params: Some(vec![("sellerSku", seller_sku.to_string())]),
    //         method: HttpMethod::Delete,
    //         rate: 2.0,
    //         burst: 10,
    //     };

    //     let mut query_params = vec![];
    //     query_params.push(("marketplaceId".to_string(), marketplace_id.to_string()));
    //     let response = self
    //         .make_request(&endpoint, Some(query_params), None, None)
    //         .await?;

    //     if response.status().is_success() {
    //         let result: DeleteInventoryItemResponse = response.json().await?;
    //         Ok(result)
    //     } else {
    //         let status = response.status();
    //         let error_text = response.text().await?;
    //         Err(anyhow::anyhow!(
    //             "Failed to delete inventory item {}: {} - Response: {}",
    //             seller_sku,
    //             status,
    //             error_text
    //         ))
    //     }
    // }
}
