# \ListingsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_listings_item**](ListingsApi.md#delete_listings_item) | **DELETE** /listings/2021-08-01/items/{sellerId}/{sku} | 
[**get_listings_item**](ListingsApi.md#get_listings_item) | **GET** /listings/2021-08-01/items/{sellerId}/{sku} | 
[**patch_listings_item**](ListingsApi.md#patch_listings_item) | **PATCH** /listings/2021-08-01/items/{sellerId}/{sku} | 
[**put_listings_item**](ListingsApi.md#put_listings_item) | **PUT** /listings/2021-08-01/items/{sellerId}/{sku} | 
[**search_listings_items**](ListingsApi.md#search_listings_items) | **GET** /listings/2021-08-01/items/{sellerId} | 



## delete_listings_item

> models::ListingsItemSubmissionResponse delete_listings_item(seller_id, sku, marketplace_ids, issue_locale)


Delete a listings item for a selling partner.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput can receive higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api) in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_id** | **String** | A selling partner identifier, such as a merchant account or vendor code. | [required] |
**sku** | **String** | A selling partner provided identifier for an Amazon listing. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**issue_locale** | Option<**String**> | A locale for localization of issues. When not provided, the default language code of the first marketplace is used. Examples: `en_US`, `fr_CA`, `fr_FR`. Localized messages default to `en_US` when a localization is not available in the specified locale. |  |

### Return type

[**models::ListingsItemSubmissionResponse**](ListingsItemSubmissionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_listings_item

> models::Item get_listings_item(seller_id, sku, marketplace_ids, issue_locale, included_data)


Returns details about a listings item for a selling partner.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput can receive higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api) in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_id** | **String** | A selling partner identifier, such as a merchant account or vendor code. | [required] |
**sku** | **String** | A selling partner provided identifier for an Amazon listing. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**issue_locale** | Option<**String**> | A locale for localization of issues. When not provided, the default language code of the first marketplace is used. Examples: `en_US`, `fr_CA`, `fr_FR`. Localized messages default to `en_US` when a localization is not available in the specified locale. |  |
**included_data** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of data sets to include in the response. Default: `summaries`. |  |[default to ["summaries"]]

### Return type

[**models::Item**](Item.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_listings_item

> models::ListingsItemSubmissionResponse patch_listings_item(seller_id, sku, marketplace_ids, body, included_data, mode, issue_locale)


Partially update (patch) a listings item for a selling partner. Only top-level listings item attributes can be patched. Patching nested attributes is not supported.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput can receive higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api) in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_id** | **String** | A selling partner identifier, such as a merchant account or vendor code. | [required] |
**sku** | **String** | A selling partner provided identifier for an Amazon listing. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**body** | [**ListingsItemPatchRequest**](ListingsItemPatchRequest.md) | The request body schema for the `patchListingsItem` operation. | [required] |
**included_data** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of data sets to include in the response. Default: `issues`. |  |[default to ["issues"]]
**mode** | Option<**String**> | The mode of operation for the request. |  |
**issue_locale** | Option<**String**> | A locale for localization of issues. When not provided, the default language code of the first marketplace is used. Examples: `en_US`, `fr_CA`, `fr_FR`. Localized messages default to `en_US` when a localization is not available in the specified locale. |  |

### Return type

[**models::ListingsItemSubmissionResponse**](ListingsItemSubmissionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_listings_item

> models::ListingsItemSubmissionResponse put_listings_item(seller_id, sku, marketplace_ids, body, included_data, mode, issue_locale)


Creates a new or fully-updates an existing listings item for a selling partner.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput can receive higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api) in the Selling Partner API documentation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_id** | **String** | A selling partner identifier, such as a merchant account or vendor code. | [required] |
**sku** | **String** | A selling partner provided identifier for an Amazon listing. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**body** | [**ListingsItemPutRequest**](ListingsItemPutRequest.md) | The request body schema for the `putListingsItem` operation. | [required] |
**included_data** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of data sets to include in the response. Default: `issues`. |  |[default to ["issues"]]
**mode** | Option<**String**> | The mode of operation for the request. |  |
**issue_locale** | Option<**String**> | A locale for localization of issues. When not provided, the default language code of the first marketplace is used. Examples: `en_US`, `fr_CA`, `fr_FR`. Localized messages default to `en_US` when a localization is not available in the specified locale. |  |

### Return type

[**models::ListingsItemSubmissionResponse**](ListingsItemSubmissionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_listings_items

> models::ItemSearchResults search_listings_items(seller_id, marketplace_ids, issue_locale, included_data, identifiers, identifiers_type, variation_parent_sku, package_hierarchy_sku, created_after, created_before, last_updated_after, last_updated_before, with_issue_severity, with_status, without_status, sort_by, sort_order, page_size, page_token)


Search for and return a list of selling partner listings items and their respective details.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that are applied to the requested operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput might have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_id** | **String** | A selling partner identifier, such as a merchant account or vendor code. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**issue_locale** | Option<**String**> | A locale that is used to localize issues. When not provided, the default language code of the first marketplace is used. Examples: \"en_US\", \"fr_CA\", \"fr_FR\". When a localization is not available in the specified locale, localized messages default to \"en_US\". |  |
**included_data** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of datasets that you want to include in the response. Default: `summaries`. |  |[default to ["summaries"]]
**identifiers** | Option<[**Vec<String>**](String.md)> | A comma-delimited list of product identifiers that you can use to search for listings items.   **Note**:  1. This is required when you specify `identifiersType`. 2. You cannot use 'identifiers' if you specify `variationParentSku` or `packageHierarchySku`. |  |
**identifiers_type** | Option<**String**> | A type of product identifiers that you can use to search for listings items.   **Note**:  This is required when `identifiers` is provided. |  |
**variation_parent_sku** | Option<**String**> | Filters results to include listing items that are variation children of the specified SKU.   **Note**: You cannot use `variationParentSku` if you include `identifiers` or `packageHierarchySku` in your request. |  |
**package_hierarchy_sku** | Option<**String**> | Filter results to include listing items that contain or are contained by the specified SKU.   **Note**: You cannot use `packageHierarchySku` if you include `identifiers` or `variationParentSku` in your request. |  |
**created_after** | Option<**String**> | A date-time that is used to filter listing items. The response includes listings items that were created at or after this time. Values are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. |  |
**created_before** | Option<**String**> | A date-time that is used to filter listing items. The response includes listings items that were created at or before this time. Values are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. |  |
**last_updated_after** | Option<**String**> | A date-time that is used to filter listing items. The response includes listings items that were last updated at or after this time. Values are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. |  |
**last_updated_before** | Option<**String**> | A date-time that is used to filter listing items. The response includes listings items that were last updated at or before this time. Values are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. |  |
**with_issue_severity** | Option<[**Vec<String>**](String.md)> | Filter results to include only listing items that have issues that match one or more of the specified severity levels. |  |
**with_status** | Option<[**Vec<String>**](String.md)> | Filter results to include only listing items that have the specified status. |  |
**without_status** | Option<[**Vec<String>**](String.md)> | Filter results to include only listing items that don't contain the specified statuses. |  |
**sort_by** | Option<**String**> | An attribute by which to sort the returned listing items. |  |[default to lastUpdatedDate]
**sort_order** | Option<**String**> | The order in which to sort the result items. |  |[default to DESC]
**page_size** | Option<**i32**> | The number of results that you want to include on each page. |  |[default to 10]
**page_token** | Option<**String**> | A token that you can use to fetch a specific page when there are multiple pages of results. |  |

### Return type

[**models::ItemSearchResults**](ItemSearchResults.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

