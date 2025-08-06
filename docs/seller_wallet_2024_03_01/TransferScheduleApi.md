# \TransferScheduleApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_transfer_schedule**](TransferScheduleApi.md#create_transfer_schedule) | **POST** /finances/transfers/wallet/2024-03-01/transferSchedules | Create a transfer schedule request from Amazon SW account to another customer provided account
[**delete_schedule_transaction**](TransferScheduleApi.md#delete_schedule_transaction) | **DELETE** /finances/transfers/wallet/2024-03-01/transferSchedules | Delete a transaction request that is scheduled from Amazon SW account to another customer provided account
[**get_transfer_schedule**](TransferScheduleApi.md#get_transfer_schedule) | **GET** /finances/transfers/wallet/2024-03-01/transferSchedules/{transferScheduleId} | Find particular Amazon SW account transfer schedule by Amazon transfer schedule identifier
[**list_transfer_schedules**](TransferScheduleApi.md#list_transfer_schedules) | **GET** /finances/transfers/wallet/2024-03-01/transferSchedules | The API will return all the transfer schedules for a given Amazon SW account
[**update_transfer_schedule**](TransferScheduleApi.md#update_transfer_schedule) | **PUT** /finances/transfers/wallet/2024-03-01/transferSchedules | Update a transfer schedule information. Only fields (i.e; transferScheduleInformation, paymentPreference, transferScheduleStatus) in the request body can be updated.



## create_transfer_schedule

> models::TransferSchedule create_transfer_schedule(dest_account_digital_signature, amount_digital_signature, body)
Create a transfer schedule request from Amazon SW account to another customer provided account

Create a transfer schedule request from a Seller Wallet account to another customer-provided account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dest_account_digital_signature** | **String** | Digital signature for the destination bank account details. | [required] |
**amount_digital_signature** | **String** | Digital signature for the source currency transaction amount. | [required] |
**body** | [**TransferScheduleRequest**](TransferScheduleRequest.md) | Defines the actual payload of the request | [required] |

### Return type

[**models::TransferSchedule**](TransferSchedule.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_schedule_transaction

> models::DeleteTransferSchedule delete_schedule_transaction(transfer_schedule_id)
Delete a transaction request that is scheduled from Amazon SW account to another customer provided account

Delete a transaction request that is scheduled from a Seller Wallet account to another customer-provided account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_schedule_id** | **String** | A unique reference id for a scheduled transfer | [required] |

### Return type

[**models::DeleteTransferSchedule**](DeleteTransferSchedule.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transfer_schedule

> models::TransferSchedule get_transfer_schedule(transfer_schedule_id)
Find particular Amazon SW account transfer schedule by Amazon transfer schedule identifier

Find a particular Seller Wallet account transfer schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_schedule_id** | **String** | Schedule ID of the Amazon SW transfer | [required] |

### Return type

[**models::TransferSchedule**](TransferSchedule.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transfer_schedules

> models::TransferScheduleListing list_transfer_schedules(account_id, next_page_token)
The API will return all the transfer schedules for a given Amazon SW account

Retrieve transfer schedules of a Seller Wallet bank account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the Amazon SW account | [required] |
**next_page_token** | Option<**String**> | Pagination token to retrieve a specific page of results. |  |

### Return type

[**models::TransferScheduleListing**](TransferScheduleListing.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_transfer_schedule

> models::TransferSchedule update_transfer_schedule(dest_account_digital_signature, amount_digital_signature, body)
Update a transfer schedule information. Only fields (i.e; transferScheduleInformation, paymentPreference, transferScheduleStatus) in the request body can be updated.

Returns a transfer belonging to the updated scheduled transfer request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dest_account_digital_signature** | **String** | Digital signature for the destination bank account details. | [required] |
**amount_digital_signature** | **String** | Digital signature for the source currency transaction amount. | [required] |
**body** | [**TransferSchedule**](TransferSchedule.md) | Defines the actual payload of the scheduled transfer request that is to be updated.  | [required] |

### Return type

[**models::TransferSchedule**](TransferSchedule.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

