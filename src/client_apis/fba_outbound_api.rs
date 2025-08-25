use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Requests that Amazon stop attempting to fulfill the fulfillment order indicated by the specified order identifier.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn cancel_fulfillment_order(
        &self,
        seller_fulfillment_order_id: &str,
    ) -> Result<models::fulfillment_outbound_2020_07_01::CancelFulfillmentOrderResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/cancel_fulfillment_order",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fba_outbound_api::cancel_fulfillment_order(
            &configuration,
            seller_fulfillment_order_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Requests that Amazon ship items from the seller's inventory in Amazon's fulfillment network to a destination address.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn create_fulfillment_order(
        &self,
        body: models::fulfillment_outbound_2020_07_01::CreateFulfillmentOrderRequest,
    ) -> Result<models::fulfillment_outbound_2020_07_01::CreateFulfillmentOrderResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/create_fulfillment_order",
                2.0,
                30,
            )
            .await?;
        let res =
            crate::apis::fba_outbound_api::create_fulfillment_order(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Creates a fulfillment return.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn create_fulfillment_return(
        &self,
        seller_fulfillment_order_id: &str,
        body: models::fulfillment_outbound_2020_07_01::CreateFulfillmentReturnRequest,
    ) -> Result<models::fulfillment_outbound_2020_07_01::CreateFulfillmentReturnResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/create_fulfillment_return",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fba_outbound_api::create_fulfillment_return(
            &configuration,
            seller_fulfillment_order_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns delivery options that include an estimated delivery date and offer expiration, based on criteria that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 30 |
    pub async fn delivery_offers(
        &self,
        body: models::fulfillment_outbound_2020_07_01::GetDeliveryOffersRequest,
    ) -> Result<models::fulfillment_outbound_2020_07_01::GetDeliveryOffersResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_outbound_2020_07_01/delivery_offers", 5.0, 30)
            .await?;
        let res = crate::apis::fba_outbound_api::delivery_offers(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of inventory items that are eligible for the fulfillment feature you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_feature_inventory(
        &self,
        marketplace_id: &str,
        feature_name: &str,
        next_token: Option<&str>,
        query_start_date: Option<String>,
    ) -> Result<models::fulfillment_outbound_2020_07_01::GetFeatureInventoryResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/get_feature_inventory",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fba_outbound_api::get_feature_inventory(
            &configuration,
            marketplace_id,
            feature_name,
            next_token,
            query_start_date,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the number of items with the sellerSKU you specify that can have orders fulfilled using the specified feature.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_feature_sku(
        &self,
        marketplace_id: &str,
        feature_name: &str,
        seller_sku: &str,
    ) -> Result<models::fulfillment_outbound_2020_07_01::GetFeatureSkuResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_outbound_2020_07_01/get_feature_sku", 2.0, 30)
            .await?;
        let res = crate::apis::fba_outbound_api::get_feature_sku(
            &configuration,
            marketplace_id,
            feature_name,
            seller_sku,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of features available for Multi-Channel Fulfillment orders in the marketplace you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_features(
        &self,
        marketplace_id: &str,
    ) -> Result<models::fulfillment_outbound_2020_07_01::GetFeaturesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_outbound_2020_07_01/get_features", 2.0, 30)
            .await?;
        let res =
            crate::apis::fba_outbound_api::get_features(&configuration, marketplace_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the fulfillment order indicated by the specified order identifier.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_fulfillment_order(
        &self,
        seller_fulfillment_order_id: &str,
    ) -> Result<models::fulfillment_outbound_2020_07_01::GetFulfillmentOrderResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/get_fulfillment_order",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fba_outbound_api::get_fulfillment_order(
            &configuration,
            seller_fulfillment_order_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of fulfillment order previews based on shipping criteria that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_fulfillment_preview(
        &self,
        body: models::fulfillment_outbound_2020_07_01::GetFulfillmentPreviewRequest,
    ) -> Result<models::fulfillment_outbound_2020_07_01::GetFulfillmentPreviewResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/get_fulfillment_preview",
                2.0,
                30,
            )
            .await?;
        let res =
            crate::apis::fba_outbound_api::get_fulfillment_preview(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns delivery tracking information for a package in an outbound shipment for a Multi-Channel Fulfillment order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_package_tracking_details(
        &self,
        package_number: i32,
    ) -> Result<models::fulfillment_outbound_2020_07_01::GetPackageTrackingDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/get_package_tracking_details",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fba_outbound_api::get_package_tracking_details(
            &configuration,
            package_number,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of fulfillment orders fulfilled after (or at) a specified date-time.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn list_all_fulfillment_orders(
        &self,
        query_start_date: Option<String>,
        next_token: Option<&str>,
    ) -> Result<models::fulfillment_outbound_2020_07_01::ListAllFulfillmentOrdersResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/list_all_fulfillment_orders",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fba_outbound_api::list_all_fulfillment_orders(
            &configuration,
            query_start_date,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of return reason codes for a seller SKU in a given marketplace.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn list_return_reason_codes(
        &self,
        seller_sku: &str,
        marketplace_id: Option<&str>,
        seller_fulfillment_order_id: Option<&str>,
        language: Option<&str>,
    ) -> Result<models::fulfillment_outbound_2020_07_01::ListReturnReasonCodesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/list_return_reason_codes",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fba_outbound_api::list_return_reason_codes(
            &configuration,
            seller_sku,
            marketplace_id,
            seller_fulfillment_order_id,
            language,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Requests that Amazon update the status of an order in the sandbox testing environment. This is a sandbox-only operation.
    pub async fn submit_fulfillment_order_status_update(
        &self,
        seller_fulfillment_order_id: &str,
        body: models::fulfillment_outbound_2020_07_01::SubmitFulfillmentOrderStatusUpdateRequest,
    ) -> Result<models::fulfillment_outbound_2020_07_01::SubmitFulfillmentOrderStatusUpdateResponse>
    {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/submit_fulfillment_order_status_update",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fba_outbound_api::submit_fulfillment_order_status_update(
            &configuration,
            seller_fulfillment_order_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates and/or requests shipment for a fulfillment order with an order hold on it.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn update_fulfillment_order(
        &self,
        seller_fulfillment_order_id: &str,
        body: models::fulfillment_outbound_2020_07_01::UpdateFulfillmentOrderRequest,
    ) -> Result<models::fulfillment_outbound_2020_07_01::UpdateFulfillmentOrderResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_outbound_2020_07_01/update_fulfillment_order",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fba_outbound_api::update_fulfillment_order(
            &configuration,
            seller_fulfillment_order_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
