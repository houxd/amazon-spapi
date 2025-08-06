# \TransferPreviewApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_transfer_preview**](TransferPreviewApi.md#get_transfer_preview) | **GET** /finances/transfers/wallet/2024-03-01/transferPreview | Fetch potential fees that could be applied on a transaction on the basis of the source and destination country currency code



## get_transfer_preview

> models::TransferRatePreview get_transfer_preview(source_country_code, source_currency_code, destination_country_code, destination_currency_code, base_amount)
Fetch potential fees that could be applied on a transaction on the basis of the source and destination country currency code

Returns list of potential fees on a transaction based on the source and destination country currency code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_country_code** | **String** | Represents 2 character country code of source transaction account in ISO 3166 standard format. | [required] |
**source_currency_code** | **String** | Represents 3 letter currency code in ISO 4217 standard format of the source transaction country. | [required] |
**destination_country_code** | **String** | Represents 2 character country code of destination transaction account in ISO 3166 standard format. | [required] |
**destination_currency_code** | **String** | Represents 3 letter currency code in ISO 4217 standard format of the destination transaction country. | [required] |
**base_amount** | **f64** | Represents the base transaction amount without any markup fees, rates that will be used to get the transfer preview. | [required] |

### Return type

[**models::TransferRatePreview**](TransferRatePreview.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

