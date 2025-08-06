# \ProductPricingApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_competitive_pricing**](ProductPricingApi.md#get_competitive_pricing) | **GET** /products/pricing/v0/competitivePrice | 
[**get_item_offers**](ProductPricingApi.md#get_item_offers) | **GET** /products/pricing/v0/items/{Asin}/offers | 
[**get_item_offers_batch**](ProductPricingApi.md#get_item_offers_batch) | **POST** /batches/products/pricing/v0/itemOffers | 
[**get_listing_offers**](ProductPricingApi.md#get_listing_offers) | **GET** /products/pricing/v0/listings/{SellerSKU}/offers | 
[**get_listing_offers_batch**](ProductPricingApi.md#get_listing_offers_batch) | **POST** /batches/products/pricing/v0/listingOffers | 
[**get_pricing**](ProductPricingApi.md#get_pricing) | **GET** /products/pricing/v0/price | 



## get_competitive_pricing

> models::GetPricingResponse get_competitive_pricing(marketplace_id, item_type, asins, skus, customer_type)


Returns competitive pricing information for a seller's offer listings based on seller SKU or ASIN.  **Note:** The parameters associated with this operation may contain special characters that require URL encoding to call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. Specifies the marketplace for which prices are returned. | [required] |
**item_type** | **String** | Indicates whether ASIN values or seller SKU values are used to identify items. If you specify Asin, the information in the response will be dependent on the list of Asins you provide in the Asins parameter. If you specify Sku, the information in the response will be dependent on the list of Skus you provide in the Skus parameter. Possible values: Asin, Sku. | [required] |
**asins** | Option<[**Vec<String>**](String.md)> | A list of up to twenty Amazon Standard Identification Number (ASIN) values used to identify items in the given marketplace. |  |
**skus** | Option<[**Vec<String>**](String.md)> | A list of up to twenty seller SKU values used to identify items in the given marketplace. |  |
**customer_type** | Option<**String**> | Indicates whether to request pricing information from the point of view of Consumer or Business buyers. Default is Consumer. |  |

### Return type

[**models::GetPricingResponse**](GetPricingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_offers

> models::GetOffersResponse get_item_offers(marketplace_id, item_condition, asin, customer_type)


Returns the lowest priced offers for a single item based on ASIN.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. Specifies the marketplace for which prices are returned. | [required] |
**item_condition** | **String** | Filters the offer listings to be considered based on item condition. Possible values: New, Used, Collectible, Refurbished, Club. | [required] |
**asin** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [required] |
**customer_type** | Option<**String**> | Indicates whether to request Consumer or Business offers. Default is Consumer. |  |

### Return type

[**models::GetOffersResponse**](GetOffersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_offers_batch

> models::GetItemOffersBatchResponse get_item_offers_batch(get_item_offers_batch_request_body)


Returns the lowest priced offers for a batch of items based on ASIN.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.1 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_item_offers_batch_request_body** | [**GetItemOffersBatchRequest**](GetItemOffersBatchRequest.md) | The request associated with the `getItemOffersBatch` API call. | [required] |

### Return type

[**models::GetItemOffersBatchResponse**](GetItemOffersBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_listing_offers

> models::GetOffersResponse get_listing_offers(marketplace_id, item_condition, seller_sku, customer_type)


Returns the lowest priced offers for a single SKU listing.  **Note:** The parameters associated with this operation may contain special characters that require URL encoding to call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 2 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. Specifies the marketplace for which prices are returned. | [required] |
**item_condition** | **String** | Filters the offer listings based on item condition. Possible values: New, Used, Collectible, Refurbished, Club. | [required] |
**seller_sku** | **String** | Identifies an item in the given marketplace. SellerSKU is qualified by the seller's SellerId, which is included with every operation that you submit. | [required] |
**customer_type** | Option<**String**> | Indicates whether to request Consumer or Business offers. Default is Consumer. |  |

### Return type

[**models::GetOffersResponse**](GetOffersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_listing_offers_batch

> models::GetListingOffersBatchResponse get_listing_offers_batch(get_listing_offers_batch_request_body)


Returns the lowest priced offers for a batch of listings by SKU.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_listing_offers_batch_request_body** | [**GetListingOffersBatchRequest**](GetListingOffersBatchRequest.md) | The request associated with the `getListingOffersBatch` API call. | [required] |

### Return type

[**models::GetListingOffersBatchResponse**](GetListingOffersBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pricing

> models::GetPricingResponse get_pricing(marketplace_id, item_type, asins, skus, item_condition, offer_type)


Returns pricing information for a seller's offer listings based on seller SKU or ASIN.  **Note:** The parameters associated with this operation may contain special characters that require URL encoding to call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. Specifies the marketplace for which prices are returned. | [required] |
**item_type** | **String** | Indicates whether ASIN values or seller SKU values are used to identify items. If you specify Asin, the information in the response will be dependent on the list of Asins you provide in the Asins parameter. If you specify Sku, the information in the response will be dependent on the list of Skus you provide in the Skus parameter. | [required] |
**asins** | Option<[**Vec<String>**](String.md)> | A list of up to twenty Amazon Standard Identification Number (ASIN) values used to identify items in the given marketplace. |  |
**skus** | Option<[**Vec<String>**](String.md)> | A list of up to twenty seller SKU values used to identify items in the given marketplace. |  |
**item_condition** | Option<**String**> | Filters the offer listings based on item condition. Possible values: New, Used, Collectible, Refurbished, Club. |  |
**offer_type** | Option<**String**> | Indicates whether to request pricing information for the seller's B2C or B2B offers. Default is B2C. |  |

### Return type

[**models::GetPricingResponse**](GetPricingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

