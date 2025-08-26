use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Returns the invoice status for the shipment you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1.133 | 25 |
    pub async fn get_invoice_status(
        &self,
        shipment_id: &str,
    ) -> Result<models::shipment_invoicing_v0::GetInvoiceStatusResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("shipment_invoicing_v0/get_invoice_status", 1.133, 25)
            .await?;
        let res = crate::apis::shipment_invoicing_v0::get_invoice_status(
            &configuration,
            shipment_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the shipment details required to issue an invoice for the specified shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1.133 | 25 |
    pub async fn get_shipment_details(
        &self,
        shipment_id: &str,
    ) -> Result<models::shipment_invoicing_v0::GetShipmentDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("shipment_invoicing_v0/get_shipment_details", 1.133, 25)
            .await?;
        let res = crate::apis::shipment_invoicing_v0::get_shipment_details(
            &configuration,
            shipment_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Submits a shipment invoice document for a given shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1.133 | 25 |
    pub async fn submit_invoice(
        &self,
        shipment_id: &str,
        body: models::shipment_invoicing_v0::SubmitInvoiceRequest,
    ) -> Result<models::shipment_invoicing_v0::SubmitInvoiceResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("shipment_invoicing_v0/submit_invoice", 1.133, 25)
            .await?;
        let res = crate::apis::shipment_invoicing_v0::submit_invoice(
            &configuration,
            shipment_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
