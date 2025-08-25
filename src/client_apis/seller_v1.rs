use crate::{
    client::SpapiClient,
    models::sellers::{GetAccountResponse, GetMarketplaceParticipationsResponse},
};
use anyhow::Result;

impl SpapiClient {
    /// Returns information about a seller account and its marketplaces.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.016 | 15 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_account(&self) -> Result<GetAccountResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/sellers/v1/account", 0.016, 15)
            .await?;
        let res = crate::apis::sellers_v1::get_account(&configuration).await?;
        guard.mark_response().await;
        Ok(res)
    }
    
    /// Returns a list of marketplaces where the seller can list items and information about the seller's participation in those marketplaces.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.016 | 15 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_marketplace_participations(
        &self,
    ) -> Result<GetMarketplaceParticipationsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/sellers/v1/marketplaceParticipations", 0.016, 15)
            .await?;
        let res = crate::apis::sellers_v1::get_marketplace_participations(&configuration).await?;
        guard.mark_response().await;
        Ok(res)
    }
}
