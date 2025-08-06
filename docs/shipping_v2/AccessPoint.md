# AccessPoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_point_id** | Option<**String**> | Unique identifier for the access point | [optional]
**name** | Option<**String**> | Name of entity (store/hub etc) where this access point is located | [optional]
**timezone** | Option<**String**> | Timezone of access point | [optional]
**r#type** | Option<[**models::AccessPointType**](AccessPointType.md)> |  | [optional]
**accessibility_attributes** | Option<[**models::AccessibilityAttributes**](AccessibilityAttributes.md)> |  | [optional]
**address** | Option<[**models::Address**](Address.md)> |  | [optional]
**exception_operating_hours** | Option<[**Vec<models::ExceptionOperatingHours>**](ExceptionOperatingHours.md)> | Exception operating hours for Access Point | [optional]
**assistance_type** | Option<**String**> | Assistance type enum for Access point i.e. STAFF_ASSISTED or SELF_ASSISTED | [optional]
**score** | Option<**String**> | The score of access point, based on proximity to postal code and sorting preference. This can be used to sort access point results on shipper's end. | [optional]
**standard_operating_hours** | Option<[**std::collections::HashMap<String, models::OperatingHours>**](OperatingHours.md)> | Map of day of the week to operating hours of that day | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


