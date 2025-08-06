# Shipment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_reference_id** | Option<**String**> | A unique identifier created by Amazon that identifies this Amazon-partnered, Less Than Truckload/Full Truckload (LTL/FTL) shipment. | [optional]
**contact_information** | Option<[**models::ContactInformation**](ContactInformation.md)> |  | [optional]
**dates** | Option<[**models::Dates**](Dates.md)> |  | [optional]
**destination** | [**models::ShipmentDestination**](ShipmentDestination.md) |  | 
**freight_information** | Option<[**models::FreightInformation**](FreightInformation.md)> |  | [optional]
**name** | Option<**String**> | The name of the shipment. | [optional]
**placement_option_id** | **String** | The identifier of a placement option. A placement option represents the shipment splits and destinations of SKUs. | 
**selected_delivery_window** | Option<[**models::SelectedDeliveryWindow**](SelectedDeliveryWindow.md)> |  | [optional]
**selected_transportation_option_id** | Option<**String**> | Identifier of a transportation option. A transportation option represent one option for how to send a shipment. | [optional]
**self_ship_appointment_details** | Option<[**Vec<models::SelfShipAppointmentDetails>**](SelfShipAppointmentDetails.md)> | List of self ship appointment details. | [optional]
**shipment_confirmation_id** | Option<**String**> | The confirmed shipment ID which shows up on labels (for example, `FBA1234ABCD`). | [optional]
**shipment_id** | **String** | Identifier of a shipment. A shipment contains the boxes and units being inbounded. | 
**source** | [**models::ShipmentSource**](ShipmentSource.md) |  | 
**status** | Option<**String**> | The status of a shipment. The state of the shipment will typically start as `UNCONFIRMED`, then transition to `WORKING` after a placement option has been confirmed, and then to `READY_TO_SHIP` once labels are generated.  Possible values: `ABANDONED`, `CANCELLED`, `CHECKED_IN`, `CLOSED`, `DELETED`, `DELIVERED`, `IN_TRANSIT`, `MIXED`, `READY_TO_SHIP`, `RECEIVING`, `SHIPPED`, `UNCONFIRMED`, `WORKING` | [optional]
**tracking_details** | Option<[**models::TrackingDetails**](TrackingDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


