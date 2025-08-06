# ChargeRefundEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**reason_code** | Option<**String**> | The reason given for a charge refund.  Example: `SubscriptionFeeCorrection` | [optional]
**reason_code_description** | Option<**String**> | A description of the Reason Code.   Example: `SubscriptionFeeCorrection` | [optional]
**charge_refund_transactions** | Option<[**Vec<models::ChargeRefundTransaction>**](ChargeRefundTransaction.md)> | A list of `ChargeRefund` transactions | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


