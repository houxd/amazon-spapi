use crate::client::{ApiEndpoint, ApiMethod, SpapiClient};
use crate::models;
use anyhow::Result;
use std::collections::HashSet;
use std::vec;

impl SpapiClient {
    pub async fn get_catalog_item_v2020_12_01(
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
    pub async fn search_catalog_items_v2020_12_01(
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
