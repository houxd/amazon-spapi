# TransferSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transfer_schedule_id** | **String** | The unique identifier provided by Amazon to the scheduled transfer  | 
**transaction_type** | [**models::TransactionType**](TransactionType.md) |  | 
**transaction_source_account** | Option<[**models::TransactionAccount**](TransactionAccount.md)> |  | [optional]
**transaction_destination_account** | [**models::TransactionAccount**](TransactionAccount.md) |  | 
**transfer_schedule_status** | [**models::TransferScheduleStatus**](TransferScheduleStatus.md) |  | 
**transfer_schedule_information** | [**models::TransferScheduleInformation**](TransferScheduleInformation.md) |  | 
**payment_preference** | Option<[**models::PaymentPreference**](PaymentPreference.md)> |  | [optional]
**transfer_schedule_failures** | [**Vec<models::TransferScheduleFailures>**](TransferScheduleFailures.md) | Collection that holds Transfer Schedules that has been cancelled or failed due to certain reasons.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


