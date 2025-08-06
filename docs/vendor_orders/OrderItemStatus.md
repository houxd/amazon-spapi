# OrderItemStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_sequence_number** | **String** | Numbering of the item on the purchase order. The first item will be 1, the second 2, and so on. | 
**buyer_product_identifier** | Option<**String**> | Buyer's Standard Identification Number (ASIN) of an item. | [optional]
**vendor_product_identifier** | Option<**String**> | The vendor selected product identification of the item. | [optional]
**net_cost** | Option<[**models::Money**](Money.md)> |  | [optional]
**list_price** | Option<[**models::Money**](Money.md)> |  | [optional]
**ordered_quantity** | Option<[**models::OrderItemStatusOrderedQuantity**](OrderItemStatus_orderedQuantity.md)> |  | [optional]
**acknowledgement_status** | Option<[**models::OrderItemStatusAcknowledgementStatus**](OrderItemStatus_acknowledgementStatus.md)> |  | [optional]
**receiving_status** | Option<[**models::OrderItemStatusReceivingStatus**](OrderItemStatus_receivingStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


