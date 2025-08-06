# BankAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The unique identifier provided by Amazon to identify the account  | [optional]
**account_holder_name** | **String** | BankAccount holder's name (expected to be Amazon customer)  | 
**bank_account_number_format** | [**models::BankAccountNumberFormat**](BankAccountNumberFormat.md) |  | 
**bank_name** | Option<**String**> | The name of the bank, for all Amazon Seller Wallet account the value will be Amazon Seller Wallet  | [optional]
**bank_account_ownership_type** | [**models::BankAccountOwnershipType**](BankAccountOwnershipType.md) |  | 
**routing_number** | **String** | Routing number for automated clearing house transfers, for all Amazon Seller Wallet account the value will be denoted by nine cosecutive 0's,   | 
**bank_number_format** | [**models::BankNumberFormat**](BankNumberFormat.md) |  | 
**account_country_code** | **String** | The two digit country code, in ISO 3166 format.  | 
**account_currency** | **String** | BankAccount currency code in ISO 4217 format  | 
**bank_account_number_tail** | **String** | Last 3 digit of the bank account number, for all Amazon Seller Wallet account the value will be three consecutive 0's  | 
**bank_account_holder_status** | Option<[**models::BankAccountHolderStatus**](BankAccountHolderStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


