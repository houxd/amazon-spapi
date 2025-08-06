# \CustomerFeedbackApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_browse_node_return_topics**](CustomerFeedbackApi.md#get_browse_node_return_topics) | **GET** /customerFeedback/2024-06-01/browseNodes/{browseNodeId}/returns/topics | 
[**get_browse_node_return_trends**](CustomerFeedbackApi.md#get_browse_node_return_trends) | **GET** /customerFeedback/2024-06-01/browseNodes/{browseNodeId}/returns/trends | 
[**get_browse_node_review_topics**](CustomerFeedbackApi.md#get_browse_node_review_topics) | **GET** /customerFeedback/2024-06-01/browseNodes/{browseNodeId}/reviews/topics | 
[**get_browse_node_review_trends**](CustomerFeedbackApi.md#get_browse_node_review_trends) | **GET** /customerFeedback/2024-06-01/browseNodes/{browseNodeId}/reviews/trends | 
[**get_item_browse_node**](CustomerFeedbackApi.md#get_item_browse_node) | **GET** /customerFeedback/2024-06-01/items/{asin}/browseNode | 
[**get_item_review_topics**](CustomerFeedbackApi.md#get_item_review_topics) | **GET** /customerFeedback/2024-06-01/items/{asin}/reviews/topics | 
[**get_item_review_trends**](CustomerFeedbackApi.md#get_item_review_trends) | **GET** /customerFeedback/2024-06-01/items/{asin}/reviews/trends | 



## get_browse_node_return_topics

> models::BrowseNodeReturnTopicsResponse get_browse_node_return_topics(browse_node_id, marketplace_id)


Retrieve the topics that customers mention when they return items in a browse node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**browse_node_id** | **String** | A browse node ID is a unique identifier for a browse node. A browse node is a named location in a browse tree that is used for navigation, product classification, and website content. | [required] |
**marketplace_id** | **String** | The MarketplaceId is the globally unique identifier of a marketplace, you can refer to the marketplaceId here : https://developer-docs.amazon.com/sp-api/docs/marketplace-ids. | [required] |

### Return type

[**models::BrowseNodeReturnTopicsResponse**](BrowseNodeReturnTopicsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_browse_node_return_trends

> models::BrowseNodeReturnTrendsResponse get_browse_node_return_trends(browse_node_id, marketplace_id)


Retrieve the trends of topics that customers mention when they return items in a browse node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**browse_node_id** | **String** | A browse node ID is a unique identifier of a browse node. A browse node is a named location in a browse tree that is used for navigation, product classification, and website content. | [required] |
**marketplace_id** | **String** | The MarketplaceId is the globally unique identifier of a marketplace, you can refer to the marketplaceId here : https://developer-docs.amazon.com/sp-api/docs/marketplace-ids. | [required] |

### Return type

[**models::BrowseNodeReturnTrendsResponse**](BrowseNodeReturnTrendsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_browse_node_review_topics

> models::BrowseNodeReviewTopicsResponse get_browse_node_review_topics(browse_node_id, marketplace_id, sort_by)


Retrieve a browse node's ten most positive and ten most negative review topics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**browse_node_id** | **String** | The ID of a browse node. A browse node is a named location in a browse tree that is used for navigation, product classification, and website content. | [required] |
**marketplace_id** | **String** | The MarketplaceId is the globally unique identifier of a marketplace, you can refer to the marketplaceId here : https://developer-docs.amazon.com/sp-api/docs/marketplace-ids. | [required] |
**sort_by** | **String** | The metric by which to sort the data in the response. | [required] |

### Return type

[**models::BrowseNodeReviewTopicsResponse**](BrowseNodeReviewTopicsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_browse_node_review_trends

> models::BrowseNodeReviewTrendsResponse get_browse_node_review_trends(browse_node_id, marketplace_id)


Retrieve the positive and negative review trends of items in a browse node for the past six months.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**browse_node_id** | **String** | A browse node ID is a unique identifier of a browse node. A browse node is a named location in a browse tree that is used for navigation, product classification, and website content. | [required] |
**marketplace_id** | **String** | The marketplace ID is the globally unique identifier of a marketplace. For more information, refer to [Marketplace IDs](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids). | [required] |

### Return type

[**models::BrowseNodeReviewTrendsResponse**](BrowseNodeReviewTrendsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_browse_node

> models::BrowseNodeResponse get_item_browse_node(asin, marketplace_id)


This API returns the associated browse node of the requested ASIN. A browse node is a location in a browse tree that is used for navigation, product classification, and website content on the Amazon retail website.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asin** | **String** | The Amazon Standard Identification Number (ASIN) is the unique identifier of a product within a marketplace. | [required] |
**marketplace_id** | **String** | The MarketplaceId is the globally unique identifier of a marketplace, you can refer to the marketplaceId here : https://developer-docs.amazon.com/sp-api/docs/marketplace-ids. | [required] |

### Return type

[**models::BrowseNodeResponse**](BrowseNodeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_review_topics

> models::ItemReviewTopicsResponse get_item_review_topics(asin, marketplace_id, sort_by)


Retrieve an item's ten most positive and ten most negative review topics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asin** | **String** | The Amazon Standard Identification Number (ASIN) is the unique identifier of a product within a marketplace. The value must be a child ASIN. | [required] |
**marketplace_id** | **String** | The MarketplaceId is the globally unique identifier of a marketplace, you can refer to the marketplaceId here : https://developer-docs.amazon.com/sp-api/docs/marketplace-ids. | [required] |
**sort_by** | **String** | The metric by which to sort data in the response. | [required] |

### Return type

[**models::ItemReviewTopicsResponse**](ItemReviewTopicsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_review_trends

> models::ItemReviewTrendsResponse get_item_review_trends(asin, marketplace_id)


Retrieve an item's positive and negative review trends for the past six months.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asin** | **String** | The Amazon Standard Identification Number (ASIN) is the unique identifier of a product within a marketplace. This API takes child ASIN as an input. | [required] |
**marketplace_id** | **String** | The MarketplaceId is the globally unique identifier of a marketplace, you can refer to the marketplaceId here : https://developer-docs.amazon.com/sp-api/docs/marketplace-ids. | [required] |

### Return type

[**models::ItemReviewTrendsResponse**](ItemReviewTrendsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

