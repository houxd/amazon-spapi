# Package

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scheduled_package_id** | [**models::ScheduledPackageId**](ScheduledPackageId.md) |  | 
**package_dimensions** | [**models::Dimensions**](Dimensions.md) |  | 
**package_weight** | [**models::Weight**](Weight.md) |  | 
**package_items** | Option<[**Vec<models::Item>**](Item.md)> | A list of items contained in the package. | [optional]
**package_time_slot** | [**models::TimeSlot**](TimeSlot.md) |  | 
**package_identifier** | Option<**String**> | Optional seller-created identifier that is printed on the shipping label to help the seller identify the package. | [optional]
**invoice** | Option<[**models::InvoiceData**](InvoiceData.md)> |  | [optional]
**package_status** | Option<[**models::PackageStatus**](PackageStatus.md)> |  | [optional]
**tracking_details** | Option<[**models::TrackingDetails**](TrackingDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


