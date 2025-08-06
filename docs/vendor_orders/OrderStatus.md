# OrderStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**purchase_order_number** | **String** | The buyer's purchase order number for this order. Formatting Notes: 8-character alpha-numeric code. | 
**purchase_order_status** | **String** | The status of the buyer's purchase order for this order. | 
**purchase_order_date** | **String** | The date the purchase order was placed. Must be in ISO-8601 date/time format. | 
**last_updated_date** | Option<**String**> | The date when the purchase order was last updated. Must be in ISO-8601 date/time format. | [optional]
**selling_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**ship_to_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**item_status** | [**Vec<models::OrderItemStatus>**](OrderItemStatus.md) | Detailed description of items order status. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


