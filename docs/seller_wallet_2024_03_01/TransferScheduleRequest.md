# TransferScheduleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_account_id** | **String** | The unique identifier of the source Amazon SW bank account from where the money needs to be debited  | 
**source_currency_code** | **String** | Represents 3 letter currency code in ISO 4217 standard format of the source payment method country  | 
**destination_account_id** | **String** | Optional field to specify the unique identifier of the destination bank account where the money needs to be deposited  | 
**destination_transaction_instrument** | [**models::TransactionInstrumentDetails**](TransactionInstrumentDetails.md) |  | 
**transaction_type** | [**models::TransactionType**](TransactionType.md) |  | 
**transfer_schedule_information** | [**models::TransferScheduleInformation**](TransferScheduleInformation.md) |  | 
**payment_preference** | [**models::PaymentPreference**](PaymentPreference.md) |  | 
**transfer_schedule_status** | Option<[**models::TransferScheduleStatus**](TransferScheduleStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


