# \ServiceApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_appointment_for_service_job_by_service_job_id**](ServiceApi.md#add_appointment_for_service_job_by_service_job_id) | **POST** /service/v1/serviceJobs/{serviceJobId}/appointments | 
[**assign_appointment_resources**](ServiceApi.md#assign_appointment_resources) | **PUT** /service/v1/serviceJobs/{serviceJobId}/appointments/{appointmentId}/resources | 
[**cancel_reservation**](ServiceApi.md#cancel_reservation) | **DELETE** /service/v1/reservation/{reservationId} | 
[**cancel_service_job_by_service_job_id**](ServiceApi.md#cancel_service_job_by_service_job_id) | **PUT** /service/v1/serviceJobs/{serviceJobId}/cancellations | 
[**complete_service_job_by_service_job_id**](ServiceApi.md#complete_service_job_by_service_job_id) | **PUT** /service/v1/serviceJobs/{serviceJobId}/completions | 
[**create_reservation**](ServiceApi.md#create_reservation) | **POST** /service/v1/reservation | 
[**create_service_document_upload_destination**](ServiceApi.md#create_service_document_upload_destination) | **POST** /service/v1/documents | 
[**get_appointment_slots**](ServiceApi.md#get_appointment_slots) | **GET** /service/v1/appointmentSlots | 
[**get_appointmment_slots_by_job_id**](ServiceApi.md#get_appointmment_slots_by_job_id) | **GET** /service/v1/serviceJobs/{serviceJobId}/appointmentSlots | 
[**get_fixed_slot_capacity**](ServiceApi.md#get_fixed_slot_capacity) | **POST** /service/v1/serviceResources/{resourceId}/capacity/fixed | 
[**get_range_slot_capacity**](ServiceApi.md#get_range_slot_capacity) | **POST** /service/v1/serviceResources/{resourceId}/capacity/range | 
[**get_service_job_by_service_job_id**](ServiceApi.md#get_service_job_by_service_job_id) | **GET** /service/v1/serviceJobs/{serviceJobId} | 
[**get_service_jobs**](ServiceApi.md#get_service_jobs) | **GET** /service/v1/serviceJobs | 
[**reschedule_appointment_for_service_job_by_service_job_id**](ServiceApi.md#reschedule_appointment_for_service_job_by_service_job_id) | **POST** /service/v1/serviceJobs/{serviceJobId}/appointments/{appointmentId} | 
[**set_appointment_fulfillment_data**](ServiceApi.md#set_appointment_fulfillment_data) | **PUT** /service/v1/serviceJobs/{serviceJobId}/appointments/{appointmentId}/fulfillment | 
[**update_reservation**](ServiceApi.md#update_reservation) | **PUT** /service/v1/reservation/{reservationId} | 
[**update_schedule**](ServiceApi.md#update_schedule) | **PUT** /service/v1/serviceResources/{resourceId}/schedules | 



## add_appointment_for_service_job_by_service_job_id

> models::SetAppointmentResponse add_appointment_for_service_job_by_service_job_id(service_job_id, body)


Adds an appointment to the service job indicated by the service job identifier specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_job_id** | **String** | An Amazon defined service job identifier. | [required] |
**body** | [**AddAppointmentRequest**](AddAppointmentRequest.md) | Add appointment operation input details. | [required] |

### Return type

[**models::SetAppointmentResponse**](SetAppointmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assign_appointment_resources

> models::AssignAppointmentResourcesResponse assign_appointment_resources(service_job_id, appointment_id, body)


Assigns new resource(s) or overwrite/update the existing one(s) to a service job appointment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 2 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_job_id** | **String** | An Amazon-defined service job identifier. Get this value by calling the `getServiceJobs` operation of the Services API. | [required] |
**appointment_id** | **String** | An Amazon-defined identifier of active service job appointment. | [required] |
**body** | [**AssignAppointmentResourcesRequest**](AssignAppointmentResourcesRequest.md) |  | [required] |

### Return type

[**models::AssignAppointmentResourcesResponse**](AssignAppointmentResourcesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_reservation

> models::CancelReservationResponse cancel_reservation(reservation_id, marketplace_ids)


Cancel a reservation.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reservation_id** | **String** | Reservation Identifier | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | An identifier for the marketplace in which the resource operates. | [required] |

### Return type

[**models::CancelReservationResponse**](CancelReservationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_service_job_by_service_job_id

> models::CancelServiceJobByServiceJobIdResponse cancel_service_job_by_service_job_id(service_job_id, cancellation_reason_code)


Cancels the service job indicated by the service job identifier specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_job_id** | **String** | An Amazon defined service job identifier. | [required] |
**cancellation_reason_code** | **String** | A cancel reason code that specifies the reason for cancelling a service job. | [required] |

### Return type

[**models::CancelServiceJobByServiceJobIdResponse**](CancelServiceJobByServiceJobIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_service_job_by_service_job_id

> models::CompleteServiceJobByServiceJobIdResponse complete_service_job_by_service_job_id(service_job_id)


Completes the service job indicated by the service job identifier specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_job_id** | **String** | An Amazon defined service job identifier. | [required] |

### Return type

[**models::CompleteServiceJobByServiceJobIdResponse**](CompleteServiceJobByServiceJobIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_reservation

> models::CreateReservationResponse create_reservation(marketplace_ids, body)


Create a reservation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_ids** | [**Vec<String>**](String.md) | An identifier for the marketplace in which the resource operates. | [required] |
**body** | [**CreateReservationRequest**](CreateReservationRequest.md) | Reservation details | [required] |

### Return type

[**models::CreateReservationResponse**](CreateReservationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_service_document_upload_destination

> models::CreateServiceDocumentUploadDestination create_service_document_upload_destination(body)


Creates an upload destination.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ServiceUploadDocument**](ServiceUploadDocument.md) | Upload document operation input details. | [required] |

### Return type

[**models::CreateServiceDocumentUploadDestination**](CreateServiceDocumentUploadDestination.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_appointment_slots

> models::GetAppointmentSlotsResponse get_appointment_slots(asin, store_id, marketplace_ids, start_time, end_time)


Gets appointment slots as per the service context specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 20 | 40 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asin** | **String** | ASIN associated with the service. | [required] |
**store_id** | **String** | Store identifier defining the region scope to retrive appointment slots. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | An identifier for the marketplace for which appointment slots are queried | [required] |
**start_time** | Option<**String**> | A time from which the appointment slots will be retrieved. The specified time must be in ISO 8601 format. If `startTime` is provided, `endTime` should also be provided. Default value is as per business configuration. |  |
**end_time** | Option<**String**> | A time up to which the appointment slots will be retrieved. The specified time must be in ISO 8601 format. If `endTime` is provided, `startTime` should also be provided. Default value is as per business configuration. Maximum range of appointment slots can be 90 days. |  |

### Return type

[**models::GetAppointmentSlotsResponse**](GetAppointmentSlotsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_appointmment_slots_by_job_id

> models::GetAppointmentSlotsResponse get_appointmment_slots_by_job_id(service_job_id, marketplace_ids, start_time, end_time)


Gets appointment slots for the service associated with the service job id specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_job_id** | **String** | A service job identifier to retrive appointment slots for associated service. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | An identifier for the marketplace in which the resource operates. | [required] |
**start_time** | Option<**String**> | A time from which the appointment slots will be retrieved. The specified time must be in ISO 8601 format. If `startTime` is provided, `endTime` should also be provided. Default value is as per business configuration. |  |
**end_time** | Option<**String**> | A time up to which the appointment slots will be retrieved. The specified time must be in ISO 8601 format. If `endTime` is provided, `startTime` should also be provided. Default value is as per business configuration. Maximum range of appointment slots can be 90 days. |  |

### Return type

[**models::GetAppointmentSlotsResponse**](GetAppointmentSlotsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fixed_slot_capacity

> models::FixedSlotCapacity get_fixed_slot_capacity(resource_id, marketplace_ids, body, next_page_token)


Provides capacity in fixed-size slots.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_id** | **String** | Resource Identifier. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | An identifier for the marketplace in which the resource operates. | [required] |
**body** | [**FixedSlotCapacityQuery**](FixedSlotCapacityQuery.md) | Request body. | [required] |
**next_page_token** | Option<**String**> | Next page token returned in the response of your previous request. |  |

### Return type

[**models::FixedSlotCapacity**](FixedSlotCapacity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_range_slot_capacity

> models::RangeSlotCapacity get_range_slot_capacity(resource_id, marketplace_ids, body, next_page_token)


Provides capacity slots in a format similar to availability records.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_id** | **String** | Resource Identifier. | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | An identifier for the marketplace in which the resource operates. | [required] |
**body** | [**RangeSlotCapacityQuery**](RangeSlotCapacityQuery.md) | Request body. | [required] |
**next_page_token** | Option<**String**> | Next page token returned in the response of your previous request. |  |

### Return type

[**models::RangeSlotCapacity**](RangeSlotCapacity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_job_by_service_job_id

> models::GetServiceJobByServiceJobIdResponse get_service_job_by_service_job_id(service_job_id)


Gets details of service job indicated by the provided `serviceJobID`.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 20 | 40 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_job_id** | **String** | A service job identifier. | [required] |

### Return type

[**models::GetServiceJobByServiceJobIdResponse**](GetServiceJobByServiceJobIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_service_jobs

> models::GetServiceJobsResponse get_service_jobs(marketplace_ids, service_order_ids, service_job_status, page_token, page_size, sort_field, sort_order, created_after, created_before, last_updated_after, last_updated_before, schedule_start_date, schedule_end_date, asins, required_skills, store_ids)


Gets service job details for the specified filter query.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 40 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_ids** | [**Vec<String>**](String.md) | Used to select jobs that were placed in the specified marketplaces. | [required] |
**service_order_ids** | Option<[**Vec<String>**](String.md)> | List of service order ids for the query you want to perform.Max values supported 20. |  |
**service_job_status** | Option<[**Vec<String>**](String.md)> | A list of one or more job status by which to filter the list of jobs. |  |
**page_token** | Option<**String**> | String returned in the response of your previous request. |  |
**page_size** | Option<**i32**> | A non-negative integer that indicates the maximum number of jobs to return in the list, Value must be 1 - 20. Default 20. |  |[default to 20]
**sort_field** | Option<**String**> | Sort fields on which you want to sort the output. |  |
**sort_order** | Option<**String**> | Sort order for the query you want to perform. |  |
**created_after** | Option<**String**> | A date used for selecting jobs created at or after a specified time. Must be in ISO 8601 format. Required if `LastUpdatedAfter` is not specified. Specifying both `CreatedAfter` and `LastUpdatedAfter` returns an error. |  |
**created_before** | Option<**String**> | A date used for selecting jobs created at or before a specified time. Must be in ISO 8601 format. |  |
**last_updated_after** | Option<**String**> | A date used for selecting jobs updated at or after a specified time. Must be in ISO 8601 format. Required if `createdAfter` is not specified. Specifying both `CreatedAfter` and `LastUpdatedAfter` returns an error. |  |
**last_updated_before** | Option<**String**> | A date used for selecting jobs updated at or before a specified time. Must be in ISO 8601 format. |  |
**schedule_start_date** | Option<**String**> | A date used for filtering jobs schedules at or after a specified time. Must be in ISO 8601 format. Schedule end date should not be earlier than schedule start date. |  |
**schedule_end_date** | Option<**String**> | A date used for filtering jobs schedules at or before a specified time. Must be in ISO 8601 format. Schedule end date should not be earlier than schedule start date. |  |
**asins** | Option<[**Vec<String>**](String.md)> | List of Amazon Standard Identification Numbers (ASIN) of the items. Max values supported is 20. |  |
**required_skills** | Option<[**Vec<String>**](String.md)> | A defined set of related knowledge, skills, experience, tools, materials, and work processes common to service delivery for a set of products and/or service scenarios. Max values supported is 20. |  |
**store_ids** | Option<[**Vec<String>**](String.md)> | List of Amazon-defined identifiers for the region scope. Max values supported is 50. |  |

### Return type

[**models::GetServiceJobsResponse**](GetServiceJobsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reschedule_appointment_for_service_job_by_service_job_id

> models::SetAppointmentResponse reschedule_appointment_for_service_job_by_service_job_id(service_job_id, appointment_id, body)


Reschedules an appointment for the service job indicated by the service job identifier specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_job_id** | **String** | An Amazon defined service job identifier. | [required] |
**appointment_id** | **String** | An existing appointment identifier for the Service Job. | [required] |
**body** | [**RescheduleAppointmentRequest**](RescheduleAppointmentRequest.md) | Reschedule appointment operation input details. | [required] |

### Return type

[**models::SetAppointmentResponse**](SetAppointmentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_appointment_fulfillment_data

> String set_appointment_fulfillment_data(service_job_id, appointment_id, body)


Updates the appointment fulfillment data related to a given `jobID` and `appointmentID`.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_job_id** | **String** | An Amazon-defined service job identifier. Get this value by calling the `getServiceJobs` operation of the Services API. | [required] |
**appointment_id** | **String** | An Amazon-defined identifier of active service job appointment. | [required] |
**body** | [**SetAppointmentFulfillmentDataRequest**](SetAppointmentFulfillmentDataRequest.md) | Appointment fulfillment data collection details. | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_reservation

> models::UpdateReservationResponse update_reservation(reservation_id, marketplace_ids, body)


Update a reservation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reservation_id** | **String** | Reservation Identifier | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | An identifier for the marketplace in which the resource operates. | [required] |
**body** | [**UpdateReservationRequest**](UpdateReservationRequest.md) | Reservation details | [required] |

### Return type

[**models::UpdateReservationResponse**](UpdateReservationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_schedule

> models::UpdateScheduleResponse update_schedule(resource_id, marketplace_ids, body)


Update the schedule of the given resource.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](doc:usage-plans-and-rate-limits-in-the-sp-api).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_id** | **String** | Resource (store) Identifier | [required] |
**marketplace_ids** | [**Vec<String>**](String.md) | An identifier for the marketplace in which the resource operates. | [required] |
**body** | [**UpdateScheduleRequest**](UpdateScheduleRequest.md) | Schedule details | [required] |

### Return type

[**models::UpdateScheduleResponse**](UpdateScheduleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

