# \ShippingApi

All URIs are relative to *https://sellingpartnerapi-eu.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_shipment**](ShippingApi.md#cancel_shipment) | **PUT** /shipping/v2/shipments/{shipmentId}/cancel | 
[**create_claim**](ShippingApi.md#create_claim) | **POST** /shipping/v2/claims | 
[**direct_purchase_shipment**](ShippingApi.md#direct_purchase_shipment) | **POST** /shipping/v2/shipments/directPurchase | 
[**generate_collection_form**](ShippingApi.md#generate_collection_form) | **POST** /shipping/v2/collectionForms | 
[**get_access_points**](ShippingApi.md#get_access_points) | **GET** /shipping/v2/accessPoints | 
[**get_additional_inputs**](ShippingApi.md#get_additional_inputs) | **GET** /shipping/v2/shipments/additionalInputs/schema | 
[**get_carrier_account_form_inputs**](ShippingApi.md#get_carrier_account_form_inputs) | **GET** /shipping/v2/carrierAccountFormInputs | 
[**get_carrier_accounts**](ShippingApi.md#get_carrier_accounts) | **PUT** /shipping/v2/carrierAccounts | 
[**get_collection_form**](ShippingApi.md#get_collection_form) | **GET** /shipping/v2/collectionForms/{collectionFormId} | 
[**get_collection_form_history**](ShippingApi.md#get_collection_form_history) | **PUT** /shipping/v2/collectionForms/history | 
[**get_rates**](ShippingApi.md#get_rates) | **POST** /shipping/v2/shipments/rates | 
[**get_shipment_documents**](ShippingApi.md#get_shipment_documents) | **GET** /shipping/v2/shipments/{shipmentId}/documents | 
[**get_tracking**](ShippingApi.md#get_tracking) | **GET** /shipping/v2/tracking | 
[**get_unmanifested_shipments**](ShippingApi.md#get_unmanifested_shipments) | **PUT** /shipping/v2/unmanifestedShipments | 
[**link_carrier_account**](ShippingApi.md#link_carrier_account) | **PUT** /shipping/v2/carrierAccounts/{carrierId} | 
[**link_carrier_account_0**](ShippingApi.md#link_carrier_account_0) | **POST** /shipping/v2/carrierAccounts/{carrierId} | 
[**one_click_shipment**](ShippingApi.md#one_click_shipment) | **POST** /shipping/v2/oneClickShipment | 
[**purchase_shipment**](ShippingApi.md#purchase_shipment) | **POST** /shipping/v2/shipments | 
[**submit_ndr_feedback**](ShippingApi.md#submit_ndr_feedback) | **POST** /shipping/v2/ndrFeedback | 
[**unlink_carrier_account**](ShippingApi.md#unlink_carrier_account) | **PUT** /shipping/v2/carrierAccounts/{carrierId}/unlink | 



## cancel_shipment

> models::CancelShipmentResponse cancel_shipment(shipment_id, x_amzn_shipping_business_id)


Cancels a purchased shipment. Returns an empty object if the shipment is successfully cancelled.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment_id** | **String** | The shipment identifier originally returned by the purchaseShipment operation. | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::CancelShipmentResponse**](CancelShipmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_claim

> models::CreateClaimResponse create_claim(body, x_amzn_shipping_business_id)


This API will be used to create claim for single eligible shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateClaimRequest**](CreateClaimRequest.md) | Request body for the createClaim operation | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::CreateClaimResponse**](CreateClaimResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## direct_purchase_shipment

> models::DirectPurchaseResponse direct_purchase_shipment(body, x_amzn_idempotency_key, locale, x_amzn_shipping_business_id)


Purchases the shipping service for a shipment using the best fit service offering. Returns purchase related details and documents.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DirectPurchaseRequest**](DirectPurchaseRequest.md) | DirectPurchaseRequest body | [required] |
**x_amzn_idempotency_key** | Option<**String**> | A unique value which the server uses to recognize subsequent retries of the same request. |  |
**locale** | Option<**String**> | The IETF Language Tag. Note that this only supports the primary language subtag with one secondary language subtag (i.e. en-US, fr-CA). The secondary language subtag is almost always a regional designation. This does not support additional subtags beyond the primary and secondary language subtags.  |  |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::DirectPurchaseResponse**](DirectPurchaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_collection_form

> models::GenerateCollectionFormResponse generate_collection_form(body, x_amzn_idempotency_key, x_amzn_shipping_business_id)


This API  Call to generate the collection form.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GenerateCollectionFormRequest**](GenerateCollectionFormRequest.md) | GenerateCollectionFormRequest body | [required] |
**x_amzn_idempotency_key** | Option<**String**> | A unique value which the server uses to recognize subsequent retries of the same request. |  |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GenerateCollectionFormResponse**](GenerateCollectionFormResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_access_points

> models::GetAccessPointsResponse get_access_points(access_point_types, country_code, postal_code, x_amzn_shipping_business_id)


Returns a list of access points in proximity of input postal code.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_point_types** | [**Vec<String>**](String.md) | Access point types | [required] |
**country_code** | **String** | Country code for access point | [required] |
**postal_code** | **String** | postal code for access point | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GetAccessPointsResponse**](GetAccessPointsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_additional_inputs

> models::GetAdditionalInputsResponse get_additional_inputs(request_token, rate_id, x_amzn_shipping_business_id)


Returns the JSON schema to use for providing additional inputs when needed to purchase a shipping offering. Call the getAdditionalInputs operation when the response to a previous call to the getRates operation indicates that additional inputs are required for the rate (shipping offering) that you want to purchase.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_token** | **String** | The request token returned in the response to the getRates operation. | [required] |
**rate_id** | **String** | The rate identifier for the shipping offering (rate) returned in the response to the getRates operation. | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GetAdditionalInputsResponse**](GetAdditionalInputsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_carrier_account_form_inputs

> models::GetCarrierAccountFormInputsResponse get_carrier_account_form_inputs(x_amzn_shipping_business_id)


This API will return a list of input schema required to register a shipper account with the carrier.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GetCarrierAccountFormInputsResponse**](GetCarrierAccountFormInputsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_carrier_accounts

> models::GetCarrierAccountsResponse get_carrier_accounts(body, x_amzn_shipping_business_id)


This API will return Get all carrier accounts for a merchant.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GetCarrierAccountsRequest**](GetCarrierAccountsRequest.md) | GetCarrierAccountsRequest body | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GetCarrierAccountsResponse**](GetCarrierAccountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_form

> models::GetCollectionFormResponse get_collection_form(collection_form_id, x_amzn_shipping_business_id)


This API reprint a collection form.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_form_id** | **String** | collection form Id to reprint a collection. | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GetCollectionFormResponse**](GetCollectionFormResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_form_history

> models::GetCollectionFormHistoryResponse get_collection_form_history(body, x_amzn_shipping_business_id)


This API Call to get the history of the previously generated collection forms.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GetCollectionFormHistoryRequest**](GetCollectionFormHistoryRequest.md) | GetCollectionFormHistoryRequest body | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GetCollectionFormHistoryResponse**](GetCollectionFormHistoryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rates

> models::GetRatesResponse get_rates(body, x_amzn_shipping_business_id)


Returns the available shipping service offerings.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GetRatesRequest**](GetRatesRequest.md) | GetRatesRequest body | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GetRatesResponse**](GetRatesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shipment_documents

> models::GetShipmentDocumentsResponse get_shipment_documents(shipment_id, package_client_reference_id, format, dpi, x_amzn_shipping_business_id)


Returns the shipping documents associated with a package in a shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shipment_id** | **String** | The shipment identifier originally returned by the purchaseShipment operation. | [required] |
**package_client_reference_id** | **String** | The package client reference identifier originally provided in the request body parameter for the getRates operation. | [required] |
**format** | Option<**String**> | The file format of the document. Must be one of the supported formats returned by the getRates operation. |  |
**dpi** | Option<**f64**> | The resolution of the document (for example, 300 means 300 dots per inch). Must be one of the supported resolutions returned in the response to the getRates operation. |  |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GetShipmentDocumentsResponse**](GetShipmentDocumentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tracking

> models::GetTrackingResponse get_tracking(tracking_id, carrier_id, x_amzn_shipping_business_id)


Returns tracking information for a purchased shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tracking_id** | **String** | A carrier-generated tracking identifier originally returned by the purchaseShipment operation. | [required] |
**carrier_id** | **String** | A carrier identifier originally returned by the getRates operation for the selected rate. | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GetTrackingResponse**](GetTrackingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unmanifested_shipments

> models::GetUnmanifestedShipmentsResponse get_unmanifested_shipments(body, x_amzn_shipping_business_id)


This API Get all unmanifested carriers with shipment locations. Any locations which has unmanifested shipments         with an eligible carrier for manifesting shall be returned.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GetUnmanifestedShipmentsRequest**](GetUnmanifestedShipmentsRequest.md) | GetUmanifestedShipmentsRequest body | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::GetUnmanifestedShipmentsResponse**](GetUnmanifestedShipmentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_carrier_account

> models::LinkCarrierAccountResponse link_carrier_account(carrier_id, body, x_amzn_shipping_business_id)


This API associates/links the specified carrier account with the merchant.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**carrier_id** | **String** | An identifier for the carrier with which the seller's account is being linked. | [required] |
**body** | [**LinkCarrierAccountRequest**](LinkCarrierAccountRequest.md) | LinkCarrierAccountRequest body | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::LinkCarrierAccountResponse**](LinkCarrierAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## link_carrier_account_0

> models::LinkCarrierAccountResponse link_carrier_account_0(carrier_id, body, x_amzn_shipping_business_id)


This API associates/links the specified carrier account with the merchant.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**carrier_id** | **String** | An identifier for the carrier with which the seller's account is being linked. | [required] |
**body** | [**LinkCarrierAccountRequest**](LinkCarrierAccountRequest.md) | LinkCarrierAccountRequest body | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::LinkCarrierAccountResponse**](LinkCarrierAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## one_click_shipment

> models::OneClickShipmentResponse one_click_shipment(body, x_amzn_shipping_business_id)


Purchases a shipping service identifier and returns purchase-related details and documents.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OneClickShipmentRequest**](OneClickShipmentRequest.md) | OneClickShipmentRequest body | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::OneClickShipmentResponse**](OneClickShipmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## purchase_shipment

> models::PurchaseShipmentResponse purchase_shipment(body, x_amzn_idempotency_key, x_amzn_shipping_business_id)


Purchases a shipping service and returns purchase related details and documents.  Note: You must complete the purchase within 10 minutes of rate creation by the shipping service provider. If you make the request after the 10 minutes have expired, you will receive an error response with the error code equal to \"TOKEN_EXPIRED\". If you receive this error response, you must get the rates for the shipment again.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PurchaseShipmentRequest**](PurchaseShipmentRequest.md) | PurchaseShipmentRequest body | [required] |
**x_amzn_idempotency_key** | Option<**String**> | A unique value which the server uses to recognize subsequent retries of the same request. |  |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::PurchaseShipmentResponse**](PurchaseShipmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_ndr_feedback

> submit_ndr_feedback(body, x_amzn_shipping_business_id)


This API submits the NDR (Non-delivery Report) Feedback for any eligible shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SubmitNdrFeedbackRequest**](SubmitNdrFeedbackRequest.md) | Request body for ndrFeedback operation | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_carrier_account

> models::UnlinkCarrierAccountResponse unlink_carrier_account(carrier_id, body, x_amzn_shipping_business_id)


This API Unlink the specified carrier account with the merchant.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 80 | 100 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values then those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**carrier_id** | **String** | carrier Id to unlink with merchant. | [required] |
**body** | [**UnlinkCarrierAccountRequest**](UnlinkCarrierAccountRequest.md) | UnlinkCarrierAccountRequest body | [required] |
**x_amzn_shipping_business_id** | Option<**String**> | Amazon shipping business to assume for this request. The default is AmazonShipping_UK. |  |

### Return type

[**models::UnlinkCarrierAccountResponse**](UnlinkCarrierAccountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

