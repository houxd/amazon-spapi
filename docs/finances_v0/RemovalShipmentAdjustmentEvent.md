# RemovalShipmentAdjustmentEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_date** | Option<**String**> | Fields with a schema type of date are in ISO 8601 date time format (for example GroupBeginDate). | [optional]
**adjustment_event_id** | Option<**String**> | The unique identifier for the adjustment event. | [optional]
**merchant_order_id** | Option<**String**> | The merchant removal orderId. | [optional]
**order_id** | Option<**String**> | The orderId for shipping inventory. | [optional]
**transaction_type** | Option<**String**> | The type of removal order.  Possible values:  * WHOLESALE_LIQUIDATION. | [optional]
**removal_shipment_item_adjustment_list** | Option<[**Vec<models::RemovalShipmentItemAdjustment>**](RemovalShipmentItemAdjustment.md)> | A comma-delimited list of Removal shipmentItemAdjustment details for FBA inventory. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


