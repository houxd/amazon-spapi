use crate::{client::SpapiClient, models};
use anyhow::Result;

impl SpapiClient {
    /// Delete a listings item for a selling partner.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput can receive higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api) in the Selling Partner API documentation.
    pub async fn delete_listings_item(
        &self,
        seller_id: &str,
        sku: &str,
        marketplace_ids: Vec<String>,
        issue_locale: Option<&str>,
    ) -> Result<models::listings_items_2021_08_01::ListingsItemSubmissionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/listings/2021-08-01/deleteListingsItem", 5.0, 5)
            .await?;
        let res = crate::apis::listings_items_2021_08_01::delete_listings_item(
            &configuration,
            seller_id,
            sku,
            marketplace_ids,
            issue_locale,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns details about a listings item for a selling partner.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput can receive higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api) in the Selling Partner API documentation.
    pub async fn get_listings_item(
        &self,
        seller_id: &str,
        sku: &str,
        marketplace_ids: Vec<String>,
        issue_locale: Option<&str>,
        included_data: Option<Vec<String>>,
    ) -> Result<models::listings_items_2021_08_01::Item> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/listings/2021-08-01/getListingsItem", 5.0, 10)
            .await?;
        let res = crate::apis::listings_items_2021_08_01::get_listings_item(
            &configuration,
            seller_id,
            sku,
            marketplace_ids,
            issue_locale,
            included_data,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Partially update (patch) a listings item for a selling partner. Only top-level listings item attributes can be patched. Patching nested attributes is not supported.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput can receive higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api) in the Selling Partner API documentation.
    pub async fn patch_listings_item(
        &self,
        seller_id: &str,
        sku: &str,
        marketplace_ids: Vec<String>,
        body: models::listings_items_2021_08_01::ListingsItemPatchRequest,
        included_data: Option<Vec<String>>,
        mode: Option<&str>,
        issue_locale: Option<&str>,
    ) -> Result<models::listings_items_2021_08_01::ListingsItemSubmissionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/listings/2021-08-01/patchListingsItem", 5.0, 5)
            .await?;
        let res = crate::apis::listings_items_2021_08_01::patch_listings_item(
            &configuration,
            seller_id,
            sku,
            marketplace_ids,
            body,
            included_data,
            mode,
            issue_locale,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Creates a new or fully-updates an existing listings item for a selling partner.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput can receive higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api) in the Selling Partner API documentation.
    pub async fn put_listings_item(
        &self,
        seller_id: &str,
        sku: &str,
        marketplace_ids: Vec<String>,
        body: models::listings_items_2021_08_01::ListingsItemPutRequest,
        included_data: Option<Vec<String>>,
        mode: Option<&str>,
        issue_locale: Option<&str>,
    ) -> Result<models::listings_items_2021_08_01::ListingsItemSubmissionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/listings/2021-08-01/putListingsItem", 5.0, 10)
            .await?;
        let res = crate::apis::listings_items_2021_08_01::put_listings_item(
            &configuration,
            seller_id,
            sku,
            marketplace_ids,
            body,
            included_data,
            mode,
            issue_locale,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Search for and return a list of selling partner listings items and their respective details.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that are applied to the requested operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput might have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn search_listings_items(
        &self,
        seller_id: &str,
        marketplace_ids: Vec<String>,
        issue_locale: Option<&str>,
        included_data: Option<Vec<String>>,
        identifiers: Option<Vec<String>>,
        identifiers_type: Option<&str>,
        variation_parent_sku: Option<&str>,
        package_hierarchy_sku: Option<&str>,
        created_after: Option<String>,
        created_before: Option<String>,
        last_updated_after: Option<String>,
        last_updated_before: Option<String>,
        with_issue_severity: Option<Vec<String>>,
        with_status: Option<Vec<String>>,
        without_status: Option<Vec<String>>,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
        page_size: Option<i32>,
        page_token: Option<&str>,
    ) -> Result<models::listings_items_2021_08_01::ItemSearchResults> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/listings/2021-08-01/searchListingsItems", 5.0, 5)
            .await?;
        let res = crate::apis::listings_items_2021_08_01::search_listings_items(
            &configuration,
            seller_id,
            marketplace_ids,
            issue_locale,
            included_data,
            identifiers,
            identifiers_type,
            variation_parent_sku,
            package_hierarchy_sku,
            created_after,
            created_before,
            last_updated_after,
            last_updated_before,
            with_issue_severity,
            with_status,
            without_status,
            sort_by,
            sort_order,
            page_size,
            page_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
