# FinancialEventGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**financial_event_group_id** | Option<**String**> | A unique identifier for the financial event group. | [optional]
**processing_status** | Option<**String**> | The processing status of the financial event group indicates whether the balance of the financial event group is settled.  Possible values:  * Open  * Closed | [optional]
**fund_transfer_status** | Option<**String**> | The status of the fund transfer. | [optional]
**original_total** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**converted_total** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**fund_transfer_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**trace_id** | Option<**String**> | The trace identifier used by sellers to look up transactions externally. | [optional]
**account_tail** | Option<**String**> | The account tail of the payment instrument. | [optional]
**beginning_balance** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**financial_event_group_start** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**financial_event_group_end** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


