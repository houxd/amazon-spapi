use crate::{client::SpapiClient, models};
use anyhow::Result;

impl SpapiClient {
    /// Returns the parent categories to which an item belongs, based on the specified ASIN or SellerSKU.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 2 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn list_catalog_categories(
        &self,
        marketplace_id: &str,
        asin: Option<&str>,
        seller_sku: Option<&str>,
    ) -> Result<models::catalog_items_v0::ListCatalogCategoriesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/catalog/v0/listCatalogCategories", 1.0, 2)
            .await?;
        let res = crate::apis::catalog_items_v0::list_catalog_categories(
            &configuration,
            marketplace_id,
            asin,
            seller_sku,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
