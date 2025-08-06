# PackageTrackingDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**package_number** | **i32** | The package identifier. | 
**tracking_number** | Option<**String**> | The tracking number for the package. | [optional]
**customer_tracking_link** | Option<**String**> | Link on swiship.com that allows customers to track the package. | [optional]
**carrier_code** | Option<**String**> | The name of the carrier. | [optional]
**carrier_phone_number** | Option<**String**> | The phone number of the carrier. | [optional]
**carrier_url** | Option<**String**> | The URL of the carrier's website. | [optional]
**ship_date** | Option<**String**> | Date timestamp | [optional]
**estimated_arrival_date** | Option<**String**> | Date timestamp | [optional]
**ship_to_address** | Option<[**models::TrackingAddress**](TrackingAddress.md)> |  | [optional]
**current_status** | Option<[**models::CurrentStatus**](CurrentStatus.md)> |  | [optional]
**current_status_description** | Option<**String**> | Description corresponding to the CurrentStatus value. | [optional]
**delivery_window** | Option<[**models::DateRange**](DateRange.md)> |  | [optional]
**signed_for_by** | Option<**String**> | The name of the person who signed for the package. | [optional]
**additional_location_info** | Option<[**models::AdditionalLocationInfo**](AdditionalLocationInfo.md)> |  | [optional]
**tracking_events** | Option<[**Vec<models::TrackingEvent>**](TrackingEvent.md)> | An array of tracking event information. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


