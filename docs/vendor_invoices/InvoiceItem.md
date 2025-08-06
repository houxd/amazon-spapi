# InvoiceItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **i32** | Unique number related to this line item. | 
**amazon_product_identifier** | Option<**String**> | Amazon Standard Identification Number (ASIN) of an item. | [optional]
**vendor_product_identifier** | Option<**String**> | The vendor selected product identifier of the item. Should be the same as was provided in the purchase order. | [optional]
**invoiced_quantity** | [**models::ItemQuantity**](ItemQuantity.md) |  | 
**net_cost** | [**models::Money**](Money.md) |  | 
**net_cost_unit_of_measure** | Option<[**models::NetCostUnitOfMeasure**](NetCostUnitOfMeasure.md)> |  | [optional]
**purchase_order_number** | Option<**String**> | The Amazon purchase order number for this invoiced line item. Formatting Notes: 8-character alpha-numeric code. This value is mandatory only when invoiceType is Invoice, and is not required when invoiceType is CreditNote. | [optional]
**hsn_code** | Option<**String**> | HSN Tax code. The HSN number cannot contain alphabets. | [optional]
**credit_note_details** | Option<[**models::CreditNoteDetails**](CreditNoteDetails.md)> |  | [optional]
**tax_details** | Option<[**Vec<models::TaxDetails>**](TaxDetails.md)> | Individual tax details per line item. | [optional]
**charge_details** | Option<[**Vec<models::ChargeDetails>**](ChargeDetails.md)> | Individual charge details per line item. | [optional]
**allowance_details** | Option<[**Vec<models::AllowanceDetails>**](AllowanceDetails.md)> | Individual allowance details per line item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


