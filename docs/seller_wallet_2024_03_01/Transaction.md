# Transaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_id** | **String** | The unique identifier provided by Amazon to the transaction  | 
**transaction_type** | [**models::TransactionType**](TransactionType.md) |  | 
**transaction_status** | [**models::TransactionStatus**](TransactionStatus.md) |  | 
**transaction_request_date** | **String** | The date when the transaction was initiated. | 
**expected_completion_date** | Option<**String**> | Expected completion date of a transaction, for existing active Payees (Trusted Beneficiaries) it will be 24 hours but for new destination bank accounts the value could go up to 5 days  | [optional]
**transaction_actual_completion_date** | Option<**String**> | Transaction completion date  | [optional]
**last_update_date** | **String** | The last update date on the transaction  | 
**requester_name** | Option<**String**> | Amazon SW customer who requested the transaction  | [optional]
**transaction_requester_source** | **String** | The transaction initiation source. This value is either the Amazon portal or PISP name that the customer used to start the transaction. | 
**transaction_description** | **String** | A description of the transaction that the requester provides when they initiate the transaction. | 
**transaction_source_account** | [**models::TransactionAccount**](TransactionAccount.md) |  | 
**transaction_destination_account** | [**models::TransactionAccount**](TransactionAccount.md) |  | 
**transaction_request_amount** | [**models::Currency**](Currency.md) |  | 
**transfer_rate_details** | [**models::TransferRatePreview**](TransferRatePreview.md) |  | 
**transaction_final_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**transaction_failure_reason** | Option<**String**> | Description in case the transaction fails before completion  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


