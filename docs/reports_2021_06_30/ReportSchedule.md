# ReportSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**report_schedule_id** | **String** | The identifier for the report schedule. This identifier is unique only in combination with a seller ID. | 
**report_type** | **String** | The report type. Refer to [Report Type Values](https://developer-docs.amazon.com/sp-api/docs/report-type-values) for more information. | 
**marketplace_ids** | Option<**Vec<String>**> | A list of marketplace identifiers. The report document's contents will contain data for all of the specified marketplaces, unless the report type indicates otherwise. | [optional]
**report_options** | Option<**std::collections::HashMap<String, String>**> | Additional information passed to reports. This varies by report type. | [optional]
**period** | **String** | An <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> period value that indicates how often a report should be created. | 
**next_report_creation_time** | Option<**String**> | The date and time when the schedule will create its next report, in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> date time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


