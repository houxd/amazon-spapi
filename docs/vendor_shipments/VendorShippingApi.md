# \VendorShippingApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_shipment_details**](VendorShippingApi.md#get_shipment_details) | **GET** /vendor/shipping/v1/shipments | GetShipmentDetails
[**get_shipment_labels**](VendorShippingApi.md#get_shipment_labels) | **GET** /vendor/shipping/v1/transportLabels | 
[**submit_shipment_confirmations**](VendorShippingApi.md#submit_shipment_confirmations) | **POST** /vendor/shipping/v1/shipmentConfirmations | SubmitShipmentConfirmations
[**submit_shipments**](VendorShippingApi.md#submit_shipments) | **POST** /vendor/shipping/v1/shipments | SubmitShipments



## get_shipment_details

> models::GetShipmentDetailsResponse get_shipment_details(limit, sort_order, next_token, created_after, created_before, shipment_confirmed_before, shipment_confirmed_after, package_label_created_before, package_label_created_after, shipped_before, shipped_after, estimated_delivery_before, estimated_delivery_after, shipment_delivery_before, shipment_delivery_after, requested_pick_up_before, requested_pick_up_after, scheduled_pick_up_before, scheduled_pick_up_after, current_shipment_status, vendor_shipment_identifier, buyer_reference_number, buyer_warehouse_code, seller_warehouse_code)
GetShipmentDetails

Returns the Details about Shipment, Carrier Details,  status of the shipment, container details and other details related to shipment based on the filter parameters value that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The limit to the number of records returned. Default value is 50 records. |  |
**sort_order** | Option<**String**> | Sort in ascending or descending order by purchase order creation date. |  |
**next_token** | Option<**String**> | Used for pagination when there are more shipments than the specified result size limit. |  |
**created_after** | Option<**String**> | Get Shipment Details that became available after this timestamp will be included in the result. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**created_before** | Option<**String**> | Get Shipment Details that became available before this timestamp will be included in the result. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**shipment_confirmed_before** | Option<**String**> | Get Shipment Details by passing Shipment confirmed create Date Before. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**shipment_confirmed_after** | Option<**String**> | Get Shipment Details by passing Shipment confirmed create Date After. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**package_label_created_before** | Option<**String**> | Get Shipment Details by passing Package label create Date by buyer. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**package_label_created_after** | Option<**String**> | Get Shipment Details by passing Package label create Date After by buyer. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**shipped_before** | Option<**String**> | Get Shipment Details by passing Shipped Date Before. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**shipped_after** | Option<**String**> | Get Shipment Details by passing Shipped Date After. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**estimated_delivery_before** | Option<**String**> | Get Shipment Details by passing Estimated Delivery Date Before. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**estimated_delivery_after** | Option<**String**> | Get Shipment Details by passing Estimated Delivery Date Before. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**shipment_delivery_before** | Option<**String**> | Get Shipment Details by passing Shipment Delivery Date Before. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**shipment_delivery_after** | Option<**String**> | Get Shipment Details by passing Shipment Delivery Date After. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**requested_pick_up_before** | Option<**String**> | Get Shipment Details by passing Before Requested pickup date. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**requested_pick_up_after** | Option<**String**> | Get Shipment Details by passing After Requested pickup date. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**scheduled_pick_up_before** | Option<**String**> | Get Shipment Details by passing Before scheduled pickup date. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**scheduled_pick_up_after** | Option<**String**> | Get Shipment Details by passing After Scheduled pickup date. Must be in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> format. |  |
**current_shipment_status** | Option<**String**> | Get Shipment Details by passing Current shipment status. |  |
**vendor_shipment_identifier** | Option<**String**> | Get Shipment Details by passing Vendor Shipment ID |  |
**buyer_reference_number** | Option<**String**> | Get Shipment Details by passing buyer Reference ID |  |
**buyer_warehouse_code** | Option<**String**> | Get Shipping Details based on buyer warehouse code. This value should be same as 'shipToParty.partyId' in the Shipment. |  |
**seller_warehouse_code** | Option<**String**> | Get Shipping Details based on vendor warehouse code. This value should be same as 'sellingParty.partyId' in the Shipment. |  |

### Return type

[**models::GetShipmentDetailsResponse**](GetShipmentDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shipment_labels

> models::GetShipmentLabels get_shipment_labels(limit, sort_order, next_token, label_created_after, label_created_before, buyer_reference_number, vendor_shipment_identifier, seller_warehouse_code)


Returns small parcel shipment labels based on the filters that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header contains the usage plan rate limits for the operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput might have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The limit to the number of records returned. Default value is 50 records. |  |
**sort_order** | Option<**String**> | Sort the list by shipment label creation date in ascending or descending order. |  |
**next_token** | Option<**String**> | A token that you use to retrieve the next page of results. The response includes `nextToken` when the number of results exceeds the specified `pageSize` value. To get the next page of results, call the operation with this token and include the same arguments as the call that produced the token. To get a complete list, call this operation until `nextToken` is null. Note that this operation can return empty pages. |  |
**label_created_after** | Option<**String**> | Shipment labels created after this time will be included in the result. This field must be in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) datetime format. |  |
**label_created_before** | Option<**String**> | Shipment labels created before this time will be included in the result. This field must be in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) datetime format. |  |
**buyer_reference_number** | Option<**String**> | Get Shipment labels by passing buyer reference number. |  |
**vendor_shipment_identifier** | Option<**String**> | Get Shipment labels by passing vendor shipment identifier. |  |
**seller_warehouse_code** | Option<**String**> | Get Shipping labels based on vendor warehouse code. This value must be same as the `sellingParty.partyId` in the shipment. |  |

### Return type

[**models::GetShipmentLabels**](GetShipmentLabels.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_shipment_confirmations

> models::SubmitShipmentConfirmationsResponse submit_shipment_confirmations(body)
SubmitShipmentConfirmations

Submits one or more shipment confirmations for vendor orders.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SubmitShipmentConfirmationsRequest**](SubmitShipmentConfirmationsRequest.md) | A request to submit shipment confirmation. | [required] |

### Return type

[**models::SubmitShipmentConfirmationsResponse**](SubmitShipmentConfirmationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_shipments

> models::SubmitShipmentConfirmationsResponse submit_shipments(body)
SubmitShipments

Submits one or more shipment request for vendor Orders.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SubmitShipments**](SubmitShipments.md) | A request to submit shipment request. | [required] |

### Return type

[**models::SubmitShipmentConfirmationsResponse**](SubmitShipmentConfirmationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

