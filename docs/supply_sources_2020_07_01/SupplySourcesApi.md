# \SupplySourcesApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**archive_supply_source**](SupplySourcesApi.md#archive_supply_source) | **DELETE** /supplySources/2020-07-01/supplySources/{supplySourceId} | 
[**create_supply_source**](SupplySourcesApi.md#create_supply_source) | **POST** /supplySources/2020-07-01/supplySources | 
[**get_supply_source**](SupplySourcesApi.md#get_supply_source) | **GET** /supplySources/2020-07-01/supplySources/{supplySourceId} | 
[**get_supply_sources**](SupplySourcesApi.md#get_supply_sources) | **GET** /supplySources/2020-07-01/supplySources | 
[**update_supply_source**](SupplySourcesApi.md#update_supply_source) | **PUT** /supplySources/2020-07-01/supplySources/{supplySourceId} | 
[**update_supply_source_status**](SupplySourcesApi.md#update_supply_source_status) | **PUT** /supplySources/2020-07-01/supplySources/{supplySourceId}/status | 



## archive_supply_source

> models::ErrorList archive_supply_source(supply_source_id)


Archive a supply source, making it inactive. Cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supply_source_id** | **String** | The unique identifier of a supply source. | [required] |

### Return type

[**models::ErrorList**](ErrorList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_supply_source

> models::CreateSupplySourceResponse create_supply_source(payload)


Create a new supply source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payload** | [**CreateSupplySourceRequest**](CreateSupplySourceRequest.md) | A request to create a supply source. | [required] |

### Return type

[**models::CreateSupplySourceResponse**](CreateSupplySourceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supply_source

> models::SupplySource get_supply_source(supply_source_id)


Retrieve a supply source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supply_source_id** | **String** | The unique identifier of a supply source. | [required] |

### Return type

[**models::SupplySource**](SupplySource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supply_sources

> models::GetSupplySourcesResponse get_supply_sources(next_page_token, page_size)


The path to retrieve paginated supply sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**next_page_token** | Option<**String**> | The pagination token to retrieve a specific page of results. |  |
**page_size** | Option<**f64**> | The number of supply sources to return per paginated request. |  |[default to 10.0]

### Return type

[**models::GetSupplySourcesResponse**](GetSupplySourcesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_supply_source

> models::ErrorList update_supply_source(supply_source_id, payload)


Update the configuration and capabilities of a supply source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supply_source_id** | **String** | The unique identitier of a supply source. | [required] |
**payload** | Option<[**UpdateSupplySourceRequest**](UpdateSupplySourceRequest.md)> |  |  |

### Return type

[**models::ErrorList**](ErrorList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_supply_source_status

> models::ErrorList update_supply_source_status(supply_source_id, payload)


Update the status of a supply source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supply_source_id** | **String** | The unique identifier of a supply source. | [required] |
**payload** | Option<[**UpdateSupplySourceStatusRequest**](UpdateSupplySourceStatusRequest.md)> |  |  |

### Return type

[**models::ErrorList**](ErrorList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

