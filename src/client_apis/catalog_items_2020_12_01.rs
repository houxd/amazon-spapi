use crate::client::SpapiClient;
use crate::models;
use anyhow::Result;
use std::collections::HashSet;
use std::vec;

impl SpapiClient {
    /// Retrieves details for an item in the Amazon catalog.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_catalog_item(
        &self,
        asin: &str,
        marketplace_ids: Vec<String>,
        included_data: Option<Vec<String>>,
        locale: Option<&str>,
    ) -> Result<models::catalog_items_2020_12_01::Item> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/catalog/2020-12-01/items", 2.0, 2)
            .await?;
        let res = crate::apis::catalog_items_2020_12_01::get_catalog_item(
            &configuration,
            asin,
            marketplace_ids,
            included_data,
            locale,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
    
    /// Search for and return a list of Amazon catalog items and associated information.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn search_catalog_items(
        &self,
        keywords: Vec<String>,
        marketplace_ids: Vec<String>,
        included_data: Option<Vec<String>>,
        brand_names: Option<Vec<String>>,
        classification_ids: Option<Vec<String>>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        keywords_locale: Option<&str>,
        locale: Option<&str>,
    ) -> Result<models::catalog_items_2020_12_01::ItemSearchResults> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/catalog/2020-12-01/items/{asin}", 2.0, 2)
            .await?;
        let res = crate::apis::catalog_items_2020_12_01::search_catalog_items(
            &configuration,
            keywords,
            marketplace_ids,
            included_data,
            brand_names,
            classification_ids,
            page_size,
            page_token,
            keywords_locale,
            locale,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
