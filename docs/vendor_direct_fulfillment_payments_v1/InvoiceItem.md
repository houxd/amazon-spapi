# InvoiceItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Numbering of the item on the purchase order. The first item will be 1, the second 2, and so on. | 
**buyer_product_identifier** | Option<**String**> | Buyer's standard identification number (ASIN) of an item. | [optional]
**vendor_product_identifier** | Option<**String**> | The vendor selected product identification of the item. | [optional]
**invoiced_quantity** | [**models::ItemQuantity**](ItemQuantity.md) |  | 
**net_cost** | [**models::Money**](Money.md) |  | 
**purchase_order_number** | **String** | The purchase order number for this order. Formatting Notes: 8-character alpha-numeric code. | 
**vendor_order_number** | Option<**String**> | The vendor's order number for this order. | [optional]
**hsn_code** | Option<**String**> | Harmonized System of Nomenclature (HSN) tax code. The HSN number cannot contain alphabets. | [optional]
**tax_details** | Option<[**Vec<models::TaxDetail>**](TaxDetail.md)> | Individual tax details per line item. | [optional]
**charge_details** | Option<[**Vec<models::ChargeDetails>**](ChargeDetails.md)> | Individual charge details per line item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


