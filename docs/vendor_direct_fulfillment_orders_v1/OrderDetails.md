# OrderDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**customer_order_number** | **String** | The customer order number. | 
**order_date** | **String** | The date the order was placed. This field is expected to be in ISO-8601 date/time format, for example:2018-07-16T23:00:00Z/ 2018-07-16T23:00:00-05:00 /2018-07-16T23:00:00-08:00. If no time zone is specified, UTC should be assumed. | 
**order_status** | Option<**String**> | Current status of the order. | [optional]
**shipment_details** | [**models::ShipmentDetails**](ShipmentDetails.md) |  | 
**tax_total** | Option<[**models::OrderDetailsTaxTotal**](OrderDetails_taxTotal.md)> |  | [optional]
**selling_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**ship_from_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**ship_to_party** | [**models::Address**](Address.md) |  | 
**bill_to_party** | [**models::PartyIdentification**](PartyIdentification.md) |  | 
**items** | [**Vec<models::OrderItem>**](OrderItem.md) | A list of items in this purchase order. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


