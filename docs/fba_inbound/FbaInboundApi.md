# \FbaInboundApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_item_eligibility_preview**](FbaInboundApi.md#get_item_eligibility_preview) | **GET** /fba/inbound/v1/eligibility/itemPreview | 



## get_item_eligibility_preview

> models::GetItemEligibilityPreviewResponse get_item_eligibility_preview(asin, program, marketplace_ids)


This operation gets an eligibility preview for an item that you specify. You can specify the type of eligibility preview that you want (INBOUND or COMMINGLING). For INBOUND previews, you can specify the marketplace in which you want to determine the item's eligibility.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asin** | **String** | The ASIN of the item for which you want an eligibility preview. | [required] |
**program** | **String** | The program that you want to check eligibility against. | [required] |
**marketplace_ids** | Option<[**Vec<String>**](String.md)> | The identifier for the marketplace in which you want to determine eligibility. Required only when program=INBOUND. |  |

### Return type

[**models::GetItemEligibilityPreviewResponse**](GetItemEligibilityPreviewResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, ItemEligibilityPreview

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

