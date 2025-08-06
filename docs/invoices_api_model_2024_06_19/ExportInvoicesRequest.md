# ExportInvoicesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_end** | Option<[**String**](string.md)> | The latest invoice creation date for invoices that you want to include in the response. Dates are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. The default is the time of the request. | [optional]
**date_start** | Option<[**String**](string.md)> | The earliest invoice creation date for invoices that you want to include in the response. Dates are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. The default is 24 hours prior to the time of the request. | [optional]
**external_invoice_id** | Option<**String**> | The external ID of the invoices you want included in the response. | [optional]
**file_format** | Option<[**models::FileFormat**](FileFormat.md)> |  | [optional]
**invoice_type** | Option<**String**> | The marketplace-specific classification of the invoice type. Use the `getInvoicesAttributes` operation to check `invoiceType` options. | [optional]
**marketplace_id** | **String** | The ID of the marketplace from which you want the invoices. | 
**series** | Option<**String**> | The series number of the invoices you want included in the response. | [optional]
**statuses** | Option<**Vec<String>**> | A list of statuses that you can use to filter invoices. Use the `getInvoicesAttributes` operation to check invoice status options.  Min count: 1 | [optional]
**transaction_identifier** | Option<[**models::TransactionIdentifier**](TransactionIdentifier.md)> |  | [optional]
**transaction_type** | Option<**String**> | The marketplace-specific classification of the transaction type for which the invoice was created. Use the `getInvoicesAttributes` operation to check `transactionType` options | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


