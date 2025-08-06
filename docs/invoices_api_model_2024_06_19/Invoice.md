# Invoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | Option<**String**> | The date and time the invoice is issued. Values are in [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date-time format. | [optional]
**error_code** | Option<**String**> | If the invoice is in an error state, this attribute displays the error code. | [optional]
**external_invoice_id** | Option<**String**> | The invoice identifier that is used by an external party. This is typically the government agency that authorized the invoice. | [optional]
**gov_response** | Option<**String**> | The response message from the government authority when there is an error during invoice issuance. | [optional]
**id** | Option<**String**> | The invoice identifier. | [optional]
**invoice_type** | Option<**String**> | The classification of the invoice type. This varies across marketplaces. Use the `getInvoicesAttributes` operation to check `invoiceType` options. | [optional]
**series** | Option<**String**> | Use this identifier in conjunction with `externalInvoiceId` to identify invoices from the same seller. | [optional]
**status** | Option<**String**> | The invoice status classification. Use the `getInvoicesAttributes` operation to check invoice status options. | [optional]
**transaction_ids** | Option<[**Vec<models::TransactionIdentifier>**](TransactionIdentifier.md)> | List with identifiers for the transactions associated to the invoice. | [optional]
**transaction_type** | Option<**String**> | Classification of the transaction that originated this invoice. Use the `getInvoicesAttributes` operation to check `transactionType` options. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


