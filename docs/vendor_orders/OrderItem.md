# OrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Numbering of the item on the purchase order. The first item will be 1, the second 2, and so on. | 
**amazon_product_identifier** | Option<**String**> | Amazon Standard Identification Number (ASIN) of an item. | [optional]
**vendor_product_identifier** | Option<**String**> | The vendor selected product identification of the item. | [optional]
**ordered_quantity** | [**models::ItemQuantity**](ItemQuantity.md) |  | 
**is_back_order_allowed** | **bool** | When true, we will accept backorder confirmations for this item. | 
**net_cost** | Option<[**models::Money**](Money.md)> |  | [optional]
**list_price** | Option<[**models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


