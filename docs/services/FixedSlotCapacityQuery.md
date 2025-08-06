# FixedSlotCapacityQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**capacity_types** | Option<[**Vec<models::CapacityType>**](CapacityType.md)> | An array of capacity types which are being requested. Default value is `[SCHEDULED_CAPACITY]`. | [optional]
**slot_duration** | Option<**f64**> | Size in which slots are being requested. This value should be a multiple of 5 and fall in the range: 5 <= `slotDuration` <= 360. | [optional]
**start_date_time** | **String** | Start date time from which the capacity slots are being requested in ISO 8601 format. | 
**end_date_time** | **String** | End date time up to which the capacity slots are being requested in ISO 8601 format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


