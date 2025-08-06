# TransactionAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The unique identifier provided by Amazon to identify the account  | [optional]
**bank_account_holder_name** | **String** | BankAccount holder's name  | 
**bank_name** | **String** | The name of the bank  | 
**bank_account_number_format** | [**models::BankAccountNumberFormat**](BankAccountNumberFormat.md) |  | 
**bank_account_number_tail** | Option<**String**> | Last 3 digit of the bank account number  | [optional]
**bank_account_country_code** | Option<**String**> | The two digit country code, in ISO 3166 format. This field is OPTIONAL for transactionSourceAccount object but is MANDATORY field for transactionDestinationAccount  | [optional]
**bank_account_currency** | **String** | The currency code in ISO 4217 format  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


