use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Sends a message asking a buyer to provide or verify customization details such as name spelling, images, initials, etc.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn confirm_customization_details(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
        body: models::messaging::CreateConfirmCustomizationDetailsRequest,
    ) -> Result<models::messaging::CreateConfirmCustomizationDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/confirm_customization_details", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::confirm_customization_details(
            &configuration,
            amazon_order_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sends a message to a buyer to provide details about an Amazon Motors order. This message can only be sent by Amazon Motors sellers.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_amazon_motors(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
        body: models::messaging::CreateAmazonMotorsRequest,
    ) -> Result<models::messaging::CreateAmazonMotorsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/create_amazon_motors", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::create_amazon_motors(
            &configuration,
            amazon_order_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sends a message to a buyer to arrange a delivery or to confirm contact information for making a delivery.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_confirm_delivery_details(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
        body: models::messaging::CreateConfirmDeliveryDetailsRequest,
    ) -> Result<models::messaging::CreateConfirmDeliveryDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/create_confirm_delivery_details", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::create_confirm_delivery_details(
            &configuration,
            amazon_order_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sends a message to ask a buyer an order-related question prior to shipping their order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_confirm_order_details(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
        body: models::messaging::CreateConfirmOrderDetailsRequest,
    ) -> Result<models::messaging::CreateConfirmOrderDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/create_confirm_order_details", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::create_confirm_order_details(
            &configuration,
            amazon_order_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sends a message to contact a Home Service customer to arrange a service call or to gather information prior to a service call.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_confirm_service_details(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
        body: models::messaging::CreateConfirmServiceDetailsRequest,
    ) -> Result<models::messaging::CreateConfirmServiceDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/create_confirm_service_details", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::create_confirm_service_details(
            &configuration,
            amazon_order_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sends a buyer a message to share a digital access key that is required to utilize digital content in their order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_digital_access_key(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
        body: models::messaging::CreateDigitalAccessKeyRequest,
    ) -> Result<models::messaging::CreateDigitalAccessKeyResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/create_digital_access_key", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::create_digital_access_key(
            &configuration,
            amazon_order_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sends a critical message that contains documents that a seller is legally obligated to provide to the buyer. This message should only be used to deliver documents that are required by law.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_legal_disclosure(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
        body: models::messaging::CreateLegalDisclosureRequest,
    ) -> Result<models::messaging::CreateLegalDisclosureResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/create_legal_disclosure", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::create_legal_disclosure(
            &configuration,
            amazon_order_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sends a non-critical message that asks a buyer to remove their negative feedback. This message should only be sent after the seller has resolved the buyer's problem.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_negative_feedback_removal(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
    ) -> Result<models::messaging::CreateNegativeFeedbackRemovalResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/create_negative_feedback_removal", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::create_negative_feedback_removal(
            &configuration,
            amazon_order_id,
            marketplace_ids,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sends a critical message to a buyer that an unexpected problem was encountered affecting the completion of the order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_unexpected_problem(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
        body: models::messaging::CreateUnexpectedProblemRequest,
    ) -> Result<models::messaging::CreateUnexpectedProblemResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/create_unexpected_problem", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::create_unexpected_problem(
            &configuration,
            amazon_order_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sends a message to a buyer to provide details about warranty information on a purchase in their order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_warranty(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
        body: models::messaging::CreateWarrantyRequest,
    ) -> Result<models::messaging::CreateWarrantyResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/create_warranty", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::create_warranty(
            &configuration,
            amazon_order_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a response containing attributes related to an order. This includes buyer preferences.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_attributes(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
    ) -> Result<models::messaging::GetAttributesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/get_attributes", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::get_attributes(
            &configuration,
            amazon_order_id,
            marketplace_ids,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of message types that are available for an order that you specify. A message type is represented by an actions object, which contains a path and query parameter(s).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_messaging_actions_for_order(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
    ) -> Result<models::messaging::GetMessagingActionsForOrderResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/get_messaging_actions_for_order", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::get_messaging_actions_for_order(
            &configuration,
            amazon_order_id,
            marketplace_ids,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sends a message providing the buyer an invoice.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn send_invoice(
        &self,
        amazon_order_id: &str,
        marketplace_ids: Vec<String>,
        body: models::messaging::InvoiceRequest,
    ) -> Result<models::messaging::InvoiceResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("messaging_v1/send_invoice", 1.0, 5)
            .await?;
        let res = crate::apis::messaging_v1::send_invoice(
            &configuration,
            amazon_order_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
