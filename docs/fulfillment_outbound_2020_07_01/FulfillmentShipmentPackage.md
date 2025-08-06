# FulfillmentShipmentPackage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**package_number** | **i32** | Identifies a package in a shipment. | 
**carrier_code** | **String** | Identifies the carrier who will deliver the shipment to the recipient. | 
**tracking_number** | Option<**String**> | The tracking number, if provided, can be used to obtain tracking and delivery information. | [optional]
**estimated_arrival_date** | Option<**String**> | Date timestamp | [optional]
**locker_details** | Option<[**models::LockerDetails**](LockerDetails.md)> |  | [optional]
**delivery_information** | Option<[**models::DeliveryInformation**](DeliveryInformation.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


