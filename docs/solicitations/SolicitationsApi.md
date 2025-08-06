# \SolicitationsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_product_review_and_seller_feedback_solicitation**](SolicitationsApi.md#create_product_review_and_seller_feedback_solicitation) | **POST** /solicitations/v1/orders/{amazonOrderId}/solicitations/productReviewAndSellerFeedback | 
[**get_solicitation_actions_for_order**](SolicitationsApi.md#get_solicitation_actions_for_order) | **GET** /solicitations/v1/orders/{amazonOrderId} | 



## create_product_review_and_seller_feedback_solicitation

> models::CreateProductReviewAndSellerFeedbackSolicitationResponse create_product_review_and_seller_feedback_solicitation(amazon_order_id, marketplace_ids)


Sends a solicitation to a buyer asking for seller feedback and a product review for the specified order. Send only one productReviewAndSellerFeedback or free form proactive message per order.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which a solicitation is sent. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |

### Return type

[**models::CreateProductReviewAndSellerFeedbackSolicitationResponse**](CreateProductReviewAndSellerFeedbackSolicitationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_solicitation_actions_for_order

> models::GetSolicitationActionsForOrderResponse get_solicitation_actions_for_order(amazon_order_id, marketplace_ids)


Returns a list of solicitation types that are available for an order that you specify. A solicitation type is represented by an actions object, which contains a path and query parameter(s). You can use the path and parameter(s) to call an operation that sends a solicitation. Currently only the productReviewAndSellerFeedbackSolicitation solicitation type is available.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon order identifier. This specifies the order for which you want a list of available solicitation types. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | A marketplace identifier. This specifies the marketplace in which the order was placed. Only one marketplace can be specified. | [required] |

### Return type

[**models::GetSolicitationActionsForOrderResponse**](GetSolicitationActionsForOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

