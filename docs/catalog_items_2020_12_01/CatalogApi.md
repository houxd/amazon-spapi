# \CatalogApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_catalog_item**](CatalogApi.md#get_catalog_item) | **GET** /catalog/2020-12-01/items/{asin} | 
[**search_catalog_items**](CatalogApi.md#search_catalog_items) | **GET** /catalog/2020-12-01/items | 



## get_catalog_item

> models::Item get_catalog_item(asin, marketplace_ids, included_data, locale)


Retrieves details for an item in the Amazon catalog.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asin** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers. Data sets in the response contain data only for the specified marketplaces. | [required] |
**included_data** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of data sets to include in the response. Default: summaries. |  |
**locale** | Option<**String**> | Locale for retrieving localized summaries. Defaults to the primary locale of the marketplace. |  |

### Return type

[**models::Item**](Item.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_catalog_items

> models::ItemSearchResults search_catalog_items(keywords, marketplace_ids, included_data, brand_names, classification_ids, page_size, page_token, keywords_locale, locale)


Search for and return a list of Amazon catalog items and associated information.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**keywords** | [**Vec<String>**](String.md) | A comma-delimited list of words or item identifiers to search the Amazon catalog for. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**included_data** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of data sets to include in the response. Default: summaries. |  |
**brand_names** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of brand names to limit the search to. |  |
**classification_ids** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of classification identifiers to limit the search to. |  |
**page_size** | Option<**i32**> | Number of results to be returned per page. |  |[default to 10]
**page_token** | Option<**String**> | A token to fetch a certain page when there are multiple pages worth of results. |  |
**keywords_locale** | Option<**String**> | The language the keywords are provided in. Defaults to the primary locale of the marketplace. |  |
**locale** | Option<**String**> | Locale for retrieving localized summaries. Defaults to the primary locale of the marketplace. |  |

### Return type

[**models::ItemSearchResults**](ItemSearchResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

