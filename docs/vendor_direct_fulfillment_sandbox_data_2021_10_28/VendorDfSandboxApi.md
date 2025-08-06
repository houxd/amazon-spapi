# \VendorDfSandboxApi

All URIs are relative to *https://sandbox.sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_order_scenarios**](VendorDfSandboxApi.md#generate_order_scenarios) | **POST** /vendor/directFulfillment/sandbox/2021-10-28/orders | 



## generate_order_scenarios

> models::TransactionReference generate_order_scenarios(body)


Submits a request to generate test order data for Vendor Direct Fulfillment API entities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GenerateOrderScenarioRequest**](GenerateOrderScenarioRequest.md) | The request payload containing parameters for generating test order data scenarios. | [required] |

### Return type

[**models::TransactionReference**](TransactionReference.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

