use anyhow::Result;

use crate::{
    apis::fba_inventory_v1::get_inventory_summaries,
    client::SpapiClient,
    models::{
        self,
        fba_inventory::{GetInventorySummariesResponse, InventoryItem},
    },
};

impl SpapiClient {
    /// Requests that Amazon add items to the Sandbox Inventory with desired amount of quantity in the sandbox environment. This is a sandbox-only operation and must be directed to a sandbox endpoint. Refer to [Selling Partner API sandbox](https://developer-docs.amazon.com/sp-api/docs/the-selling-partner-api-sandbox) for more information.
    pub async fn add_inventory(
        &self,
        x_amzn_idempotency_token: &str,
        add_inventory_request_body: models::fba_inventory::AddInventoryRequest,
    ) -> Result<models::fba_inventory::AddInventoryResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/fba/inventory/v1/addInventory", 2.0, 2)
            .await?;
        let res = crate::apis::fba_inventory_v1::add_inventory(
            &configuration,
            x_amzn_idempotency_token,
            add_inventory_request_body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Requests that Amazon create product-details in the Sandbox Inventory in the sandbox environment. This is a sandbox-only operation and must be directed to a sandbox endpoint. Refer to [Selling Partner API sandbox](https://developer-docs.amazon.com/sp-api/docs/the-selling-partner-api-sandbox) for more information.
    pub async fn create_inventory_item(
        &self,
        create_inventory_item_request_body: models::fba_inventory::CreateInventoryItemRequest,
    ) -> Result<models::fba_inventory::CreateInventoryItemResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/fba/inventory/v1/createInventoryItem", 2.0, 2)
            .await?;
        let res = crate::apis::fba_inventory_v1::create_inventory_item(
            &configuration,
            create_inventory_item_request_body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Requests that Amazon Deletes an item from the Sandbox Inventory in the sandbox environment. This is a sandbox-only operation and must be directed to a sandbox endpoint. Refer to [Selling Partner API sandbox](https://developer-docs.amazon.com/sp-api/docs/the-selling-partner-api-sandbox) for more information.
    pub async fn delete_inventory_item(
        &self,
        seller_sku: &str,
        marketplace_id: &str,
    ) -> Result<models::fba_inventory::DeleteInventoryItemResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/fba/inventory/v1/deleteInventoryItem", 2.0, 2)
            .await?;
        let res = crate::apis::fba_inventory_v1::delete_inventory_item(
            &configuration,
            seller_sku,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of inventory summaries. The summaries returned depend on the presence or absence of the startDateTime, sellerSkus and sellerSku parameters:  - All inventory summaries with available details are returned when the startDateTime, sellerSkus and sellerSku parameters are omitted. - When startDateTime is provided, the operation returns inventory summaries that have had changes after the date and time specified. The sellerSkus and sellerSku parameters are ignored. Important: To avoid errors, use both startDateTime and nextToken to get the next page of inventory summaries that have changed after the date and time specified. - When the sellerSkus parameter is provided, the operation returns inventory summaries for only the specified sellerSkus. The sellerSku parameter is ignored. - When the sellerSku parameter is provided, the operation returns inventory summaries for only the specified sellerSku.  Note: The parameters associated with this operation may contain special characters that must be encoded to successfully call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  Usage Plan:  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits).
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
        let guard = self
            .limiter()
            .wait("/fba/inventory/v1/summaries", 2.0, 2)
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
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
