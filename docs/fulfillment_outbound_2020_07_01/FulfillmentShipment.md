# FulfillmentShipment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_shipment_id** | **String** | A shipment identifier assigned by Amazon. | 
**fulfillment_center_id** | **String** | An identifier for the fulfillment center that the shipment will be sent from. | 
**fulfillment_shipment_status** | **String** | The current status of the shipment. | 
**shipping_date** | Option<**String**> | Date timestamp | [optional]
**estimated_arrival_date** | Option<**String**> | Date timestamp | [optional]
**shipping_notes** | Option<**Vec<String>**> | Provides additional insight into shipment timeline. Primairly used to communicate that actual delivery dates aren't available. | [optional]
**fulfillment_shipment_item** | [**Vec<models::FulfillmentShipmentItem>**](FulfillmentShipmentItem.md) | An array of fulfillment shipment item information. | 
**fulfillment_shipment_package** | Option<[**Vec<models::FulfillmentShipmentPackage>**](FulfillmentShipmentPackage.md)> | An array of fulfillment shipment package information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


