# FixedSlot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date_time** | Option<**String**> | Start date time of slot in ISO 8601 format with precision of seconds. | [optional]
**scheduled_capacity** | Option<**i32**> | Scheduled capacity corresponding to the slot. This capacity represents the originally allocated capacity as per resource schedule. | [optional]
**available_capacity** | Option<**i32**> | Available capacity corresponding to the slot. This capacity represents the capacity available for allocation to reservations. | [optional]
**encumbered_capacity** | Option<**i32**> | Encumbered capacity corresponding to the slot. This capacity represents the capacity allocated for Amazon Jobs/Appointments/Orders. | [optional]
**reserved_capacity** | Option<**i32**> | Reserved capacity corresponding to the slot. This capacity represents the capacity made unavailable due to events like Breaks/Leaves/Lunch. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


