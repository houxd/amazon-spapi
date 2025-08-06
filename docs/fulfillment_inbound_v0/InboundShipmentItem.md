# InboundShipmentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**shipment_id** | Option<**String**> | A shipment identifier originally returned by the createInboundShipmentPlan operation. | [optional]
**seller_sku** | **String** | The seller SKU of the item. | 
**fulfillment_network_sku** | Option<**String**> | Amazon's fulfillment network SKU of the item. | [optional]
**quantity_shipped** | **i32** | The item quantity. | 
**quantity_received** | Option<**i32**> | The item quantity. | [optional]
**quantity_in_case** | Option<**i32**> | The item quantity. | [optional]
**release_date** | Option<[**String**](string.md)> | Type containing date in string format | [optional]
**prep_details_list** | Option<[**Vec<models::PrepDetails>**](PrepDetails.md)> | A list of preparation instructions and who is responsible for that preparation. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


