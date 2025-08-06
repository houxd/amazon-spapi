# TransactionInitiationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_account_id** | **String** | The unique identifier of the source Amazon SW bank account from where the money needs to be debited  | 
**destination_account_id** | Option<**String**> | Optional field to specify the unique identifier of the destination bank account where the money needs to be deposited  | [optional]
**description** | **String** | Optional field to specify description for the transaction  | 
**destination_transaction_instrument** | [**models::TransactionInstrumentDetails**](TransactionInstrumentDetails.md) |  | 
**destination_account_holder_address** | Option<[**models::AccountHolderAddress**](AccountHolderAddress.md)> |  | [optional]
**source_amount** | [**models::Currency**](Currency.md) |  | 
**transfer_rate_details** | Option<[**models::TransferRatePreview**](TransferRatePreview.md)> |  | [optional]
**request_time** | **String** | The transaction initiation request time in date-time format  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


