# DetailedShippingTimeType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**minimum_hours** | Option<**i64**> | The minimum time, in hours, that the item will likely be shipped after the order has been placed. | [optional]
**maximum_hours** | Option<**i64**> | The maximum time, in hours, that the item will likely be shipped after the order has been placed. | [optional]
**available_date** | Option<**String**> | The date when the item will be available for shipping. Only displayed for items that are not currently available for shipping. | [optional]
**availability_type** | Option<**String**> | Indicates whether the item is available for shipping now, or on a known or an unknown date in the future. If known, the availableDate property indicates the date that the item will be available for shipping. Possible values: NOW, FUTURE_WITHOUT_DATE, FUTURE_WITH_DATE. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


