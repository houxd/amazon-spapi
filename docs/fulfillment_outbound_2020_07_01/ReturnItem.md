# ReturnItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**seller_return_item_id** | **String** | An identifier assigned by the seller to the return item. | 
**seller_fulfillment_order_item_id** | **String** | The identifier assigned to the item by the seller when the fulfillment order was created. | 
**amazon_shipment_id** | **String** | The identifier for the shipment that is associated with the return item. | 
**seller_return_reason_code** | **String** | The return reason code assigned to the return item by the seller. | 
**return_comment** | Option<**String**> | An optional comment about the return item. | [optional]
**amazon_return_reason_code** | Option<**String**> | The return reason code that the Amazon fulfillment center assigned to the return item. | [optional]
**status** | [**models::FulfillmentReturnItemStatus**](FulfillmentReturnItemStatus.md) |  | 
**status_changed_date** | **String** | Date timestamp | 
**return_authorization_id** | Option<**String**> | Identifies the return authorization used to return this item. Refer to `ReturnAuthorization`. | [optional]
**return_received_condition** | Option<[**models::ReturnItemDisposition**](ReturnItemDisposition.md)> |  | [optional]
**fulfillment_center_id** | Option<**String**> | The identifier for the Amazon fulfillment center that processed the return item. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


