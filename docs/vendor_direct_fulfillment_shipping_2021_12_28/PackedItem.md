# PackedItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **i32** | The sequence number of the item. The number must be the same as the order number of the item. | 
**buyer_product_identifier** | Option<**String**> | The buyer's Amazon Standard Identification Number (ASIN) of an item. Either `buyerProductIdentifier` or `vendorProductIdentifier` is required. | [optional]
**piece_number** | Option<**i32**> | The piece number of the item in this container. This is required when the item is split across different containers. | [optional]
**vendor_product_identifier** | Option<**String**> | An item's product identifier, which the vendor selects. This identifier should be the same as the identifier, such as a SKU, in the purchase order. | [optional]
**packed_quantity** | [**models::ItemQuantity**](ItemQuantity.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


