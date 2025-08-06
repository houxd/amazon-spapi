# PaymentMethodDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_holder_name** | Option<**String**> | The name of the account holder who is registered for the payment method. | [optional]
**payment_method_id** | Option<**String**> | The payment method identifier. | [optional]
**tail** | Option<**String**> | The last three or four digits of the payment method. | [optional]
**expiry_date** | Option<[**models::ExpiryDate**](ExpiryDate.md)> |  | [optional]
**country_code** | Option<**String**> | The two-letter country code in ISO 3166-1 alpha-2 format. For payment methods in the `card` category, the code is for the country where the card was issued. For payment methods in the `bank account` category, the code is for the country where the account is located. | [optional]
**payment_method_type** | Option<[**models::PaymentMethodType**](PaymentMethodType.md)> |  | [optional]
**assignment_type** | Option<[**models::AssignmentType**](AssignmentType.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


