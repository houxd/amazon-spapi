# Export

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_message** | Option<**String**> | When the export generation fails, this attribute contains a description of the error. | [optional]
**export_id** | Option<**String**> | The export identifier. | [optional]
**generate_export_finished_at** | Option<**String**> | The date and time when the export generation finished. Vales are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. | [optional]
**generate_export_started_at** | Option<**String**> | The date and time when the export generation started. Values are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. | [optional]
**invoices_document_ids** | Option<**Vec<String>**> | The identifier for the export documents. To get the information required to retrieve the export document's contents, pass each ID in the `getInvoicesDocument` operation.  This list is empty until the status is `DONE`. | [optional]
**status** | Option<[**models::ExportStatus**](ExportStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


