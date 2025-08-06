# \VehiclesApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_vehicles**](VehiclesApi.md#get_vehicles) | **GET** /catalog/2024-11-01/automotive/vehicles | 



## get_vehicles

> models::VehiclesResponse get_vehicles(marketplace_id, vehicle_type, page_token, updated_after)


Get the latest collection of vehicles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | An identifier for the marketplace in which the resource operates. | [required] |
**vehicle_type** | **String** | An identifier for vehicle type. | [required] |
**page_token** | Option<**String**> | A token to fetch a certain page when there are multiple pages worth of results. |  |
**updated_after** | Option<**String**> | Date in ISO 8601 format, if provided only vehicles which are modified/added to Amazon's catalog after this date will be returned. |  |

### Return type

[**models::VehiclesResponse**](VehiclesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

