use crate::{client::SpapiClient, models};
use anyhow::Result;

impl SpapiClient {
    /// Delete a listings item for a selling partner.  **Note:** The parameters associated with this operation may contain special characters that must be encoded to successfully call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn delete_listings_item(
        &self,
        seller_id: &str,
        sku: &str,
        marketplace_ids: Vec<String>,
        issue_locale: Option<&str>,
    ) -> Result<models::listings_items_2020_09_01::ListingsItemSubmissionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("listings_items_2020_09_01/delete_listings_item", 5.0, 10)
            .await?;
        let res = crate::apis::listings_items_2020_09_01::delete_listings_item(
            &configuration,
            seller_id,
            sku,
            marketplace_ids,
            issue_locale,
        )
        .await;
        guard.mark_response().await;
        Ok(res?)
    }

    /// Partially update (patch) a listings item for a selling partner. Only top-level listings item attributes can be patched. Patching nested attributes is not supported.  **Note:** The parameters associated with this operation may contain special characters that must be encoded to successfully call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn patch_listings_item(
        &self,
        seller_id: &str,
        sku: &str,
        marketplace_ids: Vec<String>,
        body: models::listings_items_2020_09_01::ListingsItemPatchRequest,
        issue_locale: Option<&str>,
    ) -> Result<models::listings_items_2020_09_01::ListingsItemSubmissionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("listings_items_2020_09_01/patch_listings_item", 5.0, 10)
            .await?;
        let res = crate::apis::listings_items_2020_09_01::patch_listings_item(
            &configuration,
            seller_id,
            sku,
            marketplace_ids,
            body,
            issue_locale,
        )
        .await;
        guard.mark_response().await;
        Ok(res?)
    }

    /// Creates a new or fully-updates an existing listings item for a selling partner.  **Note:** The parameters associated with this operation may contain special characters that must be encoded to successfully call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn put_listings_item(
        &self,
        seller_id: &str,
        sku: &str,
        marketplace_ids: Vec<String>,
        body: models::listings_items_2020_09_01::ListingsItemPutRequest,
        issue_locale: Option<&str>,
    ) -> Result<models::listings_items_2020_09_01::ListingsItemSubmissionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("listings_items_2020_09_01/put_listings_item", 5.0, 10)
            .await?;
        let res = crate::apis::listings_items_2020_09_01::put_listings_item(
            &configuration,
            seller_id,
            sku,
            marketplace_ids,
            body,
            issue_locale,
        )
        .await;
        guard.mark_response().await;
        Ok(res?)
    }
}
