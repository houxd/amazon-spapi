use crate::{client::SpapiClient, models};
use anyhow::Result;

impl SpapiClient {
    /// Retrieve an Amazon product type definition.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_definitions_product_type(
        &self,
        product_type: &str,
        marketplace_ids: Vec<String>,
        seller_id: Option<&str>,
        product_type_version: Option<&str>,
        requirements: Option<&str>,
        requirements_enforced: Option<&str>,
        locale: Option<&str>,
    ) -> Result<models::definitions_product_types_2020_09_01::ProductTypeDefinition> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/definitions/2020-09-01/productTypes/{productType}", 5.0, 10)
            .await?;

        let res = crate::apis::product_type_definitions_2020_09_01::get_definitions_product_type(
            &configuration,
            product_type,
            marketplace_ids,
            seller_id,
            product_type_version,
            requirements,
            requirements_enforced,
            locale,
        )
        .await?;

        guard.mark_response().await;
        Ok(res)
    }

    /// Search for and return a list of Amazon product types that have definitions available.  **Usage Plans:**  | Plan type | Rate (requests per second) | Burst | | ---- | ---- | ---- | |Default| 5 | 10 | |Selling partner specific| Variable | Variable |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation. Rate limits for some selling partners will vary from the default rate and burst shown in the table above. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn search_definitions_product_types(
        &self,
        marketplace_ids: Vec<String>,
        keywords: Option<Vec<String>>,
        item_name: Option<&str>,
        locale: Option<&str>,
        search_locale: Option<&str>,
    ) -> Result<models::definitions_product_types_2020_09_01::ProductTypeList> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/definitions/2020-09-01/productTypes", 5.0, 10)
            .await?;

        let res =
            crate::apis::product_type_definitions_2020_09_01::search_definitions_product_types(
                &configuration,
                marketplace_ids,
                keywords,
                item_name,
                locale,
                search_locale,
            )
            .await?;

        guard.mark_response().await;
        Ok(res)
    }
}
