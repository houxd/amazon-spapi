use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Updates the shipment confirmation status for a specified order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 10 |
    pub async fn confirm_shipment(
        &self,
        order_id: &str,
        payload: models::orders_v0::ConfirmShipmentRequest,
    ) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("orders_v0/confirm_shipment", 2.0, 10)
            .await?;
        let res =
            crate::apis::orders_v0_api::confirm_shipment(&configuration, order_id, payload).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the order that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |
    pub async fn get_order(&self, order_id: &str) -> Result<models::orders_v0::GetOrderResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self.limiter().wait("orders_v0/get_order", 0.5, 30).await?;
        let res = crate::apis::orders_v0_api::get_order(&configuration, order_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the shipping address for the order that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |
    pub async fn get_order_address(
        &self,
        order_id: &str,
    ) -> Result<models::orders_v0::GetOrderAddressResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("orders_v0/get_order_address", 0.5, 30)
            .await?;
        let res = crate::apis::orders_v0_api::get_order_address(&configuration, order_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns buyer information for the order that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |
    pub async fn get_order_buyer_info(
        &self,
        order_id: &str,
    ) -> Result<models::orders_v0::GetOrderBuyerInfoResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("orders_v0/get_order_buyer_info", 0.5, 30)
            .await?;
        let res =
            crate::apis::orders_v0_api::get_order_buyer_info(&configuration, order_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns detailed order item information for the order that you specify. If `NextToken` is provided, it's used to retrieve the next page of order items.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |
    pub async fn get_order_items(
        &self,
        order_id: &str,
        next_token: Option<&str>,
    ) -> Result<models::orders_v0::GetOrderItemsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("orders_v0/get_order_items", 0.5, 30)
            .await?;
        let res = crate::apis::orders_v0_api::get_order_items(&configuration, order_id, next_token)
            .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns buyer information for the order items in the order that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |
    pub async fn get_order_items_buyer_info(
        &self,
        order_id: &str,
        next_token: Option<&str>,
    ) -> Result<models::orders_v0::GetOrderItemsBuyerInfoResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("orders_v0/get_order_items_buyer_info", 0.5, 30)
            .await?;
        let res = crate::apis::orders_v0_api::get_order_items_buyer_info(
            &configuration,
            order_id,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns regulated information for the order that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |
    pub async fn get_order_regulated_info(
        &self,
        order_id: &str,
    ) -> Result<models::orders_v0::GetOrderRegulatedInfoResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("orders_v0/get_order_regulated_info", 0.5, 30)
            .await?;
        let res =
            crate::apis::orders_v0_api::get_order_regulated_info(&configuration, order_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns orders that are created or updated during the specified time period. If you want to return specific types of orders, you can apply filters to your request.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0167 | 20 |
    pub async fn get_orders(
        &self,
        marketplace_ids: Vec<String>,
        created_after: Option<&str>,
        created_before: Option<&str>,
        last_updated_after: Option<&str>,
        last_updated_before: Option<&str>,
        order_statuses: Option<Vec<String>>,
        fulfillment_channels: Option<Vec<String>>,
        payment_methods: Option<Vec<String>>,
        buyer_email: Option<&str>,
        seller_order_id: Option<&str>,
        max_results_per_page: Option<i32>,
        easy_ship_shipment_statuses: Option<Vec<String>>,
        electronic_invoice_statuses: Option<Vec<String>>,
        next_token: Option<&str>,
        amazon_order_ids: Option<Vec<String>>,
        actual_fulfillment_supply_source_id: Option<&str>,
        is_ispu: Option<bool>,
        store_chain_store_id: Option<&str>,
        earliest_delivery_date_before: Option<&str>,
        earliest_delivery_date_after: Option<&str>,
        latest_delivery_date_before: Option<&str>,
        latest_delivery_date_after: Option<&str>,
    ) -> Result<models::orders_v0::GetOrdersResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("orders_v0/get_orders", 0.0167, 20)
            .await?;
        let res = crate::apis::orders_v0_api::get_orders(
            &configuration,
            marketplace_ids,
            created_after,
            created_before,
            last_updated_after,
            last_updated_before,
            order_statuses,
            fulfillment_channels,
            payment_methods,
            buyer_email,
            seller_order_id,
            max_results_per_page,
            easy_ship_shipment_statuses,
            electronic_invoice_statuses,
            next_token,
            amazon_order_ids,
            actual_fulfillment_supply_source_id,
            is_ispu,
            store_chain_store_id,
            earliest_delivery_date_before,
            earliest_delivery_date_after,
            latest_delivery_date_before,
            latest_delivery_date_after,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates (approves or rejects) the verification status of an order containing regulated products.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |
    pub async fn update_verification_status(
        &self,
        order_id: &str,
        payload: models::orders_v0::UpdateVerificationStatusRequest,
    ) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("orders_v0/update_verification_status", 0.5, 30)
            .await?;
        let res = crate::apis::orders_v0_api::update_verification_status(
            &configuration,
            order_id,
            payload,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
