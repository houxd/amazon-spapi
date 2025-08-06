# \VendorOrdersApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_purchase_order**](VendorOrdersApi.md#get_purchase_order) | **GET** /vendor/orders/v1/purchaseOrders/{purchaseOrderNumber} | 
[**get_purchase_orders**](VendorOrdersApi.md#get_purchase_orders) | **GET** /vendor/orders/v1/purchaseOrders | 
[**get_purchase_orders_status**](VendorOrdersApi.md#get_purchase_orders_status) | **GET** /vendor/orders/v1/purchaseOrdersStatus | 
[**submit_acknowledgement**](VendorOrdersApi.md#submit_acknowledgement) | **POST** /vendor/orders/v1/acknowledgements | 



## get_purchase_order

> models::GetPurchaseOrderResponse get_purchase_order(purchase_order_number)


Returns a purchase order based on the `purchaseOrderNumber` value that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The purchase order identifier for the order that you want. Formatting Notes: 8-character alpha-numeric code. | [required] |

### Return type

[**models::GetPurchaseOrderResponse**](GetPurchaseOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_purchase_orders

> models::GetPurchaseOrdersResponse get_purchase_orders(limit, created_after, created_before, sort_order, next_token, include_details, changed_after, changed_before, po_item_state, is_po_changed, purchase_order_state, ordering_vendor_code)


Returns a list of purchase orders created or changed during the time frame that you specify. You define the time frame using the `createdAfter`, `createdBefore`, `changedAfter` and `changedBefore` parameters. The date range to search must not be more than 7 days. You can choose to get only the purchase order numbers by setting `includeDetails` to false. You can then use the `getPurchaseOrder` operation to receive details for a specific purchase order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The limit to the number of records returned. Default value is 100 records. |  |
**created_after** | Option<**String**> | Purchase orders that became available after this time will be included in the result. Must be in ISO-8601 date/time format. |  |
**created_before** | Option<**String**> | Purchase orders that became available before this time will be included in the result. Must be in ISO-8601 date/time format. |  |
**sort_order** | Option<**String**> | Sort in ascending or descending order by purchase order creation date. |  |
**next_token** | Option<**String**> | Used for pagination when there is more purchase orders than the specified result size limit. The token value is returned in the previous API call |  |
**include_details** | Option<**bool**> | When true, returns purchase orders with complete details. Otherwise, only purchase order numbers are returned. Default value is true. |  |
**changed_after** | Option<**String**> | Purchase orders that changed after this timestamp will be included in the result. Must be in ISO-8601 date/time format. |  |
**changed_before** | Option<**String**> | Purchase orders that changed before this timestamp will be included in the result. Must be in ISO-8601 date/time format. |  |
**po_item_state** | Option<**String**> | Current state of the purchase order item. If this value is Cancelled, this API will return purchase orders which have one or more items cancelled by Amazon with updated item quantity as zero. |  |
**is_po_changed** | Option<**bool**> | When true, returns purchase orders which were modified after the order was placed. Vendors are required to pull the changed purchase order and fulfill the updated purchase order and not the original one. Default value is false. |  |
**purchase_order_state** | Option<**String**> | Filters purchase orders based on the purchase order state. |  |
**ordering_vendor_code** | Option<**String**> | Filters purchase orders based on the specified ordering vendor code. This value should be same as 'sellingParty.partyId' in the purchase order. If not included in the filter, all purchase orders for all of the vendor codes that exist in the vendor group used to authorize the API client application are returned. |  |

### Return type

[**models::GetPurchaseOrdersResponse**](GetPurchaseOrdersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, payload

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_purchase_orders_status

> models::GetPurchaseOrdersStatusResponse get_purchase_orders_status(limit, sort_order, next_token, created_after, created_before, updated_after, updated_before, purchase_order_number, purchase_order_status, item_confirmation_status, item_receive_status, ordering_vendor_code, ship_to_party_id)


Returns purchase order statuses based on the filters that you specify. Date range to search must not be more than 7 days. You can return a list of purchase order statuses using the available filters, or a single purchase order status by providing the purchase order number.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | The limit to the number of records returned. Default value is 100 records. |  |
**sort_order** | Option<**String**> | Sort in ascending or descending order by purchase order creation date. |  |
**next_token** | Option<**String**> | Used for pagination when there are more purchase orders than the specified result size limit. |  |
**created_after** | Option<**String**> | Purchase orders that became available after this timestamp will be included in the result. Must be in ISO-8601 date/time format. |  |
**created_before** | Option<**String**> | Purchase orders that became available before this timestamp will be included in the result. Must be in ISO-8601 date/time format. |  |
**updated_after** | Option<**String**> | Purchase orders for which the last purchase order update happened after this timestamp will be included in the result. Must be in ISO-8601 date/time format. |  |
**updated_before** | Option<**String**> | Purchase orders for which the last purchase order update happened before this timestamp will be included in the result. Must be in ISO-8601 date/time format. |  |
**purchase_order_number** | Option<**String**> | Provides purchase order status for the specified purchase order number. |  |
**purchase_order_status** | Option<**String**> | Filters purchase orders based on the specified purchase order status. If not included in filter, this will return purchase orders for all statuses. |  |
**item_confirmation_status** | Option<**String**> | Filters purchase orders based on their item confirmation status. If the item confirmation status is not included in the filter, purchase orders for all confirmation statuses are included. |  |
**item_receive_status** | Option<**String**> | Filters purchase orders based on the purchase order's item receive status. If the item receive status is not included in the filter, purchase orders for all receive statuses are included. |  |
**ordering_vendor_code** | Option<**String**> | Filters purchase orders based on the specified ordering vendor code. This value should be same as 'sellingParty.partyId' in the purchase order. If not included in filter, all purchase orders for all the vendor codes that exist in the vendor group used to authorize API client application are returned. |  |
**ship_to_party_id** | Option<**String**> | Filters purchase orders for a specific buyer's Fulfillment Center/warehouse by providing ship to location id here. This value should be same as 'shipToParty.partyId' in the purchase order. If not included in filter, this will return purchase orders for all the buyer's warehouses used for vendor group purchase orders. |  |

### Return type

[**models::GetPurchaseOrdersStatusResponse**](GetPurchaseOrdersStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_acknowledgement

> models::SubmitAcknowledgementResponse submit_acknowledgement(body)


Submits acknowledgements for one or more purchase orders.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SubmitAcknowledgementRequest**](SubmitAcknowledgementRequest.md) | Submits acknowledgements for one or more purchase orders from a vendor. | [required] |

### Return type

[**models::SubmitAcknowledgementResponse**](SubmitAcknowledgementResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

