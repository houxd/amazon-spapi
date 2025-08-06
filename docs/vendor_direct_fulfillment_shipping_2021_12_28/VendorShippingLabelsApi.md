# \VendorShippingLabelsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_shipping_labels**](VendorShippingLabelsApi.md#create_shipping_labels) | **POST** /vendor/directFulfillment/shipping/2021-12-28/shippingLabels/{purchaseOrderNumber} | createShippingLabels
[**get_shipping_label**](VendorShippingLabelsApi.md#get_shipping_label) | **GET** /vendor/directFulfillment/shipping/2021-12-28/shippingLabels/{purchaseOrderNumber} | getShippingLabel
[**get_shipping_labels**](VendorShippingLabelsApi.md#get_shipping_labels) | **GET** /vendor/directFulfillment/shipping/2021-12-28/shippingLabels | getShippingLabels
[**submit_shipping_label_request**](VendorShippingLabelsApi.md#submit_shipping_label_request) | **POST** /vendor/directFulfillment/shipping/2021-12-28/shippingLabels | submitShippingLabelRequest



## create_shipping_labels

> models::ShippingLabel create_shipping_labels(purchase_order_number, body)
createShippingLabels

Creates shipping labels for a purchase order and returns the labels.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values then those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The purchase order number for which you want to return the shipping labels. It should be the same number as the `purchaseOrderNumber` in the order. | [required] |
**body** | [**CreateShippingLabelsRequest**](CreateShippingLabelsRequest.md) | The request payload that contains the parameters for creating shipping labels. | [required] |

### Return type

[**models::ShippingLabel**](ShippingLabel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shipping_label

> models::ShippingLabel get_shipping_label(purchase_order_number)
getShippingLabel

Returns a shipping label for the `purchaseOrderNumber` that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values then those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The purchase order number for which you want to return the shipping label. It should be the same `purchaseOrderNumber` that you received in the order. | [required] |

### Return type

[**models::ShippingLabel**](ShippingLabel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shipping_labels

> models::ShippingLabelList get_shipping_labels(created_after, created_before, ship_from_party_id, limit, sort_order, next_token)
getShippingLabels

Returns a list of shipping labels created during the time frame that you specify. Use the `createdAfter` and `createdBefore` parameters to define the time frame. You must use both of these parameters. The date range to search must not be more than seven days.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values then those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created_after** | **String** | Shipping labels that became available after this date and time will be included in the result. Values are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. | [required] |
**created_before** | **String** | Shipping labels that became available before this date and time will be included in the result. Values are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. | [required] |
**ship_from_party_id** | Option<**String**> | The vendor `warehouseId` for order fulfillment. If not specified, the result contains orders for all warehouses. |  |
**limit** | Option<**i32**> | The limit to the number of records returned. |  |
**sort_order** | Option<**String**> | The sort order creation date. You can choose between ascending (`ASC`) or descending (`DESC`) sort order. |  |[default to ASC]
**next_token** | Option<**String**> | Used for pagination when there are more ship labels than the specified result size limit. The token value is returned in the previous API call. |  |

### Return type

[**models::ShippingLabelList**](ShippingLabelList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, pagination, shippingLabels

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_shipping_label_request

> models::TransactionReference submit_shipping_label_request(body)
submitShippingLabelRequest

Creates a shipping label for a purchase order and returns a `transactionId` for reference.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values then those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SubmitShippingLabelsRequest**](SubmitShippingLabelsRequest.md) | The request body that contains the shipping labels data. | [required] |

### Return type

[**models::TransactionReference**](TransactionReference.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

