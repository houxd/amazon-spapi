# \DefaultApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_payment_methods**](DefaultApi.md#get_payment_methods) | **GET** /finances/transfers/2024-06-01/paymentMethods | 
[**initiate_payout**](DefaultApi.md#initiate_payout) | **POST** /finances/transfers/2024-06-01/payouts | 



## get_payment_methods

> models::GetPaymentMethodsResponse get_payment_methods(marketplace_id, payment_method_types)


Returns the list of payment methods for the seller, which can be filtered by method type.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | .5 | 30 |  The `x-amzn-RateLimit-Limit` response header contains the usage plan rate limits for the operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput might have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | The identifier of the marketplace from which you want to retrieve payment methods. For the list of possible marketplace identifiers, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | [required] |
**payment_method_types** | Option<[**Vec<String>**](String.md)> | A comma-separated list of the payment method types you want to include in the response. |  |

### Return type

[**models::GetPaymentMethodsResponse**](GetPaymentMethodsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_payout

> models::InitiatePayoutResponse initiate_payout(body)


Initiates an on-demand payout to the seller's default deposit method in Seller Central for the given `marketplaceId` and `accountType`, if eligible. You can only initiate one on-demand payout for each marketplace and account type within a 24-hour period.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.017 | 2 |  The `x-amzn-RateLimit-Limit` response header contains the usage plan rate limits for the operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput might have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InitiatePayoutRequest**](InitiatePayoutRequest.md) | The request body for the `initiatePayout` operation. | [required] |

### Return type

[**models::InitiatePayoutResponse**](InitiatePayoutResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

