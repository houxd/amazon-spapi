# \FbaInventoryApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_inventory**](FbaInventoryApi.md#add_inventory) | **POST** /fba/inventory/v1/items/inventory | 
[**create_inventory_item**](FbaInventoryApi.md#create_inventory_item) | **POST** /fba/inventory/v1/items | 
[**delete_inventory_item**](FbaInventoryApi.md#delete_inventory_item) | **DELETE** /fba/inventory/v1/items/{sellerSku} | 
[**get_inventory_summaries**](FbaInventoryApi.md#get_inventory_summaries) | **GET** /fba/inventory/v1/summaries | 



## add_inventory

> models::AddInventoryResponse add_inventory(x_amzn_idempotency_token, add_inventory_request_body)


Requests that Amazon add items to the Sandbox Inventory with desired amount of quantity in the sandbox environment. This is a sandbox-only operation and must be directed to a sandbox endpoint. Refer to [Selling Partner API sandbox](https://developer-docs.amazon.com/sp-api/docs/the-selling-partner-api-sandbox) for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_amzn_idempotency_token** | **String** | A unique token/requestId provided with each call to ensure idempotency. | [required] |
**add_inventory_request_body** | [**AddInventoryRequest**](AddInventoryRequest.md) | List of items to add to Sandbox inventory. | [required] |

### Return type

[**models::AddInventoryResponse**](AddInventoryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_inventory_item

> models::CreateInventoryItemResponse create_inventory_item(create_inventory_item_request_body)


Requests that Amazon create product-details in the Sandbox Inventory in the sandbox environment. This is a sandbox-only operation and must be directed to a sandbox endpoint. Refer to [Selling Partner API sandbox](https://developer-docs.amazon.com/sp-api/docs/the-selling-partner-api-sandbox) for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_inventory_item_request_body** | [**CreateInventoryItemRequest**](CreateInventoryItemRequest.md) | CreateInventoryItem Request Body Parameter. | [required] |

### Return type

[**models::CreateInventoryItemResponse**](CreateInventoryItemResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_inventory_item

> models::DeleteInventoryItemResponse delete_inventory_item(seller_sku, marketplace_id)


Requests that Amazon Deletes an item from the Sandbox Inventory in the sandbox environment. This is a sandbox-only operation and must be directed to a sandbox endpoint. Refer to [Selling Partner API sandbox](https://developer-docs.amazon.com/sp-api/docs/the-selling-partner-api-sandbox) for more information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**seller_sku** | **String** | A single seller SKU used for querying the specified seller SKU inventory summaries. | [required] |
**marketplace_id** | **String** | The marketplace ID for the marketplace for which the sellerSku is to be deleted. | [required] |

### Return type

[**models::DeleteInventoryItemResponse**](DeleteInventoryItemResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_inventory_summaries

> models::GetInventorySummariesResponse get_inventory_summaries(granularity_type, granularity_id, marketplace_ids, details, start_date_time, seller_skus, seller_sku, next_token)


Returns a list of inventory summaries. The summaries returned depend on the presence or absence of the startDateTime, sellerSkus and sellerSku parameters:  - All inventory summaries with available details are returned when the startDateTime, sellerSkus and sellerSku parameters are omitted. - When startDateTime is provided, the operation returns inventory summaries that have had changes after the date and time specified. The sellerSkus and sellerSku parameters are ignored. Important: To avoid errors, use both startDateTime and nextToken to get the next page of inventory summaries that have changed after the date and time specified. - When the sellerSkus parameter is provided, the operation returns inventory summaries for only the specified sellerSkus. The sellerSku parameter is ignored. - When the sellerSku parameter is provided, the operation returns inventory summaries for only the specified sellerSku.  Note: The parameters associated with this operation may contain special characters that must be encoded to successfully call the API. To avoid errors with SKUs when encoding URLs, refer to [URL Encoding](https://developer-docs.amazon.com/sp-api/docs/url-encoding).  Usage Plan:  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The x-amzn-RateLimit-Limit response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**granularity_type** | **String** | The granularity type for the inventory aggregation level. | [required] |
**granularity_id** | **String** | The granularity ID for the inventory aggregation level. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | The marketplace ID for the marketplace for which to return inventory summaries. | [required] |
**details** | Option<**bool**> | true to return inventory summaries with additional summarized inventory details and quantities. Otherwise, returns inventory summaries only (default value). |  |[default to false]
**start_date_time** | Option<**String**> | A start date and time in ISO8601 format. If specified, all inventory summaries that have changed since then are returned. You must specify a date and time that is no earlier than 18 months prior to the date and time when you call the API. Note: Changes in inboundWorkingQuantity, inboundShippedQuantity and inboundReceivingQuantity are not detected. |  |
**seller_skus** | Option<[**Vec<String>**](String.md)> | A list of seller SKUs for which to return inventory summaries. You may specify up to 50 SKUs. |  |
**seller_sku** | Option<**String**> | A single seller SKU used for querying the specified seller SKU inventory summaries. |  |
**next_token** | Option<**String**> | String token returned in the response of your previous request. The string token will expire 30 seconds after being created. |  |

### Return type

[**models::GetInventorySummariesResponse**](GetInventorySummariesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

