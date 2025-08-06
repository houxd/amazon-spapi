# FixedSlotCapacity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**resource_id** | Option<**String**> | Resource Identifier. | [optional]
**slot_duration** | Option<**f64**> | The duration of each slot which is returned. This value will be a multiple of 5 and fall in the following range: 5 <= `slotDuration` <= 360. | [optional]
**capacities** | Option<[**Vec<models::FixedSlot>**](FixedSlot.md)> | Array of capacity slots in fixed slot format. | [optional]
**next_page_token** | Option<**String**> | Next page token, if there are more pages. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


