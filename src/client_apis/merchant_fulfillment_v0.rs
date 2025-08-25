use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Cancel the shipment indicated by the specified shipment identifier.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn cancel_shipment(
        &self,
        shipment_id: &str,
    ) -> Result<models::merchant_fulfillment_v0::CancelShipmentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("merchant_fulfillment_v0/cancel_shipment", 1.0, 1)
            .await?;
        let res =
            crate::apis::merchant_fulfillment_v0::cancel_shipment(&configuration, shipment_id)
                .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Create a shipment with the information provided.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn create_shipment(
        &self,
        body: models::merchant_fulfillment_v0::CreateShipmentRequest,
    ) -> Result<models::merchant_fulfillment_v0::CreateShipmentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("merchant_fulfillment_v0/create_shipment", 2.0, 2)
            .await?;
        let res =
            crate::apis::merchant_fulfillment_v0::create_shipment(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Gets a list of additional seller inputs required for a ship method. This is generally used for international shipping.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn get_additional_seller_inputs(
        &self,
        body: models::merchant_fulfillment_v0::GetAdditionalSellerInputsRequest,
    ) -> Result<models::merchant_fulfillment_v0::GetAdditionalSellerInputsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "merchant_fulfillment_v0/get_additional_seller_inputs",
                1.0,
                1,
            )
            .await?;
        let res = crate::apis::merchant_fulfillment_v0::get_additional_seller_inputs(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of shipping service offers that satisfy the specified shipment request details.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 6 | 12 |
    pub async fn get_eligible_shipment_services(
        &self,
        body: models::merchant_fulfillment_v0::GetEligibleShipmentServicesRequest,
    ) -> Result<models::merchant_fulfillment_v0::GetEligibleShipmentServicesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "merchant_fulfillment_v0/get_eligible_shipment_services",
                6.0,
                12,
            )
            .await?;
        let res = crate::apis::merchant_fulfillment_v0::get_eligible_shipment_services(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the shipment information for an existing shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn get_mfn_shipment(
        &self,
        shipment_id: &str,
    ) -> Result<models::merchant_fulfillment_v0::GetShipmentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("merchant_fulfillment_v0/get_shipment", 1.0, 1)
            .await?;
        let res = crate::apis::merchant_fulfillment_v0::get_shipment(&configuration, shipment_id)
            .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
