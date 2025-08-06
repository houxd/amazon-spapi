use anyhow::{anyhow, Ok, Result};
use std::vec;

use crate::{
    client::{ApiEndpoint, ApiMethod, SpapiClient},
    models::product_pricing_v0::{
        CustomerType, GetItemOffersBatchRequest, GetItemOffersBatchResponse,
        GetListingOffersBatchRequest, GetListingOffersBatchResponse, GetOffersResponse,
        GetPricingResponse, HttpMethod, ItemCondition, ItemOffersRequest, ListingOffersRequest,
    },
};

impl SpapiClient {
    /// Returns pricing information for a seller's offer listings based on seller SKU or ASIN.
    ///
    /// # Arguments
    /// * `marketplace_id` - A marketplace identifier. Specifies the marketplace for which prices are returned.
    /// * `item_type` - Indicates whether ASIN values or seller SKU values are used to identify items. Possible values: "Asin", "Sku".
    /// * `asins` - A list of up to twenty Amazon Standard Identification Number (ASIN) values used to identify items in the given marketplace.
    /// * `skus` - A list of up to twenty seller SKU values used to identify items in the given marketplace.
    /// * `item_condition` - Filters the offer listings based on item condition. Possible values: "New", "Used", "Collectible", "Refurbished", "Club".
    /// * `offer_type` - Indicates whether to request pricing information for the seller's B2C or B2B offers. Default is "B2C".
    ///
    /// # Rate Limits
    /// - Rate: 0.5 requests per second
    /// - Burst: 1
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SpApiClient::new(config);
    /// let response = client.get_pricing(
    ///     "ATVPDKIKX0DER",
    ///     "Asin",
    ///     Some(vec!["B08N5WRWNW".to_string(), "B07XJ8C8F7".to_string()]),
    ///     None,
    ///     Some("New".to_string()),
    ///     Some("B2C".to_string()),
    /// ).await?;
    ///
    /// println!("Pricing information: {:?}", response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_pricing(
        &self,
        marketplace_id: &str,
        item_type: &str,
        asins: Option<Vec<String>>,
        skus: Option<Vec<String>>,
        item_condition: Option<String>,
        offer_type: Option<String>,
    ) -> Result<GetPricingResponse> {
        let mut query_params = vec![];
        query_params.push(("MarketplaceId".to_string(), marketplace_id.to_string()));
        query_params.push(("ItemType".to_string(), item_type.to_string()));

        if let Some(asins_list) = asins {
            for (i, asin) in asins_list.iter().enumerate() {
                query_params.push((format!("Asins.Asin.{}", i + 1), asin.clone()));
            }
        }

        if let Some(skus_list) = skus {
            for (i, sku) in skus_list.iter().enumerate() {
                query_params.push((format!("Skus.SellerSKU.{}", i + 1), sku.clone()));
            }
        }

        if let Some(condition) = item_condition {
            query_params.push(("ItemCondition".to_string(), condition));
        }

        if let Some(offer_type_val) = offer_type {
            query_params.push(("OfferType".to_string(), offer_type_val));
        }

        let endpoint = ApiEndpoint {
            version: "pricing/v0",
            path: "/products/pricing/v0/price",
            path_params: None,
            method: ApiMethod::Get,
            rate: 0.5,
            burst: 1,
        };

        let res = self
            .request(&endpoint, Some(query_params), None, None)
            .await?;
        Self::from_json(&res)
    }

    /// Returns competitive pricing information for a seller's offer listings based on seller SKU or ASIN.
    ///
    /// # Arguments
    /// * `marketplace_id` - A marketplace identifier. Specifies the marketplace for which prices are returned.
    /// * `item_type` - Indicates whether ASIN values or seller SKU values are used to identify items. Possible values: "Asin", "Sku".
    /// * `asins` - A list of up to twenty Amazon Standard Identification Number (ASIN) values used to identify items in the given marketplace.
    /// * `skus` - A list of up to twenty seller SKU values used to identify items in the given marketplace.
    /// * `customer_type` - Indicates whether to request Consumer or Business offers. Default is "Consumer".
    ///
    /// # Rate Limits
    /// - Rate: 0.5 requests per second
    /// - Burst: 1
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SpApiClient::new(config);
    /// let response = client.get_competitive_pricing(
    ///     "ATVPDKIKX0DER",
    ///     "Asin",
    ///     Some(vec!["B08N5WRWNW".to_string(), "B07XJ8C8F7".to_string()]),
    ///     None,
    ///     Some("Consumer".to_string()),
    /// ).await?;
    ///
    /// println!("Competitive pricing: {:?}", response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_competitive_pricing(
        &self,
        marketplace_id: &str,
        item_type: &str,
        asins: Option<Vec<String>>,
        skus: Option<Vec<String>>,
        customer_type: Option<String>,
    ) -> Result<GetPricingResponse> {
        let mut query_params = vec![];
        query_params.push(("MarketplaceId".to_string(), marketplace_id.to_string()));
        query_params.push(("ItemType".to_string(), item_type.to_string()));

        if let Some(asins_list) = asins {
            for (i, asin) in asins_list.iter().enumerate() {
                query_params.push((format!("Asins.Asin.{}", i + 1), asin.clone()));
            }
        }

        if let Some(skus_list) = skus {
            for (i, sku) in skus_list.iter().enumerate() {
                query_params.push((format!("Skus.SellerSKU.{}", i + 1), sku.clone()));
            }
        }

        if let Some(customer_type_val) = customer_type {
            query_params.push(("CustomerType".to_string(), customer_type_val));
        }

        let endpoint = ApiEndpoint {
            version: "pricing/v0",
            path: "/products/pricing/v0/competitivePrice",
            path_params: None,
            method: ApiMethod::Get,
            rate: 0.5,
            burst: 1,
        };

        let res = self
            .request(&endpoint, Some(query_params), None, None)
            .await?;
        Self::from_json(&res)
    }

    /// Returns the lowest priced offers for a single item based on ASIN.
    ///
    /// # Arguments
    /// * `marketplace_id` - A marketplace identifier. Specifies the marketplace for which prices are returned.
    /// * `asin` - The Amazon Standard Identification Number (ASIN) of the item for which you want to retrieve offer information.
    /// * `item_condition` - Filters the offer listings to be considered based on item condition. Possible values: "New", "Used", "Collectible", "Refurbished", "Club".
    /// * `customer_type` - Indicates whether to request Consumer or Business offers. Default is "Consumer".
    ///
    /// # Rate Limits
    /// - Rate: 0.5 requests per second
    /// - Burst: 1
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SpApiClient::new(config);
    /// let response = client.get_item_offers(
    ///     "ATVPDKIKX0DER",
    ///     "B08N5WRWNW",
    ///     "New",
    ///     Some("Consumer".to_string()),
    /// ).await?;
    ///
    /// println!("Item offers: {:?}", response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_item_offers(
        &self,
        marketplace_id: &str,
        asin: &str,
        item_condition: &str,
        customer_type: Option<String>,
    ) -> Result<GetOffersResponse> {
        let mut query_params = vec![];
        query_params.push(("MarketplaceId".to_string(), marketplace_id.to_string()));
        query_params.push(("ItemCondition".to_string(), item_condition.to_string()));

        if let Some(customer_type_val) = customer_type {
            query_params.push(("CustomerType".to_string(), customer_type_val));
        }

        let endpoint = ApiEndpoint {
            version: "pricing/v0",
            path: "/products/pricing/v0/items/{asin}/offers",
            path_params: Some(vec![("asin", asin.to_string())]),
            method: ApiMethod::Get,
            rate: 0.5,
            burst: 1,
        };

        let res = self
            .request(&endpoint, Some(query_params), None, None)
            .await?;
        Self::from_json(&res)
    }

    /// Returns the lowest priced offers for a single SKU listing.
    ///
    /// # Arguments
    /// * `marketplace_id` - A marketplace identifier. Specifies the marketplace for which prices are returned.
    /// * `seller_sku` - A seller identifier that you want to retrieve offer information for.
    /// * `item_condition` - Filters the offer listings to be considered based on item condition. Possible values: "New", "Used", "Collectible", "Refurbished", "Club".
    /// * `customer_type` - Indicates whether to request Consumer or Business offers. Default is "Consumer".
    ///
    /// # Rate Limits
    /// - Rate: 1 request per second
    /// - Burst: 2
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SpApiClient::new(config);
    /// let response = client.get_listing_offers(
    ///     "ATVPDKIKX0DER",
    ///     "MY-SKU-123",
    ///     "New",
    ///     Some("Consumer".to_string()),
    /// ).await?;
    ///
    /// println!("Listing offers: {:?}", response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_listing_offers(
        &self,
        marketplace_id: &str,
        seller_sku: &str,
        item_condition: &str,
        customer_type: Option<String>,
    ) -> Result<GetOffersResponse> {
        let mut query_params = vec![];
        query_params.push(("MarketplaceId".to_string(), marketplace_id.to_string()));
        query_params.push(("ItemCondition".to_string(), item_condition.to_string()));

        if let Some(customer_type_val) = customer_type {
            query_params.push(("CustomerType".to_string(), customer_type_val));
        }

        let endpoint = ApiEndpoint {
            version: "pricing/v0",
            path: "/products/pricing/v0/listings/{seller_sku}/offers",
            path_params: Some(vec![("seller_sku", seller_sku.to_string())]),
            method: ApiMethod::Get,
            rate: 1.0,
            burst: 2,
        };

        let res = self
            .request(&endpoint, Some(query_params), None, None)
            .await?;
        Self::from_json(&res)
    }

    /// Returns the set of responses that correspond to the batched list of up to 20 requests defined in the request body.
    /// The response for each successful (HTTP status code 200) request in the set includes the item offers for a given ASIN.
    ///
    /// # Arguments
    /// * `request` - The batch of `getItemOffers` requests.
    ///
    /// # Rate Limits
    /// - Rate: 0.1 requests per second
    /// - Burst: 1
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SpApiClient::new(config);
    /// use spapi::models::pricing_v0::*;
    ///
    /// let batch_request = GetItemOffersBatchRequest {
    ///     requests: Some(vec![
    ///         ItemOffersRequest {
    ///             uri: "/products/pricing/v0/items/B123456789/offers".to_string(),
    ///             method: HttpMethod::Get,
    ///             marketplace_id: "ATVPDKIKX0DER".to_string(),
    ///             item_condition: "New".to_string(),
    ///             customer_type: Some("Consumer".to_string()),
    ///             headers: None,
    ///         },
    ///     ]),
    /// };
    ///
    /// let response = client.get_item_offers_batch(batch_request).await?;
    /// println!("Batch item offers: {:?}", response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_item_offers_batch(
        &self,
        request: GetItemOffersBatchRequest,
    ) -> Result<GetItemOffersBatchResponse> {
        let endpoint = ApiEndpoint {
            version: "pricing/v0",
            path: "/batches/products/pricing/v0/itemOffers",
            path_params: None,
            method: ApiMethod::Post,
            rate: 0.1,
            burst: 1,
        };

        let body = serde_json::to_string(&request)?;
        let res = self.request(&endpoint, None, None, Some(&body)).await?;
        Self::from_json(&res)
    }

    /// Returns the set of responses that correspond to the batched list of up to 20 requests defined in the request body.
    /// The response for each successful (HTTP status code 200) request in the set includes the listing offers for a given SKU.
    ///
    /// # Arguments
    /// * `request` - The batch of `getListingOffers` requests.
    ///
    /// # Rate Limits
    /// - Rate: 0.5 requests per second
    /// - Burst: 1
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SpApiClient::new(config);
    /// use spapi::models::pricing_v0::*;
    ///
    /// let batch_request = GetListingOffersBatchRequest {
    ///     requests: Some(vec![
    ///         ListingOffersRequest {
    ///             uri: "/products/pricing/v0/listings/MY-SKU-123/offers".to_string(),
    ///             method: HttpMethod::Get,
    ///             marketplace_id: "ATVPDKIKX0DER".to_string(),
    ///             item_condition: "New".to_string(),
    ///             customer_type: Some("Consumer".to_string()),
    ///             headers: None,
    ///         },
    ///     ]),
    /// };
    ///
    /// let response = client.get_listing_offers_batch(batch_request).await?;
    /// println!("Batch listing offers: {:?}", response);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_listing_offers_batch(
        &self,
        request: GetListingOffersBatchRequest,
    ) -> Result<GetListingOffersBatchResponse> {
        let endpoint = ApiEndpoint {
            version: "pricing/v0",
            path: "/batches/products/pricing/v0/listingOffers",
            path_params: None,
            method: ApiMethod::Post,
            // 测试0.5时quotaExceeded, 所以降低访问频率
            rate: 0.1, //0.5,
            burst: 1,
        };

        let body = serde_json::to_string(&request)?;

        log::debug!("Request body: {}", body);

        let res = self.request(&endpoint, None, None, Some(&body)).await?;
        Self::from_json(&res)
    }

    /// Convenience method to get item offers for multiple ASINs in a single batch request.
    /// - asins: length between 1 and 20
    pub async fn get_item_offers_batch_by_asins(
        &self,
        asins: Vec<&str>,
    ) -> Result<GetItemOffersBatchResponse> {
        let marketplace_id = self.get_marketplace_id();
        let item_offer_requests = asins
            .iter()
            .map(|&asin| {
                ItemOffersRequest {
                    uri: format!("/products/pricing/v0/items/{}/offers", asin),
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
    ) -> Result<GetListingOffersBatchResponse> {
        let marketplace_id = self.get_marketplace_id();
        let listing_offer_requests = skus
            .iter()
            .map(|&sku| {
                ListingOffersRequest {
                    uri: format!("/products/pricing/v0/listings/{}/offers", sku),
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

// Helper functions for batch requests

/// Helper function to create a batch request for getting item offers
pub fn create_item_offers_batch_request(
    marketplace_id: &str,
    asins: &[&str],
    item_condition: ItemCondition,
    customer_type: Option<CustomerType>,
) -> GetItemOffersBatchRequest {
    let requests = asins
        .iter()
        .map(|asin| ItemOffersRequest {
            uri: format!("/products/pricing/v0/items/{}/offers", asin),
            method: HttpMethod::Get,
            marketplace_id: marketplace_id.to_string(),
            item_condition: item_condition.clone(),
            customer_type: customer_type.clone(),
            headers: None,
        })
        .collect();

    GetItemOffersBatchRequest {
        requests: Some(requests),
    }
}

/// Helper function to create a batch request for getting listing offers
pub fn create_listing_offers_batch_request(
    marketplace_id: &str,
    skus: &[&str],
    item_condition: ItemCondition,
    customer_type: Option<CustomerType>,
) -> GetListingOffersBatchRequest {
    let requests = skus
        .iter()
        .map(|sku| ListingOffersRequest {
            uri: format!("/products/pricing/v0/listings/{}/offers", sku),
            method: HttpMethod::Get,
            marketplace_id: marketplace_id.to_string(),
            item_condition: item_condition.clone(),
            customer_type: customer_type.clone(),
            headers: None,
        })
        .collect();

    GetListingOffersBatchRequest {
        requests: Some(requests),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_item_offers_batch_request() {
        let asins = vec!["B123456789", "B987654321"];
        let batch_request = create_item_offers_batch_request(
            "ATVPDKIKX0DER",
            &asins,
            ItemCondition::New,
            Some(CustomerType::Consumer),
        );

        assert!(batch_request.requests.is_some());
        let requests = batch_request.requests.unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].marketplace_id, "ATVPDKIKX0DER");
        assert_eq!(requests[0].item_condition, ItemCondition::New);
        assert_eq!(requests[0].customer_type, Some(CustomerType::Consumer));
    }

    #[test]
    fn test_create_listing_offers_batch_request() {
        let skus = vec!["MY-SKU-123", "MY-SKU-456"];
        let batch_request = create_listing_offers_batch_request(
            "ATVPDKIKX0DER",
            &skus,
            ItemCondition::New,
            Some(CustomerType::Consumer),
        );

        assert!(batch_request.requests.is_some());
        let requests = batch_request.requests.unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].marketplace_id, "ATVPDKIKX0DER");
        assert_eq!(requests[0].item_condition, ItemCondition::New);
        assert_eq!(requests[0].customer_type, Some(CustomerType::Consumer));
    }
}
