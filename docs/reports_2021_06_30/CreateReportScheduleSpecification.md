# CreateReportScheduleSpecification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**report_type** | **String** | The report type. Refer to [Report Type Values](https://developer-docs.amazon.com/sp-api/docs/report-type-values) for more information. | 
**marketplace_ids** | **Vec<String>** | A list of marketplace identifiers for the report schedule. | 
**report_options** | Option<**std::collections::HashMap<String, String>**> | Additional information passed to reports. This varies by report type. | [optional]
**period** | **String** | One of a set of predefined <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> periods that specifies how often a report should be created. | 
**next_report_creation_time** | Option<**String**> | The date and time when the schedule will create its next report, in <a href='https://developer-docs.amazon.com/sp-api/docs/iso-8601'>ISO 8601</a> date time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


