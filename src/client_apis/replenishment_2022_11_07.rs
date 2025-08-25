use crate::{client::SpapiClient, models};
use anyhow::Result;

impl SpapiClient {
    /// Returns aggregated replenishment program metrics for a selling partner.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_selling_partner_metrics(
        &self,
        body: Option<models::replenishment_2022_11_07::GetSellingPartnerMetricsRequest>,
    ) -> Result<models::replenishment_2022_11_07::GetSellingPartnerMetricsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "/replenishment/2022-11-07/sellingPartners/metrics/search",
                1.0,
                1,
            )
            .await?;
        let res = crate::apis::replenishment_2022_11_07::get_selling_partner_metrics(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns aggregated replenishment program metrics for a selling partner's offers.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn list_offer_metrics(
        &self,
        body: Option<models::replenishment_2022_11_07::ListOfferMetricsRequest>,
    ) -> Result<models::replenishment_2022_11_07::ListOfferMetricsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/replenishment/2022-11-07/offers/metrics/search", 1.0, 1)
            .await?;
        let res =
            crate::apis::replenishment_2022_11_07::list_offer_metrics(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the details of a selling partner's replenishment program offers.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn list_offers(
        &self,
        body: Option<models::replenishment_2022_11_07::ListOffersRequest>,
    ) -> Result<models::replenishment_2022_11_07::ListOffersResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/replenishment/2022-11-07/offers/search", 1.0, 1)
            .await?;
        let res = crate::apis::replenishment_2022_11_07::list_offers(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }
}
