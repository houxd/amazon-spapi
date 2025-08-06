# PackedItems

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | Option<**String**> | Item sequence number for the item. The first item will be 001, the second 002, and so on. This number is used as a reference to refer to this item from the carton or pallet level. | [optional]
**buyer_product_identifier** | Option<**String**> | Buyer Standard Identification Number (ASIN) of an item. | [optional]
**vendor_product_identifier** | Option<**String**> | The vendor selected product identification of the item. Should be the same as was sent in the purchase order. | [optional]
**packed_quantity** | Option<[**models::ItemQuantity**](ItemQuantity.md)> |  | [optional]
**item_details** | Option<[**models::PackageItemDetails**](PackageItemDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


