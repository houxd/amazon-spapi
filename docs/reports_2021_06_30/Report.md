# Report

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**marketplace_ids** | Option<**Vec<String>**> | A list of marketplace identifiers for the report. | [optional]
**report_id** | **String** | The identifier for the report. This identifier is unique only in combination with a seller ID. | 
**report_type** | **String** | The report type. Refer to [Report Type Values](https://developer-docs.amazon.com/sp-api/docs/report-type-values) for more information. | 
**data_start_time** | Option<**String**> | The start of a date and time range used for selecting the data to report. | [optional]
**data_end_time** | Option<**String**> | The end of a date and time range used for selecting the data to report. | [optional]
**report_schedule_id** | Option<**String**> | The identifier of the report schedule that created this report (if any). This identifier is unique only in combination with a seller ID. | [optional]
**created_time** | **String** | The date and time when the report was created. | 
**processing_status** | **String** | The processing status of the report. | 
**processing_start_time** | Option<**String**> | The date and time when the report processing started, in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> date time format. | [optional]
**processing_end_time** | Option<**String**> | The date and time when the report processing completed, in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> date time format. | [optional]
**report_document_id** | Option<**String**> | The identifier for the report document. Pass this into the `getReportDocument` operation to get the information you will need to retrieve the report document's contents. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


