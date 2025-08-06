# Money

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency_code** | Option<**String**> | Three digit currency code in ISO 4217 format. String of length 3. | [optional]
**amount** | Option<**String**> | A decimal number with no loss of precision. Useful when precision loss is unacceptable, as with currencies. Follows RFC7159 for number representation. <br>**Pattern** : `^-?(0|([1-9]\\d*))(\\.\\d+)?([eE][+-]?\\d+)?$`. | [optional]
**unit_of_measure** | Option<**String**> | The unit of measure for prices of items sold by weight. If this field is absent, the item is sold by eaches. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


