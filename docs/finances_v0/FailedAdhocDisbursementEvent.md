# FailedAdhocDisbursementEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**funds_transfers_type** | Option<**String**> | The type of fund transfer.   Example \"Refund\" | [optional]
**transfer_id** | Option<**String**> | The transfer identifier. | [optional]
**disbursement_id** | Option<**String**> | The disbursement identifier. | [optional]
**payment_disbursement_type** | Option<**String**> | The type of payment for disbursement.   Example `CREDIT_CARD` | [optional]
**status** | Option<**String**> | The status of the failed `AdhocDisbursement`.   Example `HARD_DECLINED` | [optional]
**transfer_amount** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


