# SellerDealPaymentEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**deal_id** | Option<**String**> | The unique identifier of the deal. | [optional]
**deal_description** | Option<**String**> | The internal description of the deal. | [optional]
**event_type** | Option<**String**> | The type of event: SellerDealComplete. | [optional]
**fee_type** | Option<**String**> | The type of fee: RunLightningDealFee. | [optional]
**fee_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**tax_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**total_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


