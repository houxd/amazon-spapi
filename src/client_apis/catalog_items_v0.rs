use crate::{client::SpapiClient, models};
use anyhow::Result;

impl SpapiClient {
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
