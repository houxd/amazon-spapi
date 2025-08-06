# Invoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_type** | **String** | Identifies the type of invoice. | 
**id** | **String** | Unique number relating to the charges defined in this document. This will be invoice number if the document type is Invoice or CreditNote number if the document type is Credit Note. Failure to provide this reference will result in a rejection. | 
**reference_number** | Option<**String**> | An additional unique reference number used for regulatory or other purposes. | [optional]
**date** | **String** | Defines a date and time according to ISO8601. | 
**remit_to_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**ship_to_party** | Option<[**models::PartyIdentification**](PartyIdentification.md)> |  | [optional]
**ship_from_party** | Option<[**models::PartyIdentification**](PartyIdentification.md)> |  | [optional]
**bill_to_party** | Option<[**models::PartyIdentification**](PartyIdentification.md)> |  | [optional]
**payment_terms** | Option<[**models::PaymentTerms**](PaymentTerms.md)> |  | [optional]
**invoice_total** | [**models::Money**](Money.md) |  | 
**tax_details** | Option<[**Vec<models::TaxDetails>**](TaxDetails.md)> | Total tax amount details for all line items. | [optional]
**additional_details** | Option<[**Vec<models::AdditionalDetails>**](AdditionalDetails.md)> | Additional details provided by the selling party, for tax related or other purposes. | [optional]
**charge_details** | Option<[**Vec<models::ChargeDetails>**](ChargeDetails.md)> | Total charge amount details for all line items. | [optional]
**allowance_details** | Option<[**Vec<models::AllowanceDetails>**](AllowanceDetails.md)> | Total allowance amount details for all line items. | [optional]
**items** | Option<[**Vec<models::InvoiceItem>**](InvoiceItem.md)> | The list of invoice items. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


