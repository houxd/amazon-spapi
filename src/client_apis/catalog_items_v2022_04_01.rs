use crate::client::SpapiClient;
use crate::models::catalog_items_2022_04_01::*;
use anyhow::Result;
use std::collections::HashSet;
use std::vec;

impl SpapiClient {
    /// Search for a list of Amazon catalog items and item-related information. You can search by identifier or by keywords.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The `x-amzn-RateLimit-Limit` response header contains the usage plan rate limits for the operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput might have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn search_catalog_items(
        &self,
        marketplace_ids: Vec<String>,
        identifiers: Option<Vec<String>>,
        identifiers_type: Option<&str>,
        included_data: Option<Vec<String>>,
        locale: Option<&str>,
        seller_id: Option<&str>,
        keywords: Option<Vec<String>>,
        brand_names: Option<Vec<String>>,
        classification_ids: Option<Vec<String>>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        keywords_locale: Option<&str>,
    ) -> Result<ItemSearchResults> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/catalog/2022-04-01/searchCatalogItems", 2.0, 2)
            .await?;
        let res = crate::apis::catalog_items_2022_04_01::search_catalog_items(
            &configuration,
            marketplace_ids,
            identifiers,
            identifiers_type,
            included_data,
            locale,
            seller_id,
            keywords,
            brand_names,
            classification_ids,
            page_size,
            page_token,
            keywords_locale,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves details for an item in the Amazon catalog.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The `x-amzn-RateLimit-Limit` response header contains the usage plan rate limits for the operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput might have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_catalog_item(
        &self,
        asin: &str,
        marketplace_ids: Vec<String>,
        included_data: Option<Vec<String>>,
        locale: Option<&str>,
    ) -> Result<Item> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/catalog/2022-04-01/getCatalogItem", 2.0, 2)
            .await?;
        let res = crate::apis::catalog_items_2022_04_01::get_catalog_item(
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

    /// Convenience method to search for items by ASIN.
    pub async fn search_catalog_items_by_asin(
        &self,
        asin: &str,
        marketplace_ids: Vec<String>,
        included_data: Option<Vec<String>>,
        locale: Option<&str>,
    ) -> Result<ItemSearchResults> {
        self.search_catalog_items(
            marketplace_ids,
            Some(vec![asin.to_string()]),
            Some("ASIN"),
            included_data,
            locale,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
    }

    /// Convenience method to search for items by keywords.
    pub async fn search_catalog_items_by_keywords(
        &self,
        keywords: Vec<String>,
        marketplace_ids: Vec<String>,
        included_data: Option<Vec<String>>,
        brand_names: Option<Vec<String>>,
        classification_ids: Option<Vec<String>>,
        page_size: Option<i32>,
        page_token: Option<&str>,
        keywords_locale: Option<&str>,
    ) -> Result<ItemSearchResults> {
        self.search_catalog_items(
            marketplace_ids,
            None,
            None,
            included_data,
            None,
            None,
            Some(keywords),
            brand_names,
            classification_ids,
            page_size,
            page_token,
            keywords_locale,
        )
        .await
    }
}
