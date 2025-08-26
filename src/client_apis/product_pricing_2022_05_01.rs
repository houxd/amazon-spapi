use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Returns the competitive summary response, including featured buying options for the ASIN and `marketplaceId` combination.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.033 | 1 |
    pub async fn get_competitive_summary(
        &self,
        requests: models::product_pricing_2022_05_01::CompetitiveSummaryBatchRequest,
    ) -> Result<models::product_pricing_2022_05_01::CompetitiveSummaryBatchResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("product_pricing_2022_05_01/get_competitive_summary", 0.033, 1)
            .await?;
        let res = crate::apis::product_pricing_2022_05_01::get_competitive_summary(
            &configuration,
            requests,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the set of responses that correspond to the batched list of up to 40 requests defined in the request body. The response for each successful (HTTP status code 200) request in the set includes the computed listing price at or below which a seller can expect to become the featured offer (before applicable promotions). This is called the featured offer expected price (FOEP). Featured offer is not guaranteed because competing offers might change. Other offers might be featured based on factors such as fulfillment capabilities to a specific customer. The response to an unsuccessful request includes the available error text.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.033 | 1 |
    pub async fn get_featured_offer_expected_price_batch(
        &self,
        get_featured_offer_expected_price_batch_request_body: models::product_pricing_2022_05_01::GetFeaturedOfferExpectedPriceBatchRequest,
    ) -> Result<models::product_pricing_2022_05_01::GetFeaturedOfferExpectedPriceBatchResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("product_pricing_2022_05_01/get_featured_offer_expected_price_batch", 0.033, 1)
            .await?;
        let res = crate::apis::product_pricing_2022_05_01::get_featured_offer_expected_price_batch(
            &configuration,
            get_featured_offer_expected_price_batch_request_body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
