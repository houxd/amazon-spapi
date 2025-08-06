# PackedItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **i32** | Item Sequence Number for the item. This must be the same value as sent in the order for a given item. | 
**buyer_product_identifier** | Option<**String**> | Buyer's Standard Identification Number (ASIN) of an item. Either buyerProductIdentifier or vendorProductIdentifier is required. | [optional]
**vendor_product_identifier** | Option<**String**> | The vendor selected product identification of the item. Should be the same as was sent in the Purchase Order, like SKU Number. | [optional]
**packed_quantity** | [**models::ItemQuantity**](ItemQuantity.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


