# OrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Numbering of the item on the purchase order. The first item will be 1, the second 2, and so on. | 
**buyer_product_identifier** | Option<**String**> | Buyer's standard identification number (ASIN) of an item. | [optional]
**vendor_product_identifier** | Option<**String**> | The vendor selected product identification of the item. | [optional]
**title** | Option<**String**> | Title for the item. | [optional]
**ordered_quantity** | [**models::ItemQuantity**](ItemQuantity.md) |  | 
**scheduled_delivery_shipment** | Option<[**models::ScheduledDeliveryShipment**](ScheduledDeliveryShipment.md)> |  | [optional]
**gift_details** | Option<[**models::GiftDetails**](GiftDetails.md)> |  | [optional]
**net_price** | [**models::Money**](Money.md) |  | 
**tax_details** | Option<[**models::OrderItemTaxDetails**](OrderItem_taxDetails.md)> |  | [optional]
**total_price** | Option<[**models::Money**](Money.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


