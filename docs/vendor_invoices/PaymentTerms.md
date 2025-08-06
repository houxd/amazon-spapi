# PaymentTerms

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The payment term type for the invoice. | [optional]
**discount_percent** | Option<**String**> | A decimal number with no loss of precision. Useful when precision loss is unacceptable, as with currencies. Follows RFC7159 for number representation. <br>**Pattern** : `^-?(0|([1-9]\\d*))(\\.\\d+)?([eE][+-]?\\d+)?$`. | [optional]
**discount_due_days** | Option<**f64**> | The number of calendar days from the Base date (Invoice date) until the discount is no longer valid. | [optional]
**net_due_days** | Option<**f64**> | The number of calendar days from the base date (invoice date) until the total amount on the invoice is due. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


