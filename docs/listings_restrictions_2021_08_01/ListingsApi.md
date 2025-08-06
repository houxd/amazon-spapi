# \ListingsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_listings_restrictions**](ListingsApi.md#get_listings_restrictions) | **GET** /listings/2021-08-01/restrictions | 



## get_listings_restrictions

> models::RestrictionList get_listings_restrictions(asin, seller_id, marketplace_ids, condition_type, reason_locale)


Returns listing restrictions for an item in the Amazon Catalog.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asin** | **String** | The Amazon Standard Identification Number (ASIN) of the item. | [required] |
**seller_id** | **String** | A selling partner identifier, such as a merchant account. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A comma-delimited list of Amazon marketplace identifiers for the request. | [required] |
**condition_type** | Option<**String**> | The condition used to filter restrictions. |  |
**reason_locale** | Option<**String**> | A locale for reason text localization. When not provided, the default language code of the first marketplace is used. Examples: \"en_US\", \"fr_CA\", \"fr_FR\". Localized messages default to \"en_US\" when a localization is not available in the specified locale. |  |

### Return type

[**models::RestrictionList**](RestrictionList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

