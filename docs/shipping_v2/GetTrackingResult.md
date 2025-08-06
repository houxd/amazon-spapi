# GetTrackingResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tracking_id** | **String** | The carrier generated identifier for a package in a purchased shipment. | 
**alternate_leg_tracking_id** | **String** | The carrier generated reverse identifier for a returned package in a purchased shipment. | 
**event_history** | [**Vec<models::Event>**](Event.md) | A list of tracking events. | 
**promised_delivery_date** | **String** | The date and time by which the shipment is promised to be delivered. | 
**summary** | [**models::TrackingSummary**](TrackingSummary.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


