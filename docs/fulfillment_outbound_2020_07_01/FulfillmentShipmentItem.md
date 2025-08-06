# FulfillmentShipmentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_sku** | **String** | The seller SKU of the item. | 
**seller_fulfillment_order_item_id** | **String** | The fulfillment order item identifier that the seller created and submitted with a call to the `createFulfillmentOrder` operation. | 
**quantity** | **i32** | The item quantity. | 
**package_number** | Option<**i32**> | An identifier for the package that contains the item quantity. | [optional]
**serial_number** | Option<**String**> | The serial number of the shipped item. | [optional]
**manufacturer_lot_codes** | Option<**Vec<String>**> | String list | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


