# \CatalogApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_catalog_item**](CatalogApi.md#get_catalog_item) | **GET** /catalog/2022-04-01/items/{asin} | 
[**search_catalog_items**](CatalogApi.md#search_catalog_items) | **GET** /catalog/2022-04-01/items | 



## get_catalog_item

> models::Item get_catalog_item(asin, marketplace_ids, included_data, locale)


Retrieves details for an item in the Amazon catalog.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The `x-amzn-RateLimit-Limit` response header contains the usage plan rate limits for the operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput might have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asin** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers. To find the ID for your marketplace, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | [required] |
**included_data** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of datasets to include in the response. |  |[default to ["summaries"]]
**locale** | Option<**String**> | The locale for which you want to retrieve localized summaries. Defaults to the primary locale of the marketplace. |  |

### Return type

[**models::Item**](Item.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_catalog_items

> models::ItemSearchResults search_catalog_items(marketplace_ids, identifiers, identifiers_type, included_data, locale, seller_id, keywords, brand_names, classification_ids, page_size, page_token, keywords_locale)


Search for a list of Amazon catalog items and item-related information. You can search by identifier or by keywords.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The `x-amzn-RateLimit-Limit` response header contains the usage plan rate limits for the operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput might have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers. To find the ID for your marketplace, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | [required] |
**identifiers** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of product identifiers that you can use to search the Amazon catalog. **Note:** You cannot include `identifiers` and `keywords` in the same request. |  |
**identifiers_type** | Option<**String**> | The type of product identifiers that you can use to search the Amazon catalog. **Note:** `identifiersType` is required when `identifiers` is in the request. |  |
**included_data** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of datasets to include in the response. |  |[default to ["summaries"]]
**locale** | Option<**String**> | The locale for which you want to retrieve localized summaries. Defaults to the primary locale of the marketplace. |  |
**seller_id** | Option<**String**> | A selling partner identifier, such as a seller account or vendor code. **Note:** Required when `identifiersType` is `SKU`. |  |
**keywords** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of keywords that you can use to search the Amazon catalog. **Note:** You cannot include `keywords` and `identifiers` in the same request. |  |
**brand_names** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of brand names that you can use to limit the search in queries based on `keywords`. **Note:** Cannot be used with `identifiers`. |  |
**classification_ids** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of classification identifiers that you can use to limit the search in queries based on `keywords`. **Note:** Cannot be used with `identifiers`. |  |
**page_size** | Option<**i32**> | The number of results to include on each page. |  |[default to 10]
**page_token** | Option<**String**> | A token that you can use to fetch a specific page when there are multiple pages of results. |  |
**keywords_locale** | Option<**String**> | The language of the keywords that are included in queries based on `keywords`. Defaults to the primary locale of the marketplace. **Note:** Cannot be used with `identifiers`. |  |

### Return type

[**models::ItemSearchResults**](ItemSearchResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

