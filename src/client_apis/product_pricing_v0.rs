use anyhow::{anyhow, Ok, Result};
use std::vec;

use crate::{
    client::SpapiClient,
    models::{
        self,
        product_pricing_v0::{
            CustomerType, GetItemOffersBatchRequest, GetItemOffersBatchResponse,
            GetListingOffersBatchRequest, GetListingOffersBatchResponse, GetOffersResponse,
            GetPricingResponse, HttpMethod, ItemCondition, ItemOffersRequest, ListingOffersRequest,
        },
    },
};

impl SpapiClient {
    /// Returns competitive pricing information for a seller's offer listings based on seller SKU or ASIN.  **Note:** The parameters associated with this operation may contain special characters that require URL encoding to call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_competitive_pricing(
        &self,
        marketplace_id: &str,
        item_type: &str,
        asins: Option<Vec<String>>,
        skus: Option<Vec<String>>,
        customer_type: Option<&str>,
    ) -> Result<models::product_pricing_v0::GetPricingResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/products/pricing/v0/competitivePrice", 0.5, 1)
            .await?;
        let res = crate::apis::product_pricing_v0::get_competitive_pricing(
            &configuration,
            marketplace_id,
            item_type,
            asins,
            skus,
            customer_type,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the lowest priced offers for a single item based on ASIN.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_item_offers(
        &self,
        marketplace_id: &str,
        item_condition: &str,
        asin: &str,
        customer_type: Option<&str>,
    ) -> Result<models::product_pricing_v0::GetOffersResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/products/pricing/v0/items/{Asin}/offers", 0.5, 1)
            .await?;
        let res = crate::apis::product_pricing_v0::get_item_offers(
            &configuration,
            marketplace_id,
            item_condition,
            asin,
            customer_type,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the lowest priced offers for a batch of items based on ASIN.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.1 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_item_offers_batch(
        &self,
        get_item_offers_batch_request_body: models::product_pricing_v0::GetItemOffersBatchRequest,
    ) -> Result<models::product_pricing_v0::GetItemOffersBatchResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/batches/products/pricing/v0/itemOffers", 0.1, 1)
            .await?;

        let res = crate::apis::product_pricing_v0::get_item_offers_batch(
            &configuration,
            get_item_offers_batch_request_body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the lowest priced offers for a single SKU listing.  **Note:** The parameters associated with this operation may contain special characters that require URL encoding to call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 2 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_listing_offers(
        &self,
        marketplace_id: &str,
        item_condition: &str,
        seller_sku: &str,
        customer_type: Option<&str>,
    ) -> Result<models::product_pricing_v0::GetOffersResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/products/pricing/v0/listings/{SellerSKU}/offers", 1.0, 2)
            .await?;
        let res = crate::apis::product_pricing_v0::get_listing_offers(
            &configuration,
            marketplace_id,
            item_condition,
            seller_sku,
            customer_type,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the lowest priced offers for a batch of listings by SKU.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_listing_offers_batch(
        &self,
        get_listing_offers_batch_request_body: models::product_pricing_v0::GetListingOffersBatchRequest,
    ) -> Result<models::product_pricing_v0::GetListingOffersBatchResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/batches/products/pricing/v0/listingOffers", 0.5, 1)
            .await?;

        let res = crate::apis::product_pricing_v0::get_listing_offers_batch(
            &configuration,
            get_listing_offers_batch_request_body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns pricing information for a seller's offer listings based on seller SKU or ASIN.  **Note:** The parameters associated with this operation may contain special characters that require URL encoding to call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_pricing(
        &self,
        marketplace_id: &str,
        item_type: &str,
        asins: Option<Vec<String>>,
        skus: Option<Vec<String>>,
        item_condition: Option<&str>,
        offer_type: Option<&str>,
    ) -> Result<models::product_pricing_v0::GetPricingResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/products/pricing/v0/price", 0.5, 1)
            .await?;
        let res = crate::apis::product_pricing_v0::get_pricing(
            &configuration,
            marketplace_id,
            item_type,
            asins,
            skus,
            item_condition,
            offer_type,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Convenience method to get item offers for multiple ASINs in a single batch request.
    /// - asins: length between 1 and 20
    pub async fn get_item_offers_batch_by_asins(
        &self,
        asins: Vec<&str>,
        marketplace_id: &str,
    ) -> Result<GetItemOffersBatchResponse> {
        let item_offer_requests = asins
            .iter()
            .map(|&asin| {
                ItemOffersRequest {
                    uri: format!(
                        "/products/pricing/v0/items/{Asin}/offers",
                        Asin = crate::apis::urlencode(asin)
                    ),
                    method: HttpMethod::Get,
                    headers: None,
                    marketplace_id: marketplace_id.to_string(),
                    item_condition: ItemCondition::New,
                    customer_type: None, // Default to CustomerType::Consumer if None
                }
            })
            .collect();
        let request = GetItemOffersBatchRequest {
            requests: Some(item_offer_requests),
        };
        self.get_item_offers_batch(request).await
    }

    /// Convenience method to get listing offers for multiple SKUs in a single batch request.
    /// - skus: length between 1 and 20
    pub async fn get_listing_offers_batch_by_skus(
        &self,
        skus: Vec<&str>,
        marketplace_id: &str,
    ) -> Result<GetListingOffersBatchResponse> {
        let listing_offer_requests = skus
            .iter()
            .map(|&sku| {
                ListingOffersRequest {
                    uri: format!(
                        "/products/pricing/v0/listings/{SellerSKU}/offers",
                        SellerSKU = crate::apis::urlencode(sku)
                    ),
                    method: HttpMethod::Get,
                    headers: None,
                    marketplace_id: marketplace_id.to_string(),
                    item_condition: ItemCondition::New,
                    customer_type: None, // Default to CustomerType::Consumer if None
                }
            })
            .collect();
        self.get_listing_offers_batch(GetListingOffersBatchRequest {
            requests: Some(listing_offer_requests),
        })
        .await
    }
}
