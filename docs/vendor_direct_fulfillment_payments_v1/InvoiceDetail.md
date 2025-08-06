# InvoiceDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_number** | **String** | The unique invoice number. | 
**invoice_date** | **String** | Invoice date. | 
**reference_number** | Option<**String**> | An additional unique reference number used for regulatory or other purposes. | [optional]
**remit_to_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**ship_from_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**bill_to_party** | Option<[**models::PartyIdentification**](PartyIdentification.md)> |  | [optional]
**ship_to_country_code** | Option<**String**> | Ship-to country code. | [optional]
**payment_terms_code** | Option<**String**> | The payment terms for the invoice. | [optional]
**invoice_total** | [**models::Money**](Money.md) |  | 
**tax_totals** | Option<[**Vec<models::TaxDetail>**](TaxDetail.md)> | Individual tax details per line item. | [optional]
**additional_details** | Option<[**Vec<models::AdditionalDetails>**](AdditionalDetails.md)> | Additional details provided by the selling party, for tax-related or other purposes. | [optional]
**charge_details** | Option<[**Vec<models::ChargeDetails>**](ChargeDetails.md)> | Total charge amount details for all line items. | [optional]
**items** | [**Vec<models::InvoiceItem>**](InvoiceItem.md) | Provides the details of the items in this invoice. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


