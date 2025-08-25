use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Cancels an AWD Inbound order and its associated shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn cancel_inbound(&self, order_id: &str) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("awd_2024_05_09/cancel_inbound", 1.0, 1)
            .await?;
        let res = crate::apis::awd_api::cancel_inbound(&configuration, order_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Determines if the packages you specify are eligible for an AWD inbound order and contains error details for ineligible packages.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn check_inbound_eligibility(
        &self,
        body: models::awd_2024_05_09::InboundPackages,
    ) -> Result<models::awd_2024_05_09::InboundEligibility> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("awd_2024_05_09/check_inbound_eligibility", 1.0, 1)
            .await?;
        let res = crate::apis::awd_api::check_inbound_eligibility(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Confirms an AWD inbound order in `DRAFT` status.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn confirm_inbound(&self, order_id: &str) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("awd_2024_05_09/confirm_inbound", 1.0, 1)
            .await?;
        let res = crate::apis::awd_api::confirm_inbound(&configuration, order_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Creates a draft AWD inbound order with a list of packages for inbound shipment. The operation creates one shipment per order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn create_inbound(
        &self,
        body: models::awd_2024_05_09::InboundOrderCreationData,
    ) -> Result<models::awd_2024_05_09::InboundOrderReference> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("awd_2024_05_09/create_inbound", 1.0, 1)
            .await?;
        let res = crate::apis::awd_api::create_inbound(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves an AWD inbound order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn get_inbound(
        &self,
        order_id: &str,
    ) -> Result<models::awd_2024_05_09::InboundOrder> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("awd_2024_05_09/get_inbound", 2.0, 2)
            .await?;
        let res = crate::apis::awd_api::get_inbound(&configuration, order_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves an AWD inbound shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn get_inbound_shipment(
        &self,
        shipment_id: &str,
        sku_quantities: Option<&str>,
    ) -> Result<models::awd_2024_05_09::InboundShipment> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("awd_2024_05_09/get_inbound_shipment", 2.0, 2)
            .await?;
        let res =
            crate::apis::awd_api::get_inbound_shipment(&configuration, shipment_id, sku_quantities)
                .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves the box labels for a shipment ID that you specify. This is an asynchronous operation. If the label status is `GENERATED`, then the label URL is available.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 2 |
    pub async fn get_inbound_shipment_labels(
        &self,
        shipment_id: &str,
        page_type: Option<&str>,
        format_type: Option<&str>,
    ) -> Result<models::awd_2024_05_09::ShipmentLabels> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("awd_2024_05_09/get_inbound_shipment_labels", 1.0, 2)
            .await?;
        let res = crate::apis::awd_api::get_inbound_shipment_labels(
            &configuration,
            shipment_id,
            page_type,
            format_type,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves a summary of all the inbound AWD shipments associated with a merchant, with the ability to apply optional filters.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn list_inbound_shipments(
        &self,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
        shipment_status: Option<&str>,
        updated_after: Option<String>,
        updated_before: Option<String>,
        max_results: Option<i32>,
        next_token: Option<&str>,
    ) -> Result<models::awd_2024_05_09::ShipmentListing> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("awd_2024_05_09/list_inbound_shipments", 1.0, 1)
            .await?;
        let res = crate::apis::awd_api::list_inbound_shipments(
            &configuration,
            sort_by,
            sort_order,
            shipment_status,
            updated_after,
            updated_before,
            max_results,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Lists AWD inventory associated with a merchant with the ability to apply optional filters.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn list_inventory(
        &self,
        sku: Option<&str>,
        sort_order: Option<&str>,
        details: Option<&str>,
        next_token: Option<&str>,
        max_results: Option<i32>,
    ) -> Result<models::awd_2024_05_09::InventoryListing> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("awd_2024_05_09/list_inventory", 2.0, 2)
            .await?;
        let res = crate::apis::awd_api::list_inventory(
            &configuration,
            sku,
            sort_order,
            details,
            next_token,
            max_results,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates an AWD inbound order that is in `DRAFT` status and not yet confirmed. Use this operation to update the `packagesToInbound`, `originAddress` and `preferences` attributes.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn update_inbound(
        &self,
        order_id: &str,
        body: models::awd_2024_05_09::InboundOrder,
    ) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("awd_2024_05_09/update_inbound", 1.0, 1)
            .await?;
        let res = crate::apis::awd_api::update_inbound(&configuration, order_id, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates transport details for an AWD shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn update_inbound_shipment_transport_details(
        &self,
        shipment_id: &str,
        body: models::awd_2024_05_09::TransportationDetails,
    ) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "awd_2024_05_09/update_inbound_shipment_transport_details",
                1.0,
                1,
            )
            .await?;
        let res = crate::apis::awd_api::update_inbound_shipment_transport_details(
            &configuration,
            shipment_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
