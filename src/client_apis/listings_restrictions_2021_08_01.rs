use crate::{client::SpapiClient, models};
use anyhow::Result;

impl SpapiClient {
    /// Returns listing restrictions for an item in the Amazon Catalog.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_listings_restrictions(
        &self,
        asin: &str,
        seller_id: &str,
        marketplace_ids: Vec<String>,
        condition_type: Option<&str>,
        reason_locale: Option<&str>,
    ) -> Result<models::listings_restrictions_2021_08_01::RestrictionList> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/listings/2021-08-01/restrictions", 5.0, 10)
            .await?;

        let res = crate::apis::listings_restrictions_2021_08_01::get_listings_restrictions(
            &configuration,
            asin,
            seller_id,
            marketplace_ids,
            condition_type,
            reason_locale,
        )
        .await?;

        guard.mark_response().await;
        Ok(res)
    }
}
