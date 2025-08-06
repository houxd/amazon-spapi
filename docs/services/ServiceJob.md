# ServiceJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**create_time** | Option<**String**> | The date and time of the creation of the job in ISO 8601 format. | [optional]
**service_job_id** | Option<**String**> | Amazon identifier for the service job. | [optional]
**service_job_status** | Option<**String**> | The status of the service job. | [optional]
**scope_of_work** | Option<[**models::ScopeOfWork**](ScopeOfWork.md)> |  | [optional]
**seller** | Option<[**models::Seller**](Seller.md)> |  | [optional]
**service_job_provider** | Option<[**models::ServiceJobProvider**](ServiceJobProvider.md)> |  | [optional]
**preferred_appointment_times** | Option<[**Vec<models::AppointmentTime>**](AppointmentTime.md)> | A list of appointment windows preferred by the buyer. Included only if the buyer selected appointment windows when creating the order. | [optional]
**appointments** | Option<[**Vec<models::Appointment>**](Appointment.md)> | A list of appointments. | [optional]
**service_order_id** | Option<**String**> | The Amazon-defined identifier for an order placed by the buyer, in 3-7-7 format. | [optional]
**marketplace_id** | Option<**String**> | The marketplace identifier. | [optional]
**store_id** | Option<**String**> | The Amazon-defined identifier for the region scope. | [optional]
**buyer** | Option<[**models::Buyer**](Buyer.md)> |  | [optional]
**associated_items** | Option<[**Vec<models::AssociatedItem>**](AssociatedItem.md)> | A list of items associated with the service job. | [optional]
**service_location** | Option<[**models::ServiceLocation**](ServiceLocation.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


