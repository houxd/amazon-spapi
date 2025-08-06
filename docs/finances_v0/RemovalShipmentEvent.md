# RemovalShipmentEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**merchant_order_id** | Option<**String**> | The merchant removal orderId. | [optional]
**order_id** | Option<**String**> | The identifier for the removal shipment order. | [optional]
**transaction_type** | Option<**String**> | The type of removal order.  Possible values:  * WHOLESALE_LIQUIDATION | [optional]
**store_name** | Option<**String**> | The name of the store where the event occurred. | [optional]
**removal_shipment_item_list** | Option<[**Vec<models::RemovalShipmentItem>**](RemovalShipmentItem.md)> | A list of information about removal shipment items. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


