# CreateFulfillmentOrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU of the item. | 
**seller_fulfillment_order_item_id** | **String** | A fulfillment order item identifier that the seller creates to track fulfillment order items. Used to disambiguate multiple fulfillment items that have the same `SellerSKU`. For example, the seller might assign different `SellerFulfillmentOrderItemId` values to two items in a fulfillment order that share the same `SellerSKU` but have different `GiftMessage` values. | 
**quantity** | **i32** | The item quantity. | 
**gift_message** | Option<**String**> | A message to the gift recipient, if applicable. | [optional]
**displayable_comment** | Option<**String**> | Item-specific text that displays in recipient-facing materials such as the outbound shipment packing slip. | [optional]
**fulfillment_network_sku** | Option<**String**> | Amazon's fulfillment network SKU of the item. | [optional]
**per_unit_declared_value** | Option<[**models::Money**](Money.md)> |  | [optional]
**per_unit_price** | Option<[**models::Money**](Money.md)> |  | [optional]
**per_unit_tax** | Option<[**models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


