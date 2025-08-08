use anyhow::Result;

use crate::{
    apis::fba_inventory_api::get_inventory_summaries,
    client::{ApiEndpoint, ApiMethod, SpapiClient},
    models::{
        self,
        fba_inventory::{GetInventorySummariesResponse, InventoryItem},
    },
};

impl SpapiClient {
    pub async fn add_inventory(
        &self,
        x_amzn_idempotency_token: &str,
        add_inventory_request_body: models::fba_inventory::AddInventoryRequest,
    ) -> Result<models::fba_inventory::AddInventoryResponse> {
        let configuration = self.create_configuration().await?;
        let _ = self
            .limiter()
            .wait("/fba/inventory/v1/addInventory", 2.0, 2)
            .await?;
        let res = crate::apis::fba_inventory_api::add_inventory(
            &configuration,
            x_amzn_idempotency_token,
            add_inventory_request_body,
        )
        .await?;
        Ok(res)
    }

    pub async fn create_inventory_item(
        &self,
        create_inventory_item_request_body: models::fba_inventory::CreateInventoryItemRequest,
    ) -> Result<models::fba_inventory::CreateInventoryItemResponse> {
        let configuration = self.create_configuration().await?;
        let _ = self
            .limiter()
            .wait("/fba/inventory/v1/createInventoryItem", 2.0, 2)
            .await?;
        let res = crate::apis::fba_inventory_api::create_inventory_item(
            &configuration,
            create_inventory_item_request_body,
        )
        .await?;
        Ok(res)
    }

    pub async fn delete_inventory_item(
        &self,
        seller_sku: &str,
        marketplace_id: &str,
    ) -> Result<models::fba_inventory::DeleteInventoryItemResponse> {
        let configuration = self.create_configuration().await?;
        let _ = self
            .limiter()
            .wait("/fba/inventory/v1/deleteInventoryItem", 2.0, 2)
            .await?;
        let res = crate::apis::fba_inventory_api::delete_inventory_item(
            &configuration,
            seller_sku,
            marketplace_id,
        )
        .await?;
        Ok(res)
    }

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
        let _ = self
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

        Ok(res)
    }
}
