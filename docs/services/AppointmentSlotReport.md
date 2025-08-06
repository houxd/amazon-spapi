# AppointmentSlotReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scheduling_type** | Option<**String**> | Defines the type of slots. | [optional]
**start_time** | Option<**String**> | Start Time from which the appointment slots are generated in ISO 8601 format. | [optional]
**end_time** | Option<**String**> | End Time up to which the appointment slots are generated in ISO 8601 format. | [optional]
**appointment_slots** | Option<[**Vec<models::AppointmentSlot>**](AppointmentSlot.md)> | A list of time windows along with associated capacity in which the service can be performed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


