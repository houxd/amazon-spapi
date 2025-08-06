# ProductAdsPaymentEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**transaction_type** | Option<**String**> | Indicates if the transaction is for a charge or a refund.  Possible values:  * charge - Charge  * refund - Refund | [optional]
**invoice_id** | Option<**String**> | Identifier for the invoice that the transaction appears in. | [optional]
**base_value** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**tax_value** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**transaction_value** | Option<[**models::Currency**](Currency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


