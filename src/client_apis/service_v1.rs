use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Adds an appointment to the service job indicated by the service job identifier specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn add_appointment_for_service_job_by_service_job_id(
        &self,
        service_job_id: &str,
        body: models::services::AddAppointmentRequest,
    ) -> Result<models::services::SetAppointmentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/add_appointment_for_service_job_by_service_job_id", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::add_appointment_for_service_job_by_service_job_id(
            &configuration,
            service_job_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Assigns new resource(s) or overwrite/update the existing one(s) to a service job appointment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 2 |
    pub async fn assign_appointment_resources(
        &self,
        service_job_id: &str,
        appointment_id: &str,
        body: models::services::AssignAppointmentResourcesRequest,
    ) -> Result<models::services::AssignAppointmentResourcesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/assign_appointment_resources", 1.0, 2)
            .await?;
        let res = crate::apis::service_v1::assign_appointment_resources(
            &configuration,
            service_job_id,
            appointment_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Cancel a reservation.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn cancel_reservation(
        &self,
        reservation_id: &str,
        marketplace_ids: Vec<String>,
    ) -> Result<models::services::CancelReservationResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/cancel_reservation", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::cancel_reservation(
            &configuration,
            reservation_id,
            marketplace_ids,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Cancels the service job indicated by the service job identifier specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn cancel_service_job_by_service_job_id(
        &self,
        service_job_id: &str,
        cancellation_reason_code: &str,
    ) -> Result<models::services::CancelServiceJobByServiceJobIdResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/cancel_service_job_by_service_job_id", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::cancel_service_job_by_service_job_id(
            &configuration,
            service_job_id,
            cancellation_reason_code,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Completes the service job indicated by the service job identifier specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn complete_service_job_by_service_job_id(
        &self,
        service_job_id: &str,
    ) -> Result<models::services::CompleteServiceJobByServiceJobIdResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/complete_service_job_by_service_job_id", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::complete_service_job_by_service_job_id(
            &configuration,
            service_job_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Create a reservation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn create_reservation(
        &self,
        marketplace_ids: Vec<String>,
        body: models::services::CreateReservationRequest,
    ) -> Result<models::services::CreateReservationResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/create_reservation", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::create_reservation(
            &configuration,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Creates an upload destination.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn create_service_document_upload_destination(
        &self,
        body: models::services::ServiceUploadDocument,
    ) -> Result<models::services::CreateServiceDocumentUploadDestination> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/create_service_document_upload_destination", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::create_service_document_upload_destination(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Gets appointment slots as per the service context specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 20 | 40 |
    pub async fn get_appointment_slots(
        &self,
        asin: &str,
        store_id: &str,
        marketplace_ids: Vec<String>,
        start_time: Option<&str>,
        end_time: Option<&str>,
    ) -> Result<models::services::GetAppointmentSlotsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/get_appointment_slots", 20.0, 40)
            .await?;
        let res = crate::apis::service_v1::get_appointment_slots(
            &configuration,
            asin,
            store_id,
            marketplace_ids,
            start_time,
            end_time,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Gets appointment slots for the service associated with the service job id specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn get_appointmment_slots_by_job_id(
        &self,
        service_job_id: &str,
        marketplace_ids: Vec<String>,
        start_time: Option<&str>,
        end_time: Option<&str>,
    ) -> Result<models::services::GetAppointmentSlotsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/get_appointmment_slots_by_job_id", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::get_appointmment_slots_by_job_id(
            &configuration,
            service_job_id,
            marketplace_ids,
            start_time,
            end_time,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides capacity in fixed-size slots.   **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn get_fixed_slot_capacity(
        &self,
        resource_id: &str,
        marketplace_ids: Vec<String>,
        body: models::services::FixedSlotCapacityQuery,
        next_page_token: Option<&str>,
    ) -> Result<models::services::FixedSlotCapacity> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/get_fixed_slot_capacity", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::get_fixed_slot_capacity(
            &configuration,
            resource_id,
            marketplace_ids,
            body,
            next_page_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides capacity slots in a format similar to availability records.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn get_range_slot_capacity(
        &self,
        resource_id: &str,
        marketplace_ids: Vec<String>,
        body: models::services::RangeSlotCapacityQuery,
        next_page_token: Option<&str>,
    ) -> Result<models::services::RangeSlotCapacity> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/get_range_slot_capacity", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::get_range_slot_capacity(
            &configuration,
            resource_id,
            marketplace_ids,
            body,
            next_page_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Gets details of service job indicated by the provided `serviceJobID`.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 20 | 40 |
    pub async fn get_service_job_by_service_job_id(
        &self,
        service_job_id: &str,
    ) -> Result<models::services::GetServiceJobByServiceJobIdResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/get_service_job_by_service_job_id", 20.0, 40)
            .await?;
        let res = crate::apis::service_v1::get_service_job_by_service_job_id(
            &configuration,
            service_job_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Gets service job details for the specified filter query.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 40 |
    pub async fn get_service_jobs(
        &self,
        marketplace_ids: Vec<String>,
        service_order_ids: Option<Vec<String>>,
        service_job_status: Option<Vec<String>>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        sort_field: Option<&str>,
        sort_order: Option<&str>,
        created_after: Option<&str>,
        created_before: Option<&str>,
        last_updated_after: Option<&str>,
        last_updated_before: Option<&str>,
        schedule_start_date: Option<&str>,
        schedule_end_date: Option<&str>,
        asins: Option<Vec<String>>,
        required_skills: Option<Vec<String>>,
        store_ids: Option<Vec<String>>,
    ) -> Result<models::services::GetServiceJobsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/get_service_jobs", 10.0, 40)
            .await?;
        let res = crate::apis::service_v1::get_service_jobs(
            &configuration,
            marketplace_ids,
            service_order_ids,
            service_job_status,
            page_token,
            page_size,
            sort_field,
            sort_order,
            created_after,
            created_before,
            last_updated_after,
            last_updated_before,
            schedule_start_date,
            schedule_end_date,
            asins,
            required_skills,
            store_ids,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Reschedules an appointment for the service job indicated by the service job identifier specified.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn reschedule_appointment_for_service_job_by_service_job_id(
        &self,
        service_job_id: &str,
        appointment_id: &str,
        body: models::services::RescheduleAppointmentRequest,
    ) -> Result<models::services::SetAppointmentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/reschedule_appointment_for_service_job_by_service_job_id", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::reschedule_appointment_for_service_job_by_service_job_id(
            &configuration,
            service_job_id,
            appointment_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates the appointment fulfillment data related to a given `jobID` and `appointmentID`.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn set_appointment_fulfillment_data(
        &self,
        service_job_id: &str,
        appointment_id: &str,
        body: models::services::SetAppointmentFulfillmentDataRequest,
    ) -> Result<String> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/set_appointment_fulfillment_data", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::set_appointment_fulfillment_data(
            &configuration,
            service_job_id,
            appointment_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Update a reservation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn update_reservation(
        &self,
        reservation_id: &str,
        marketplace_ids: Vec<String>,
        body: models::services::UpdateReservationRequest,
    ) -> Result<models::services::UpdateReservationResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/update_reservation", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::update_reservation(
            &configuration,
            reservation_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Update the schedule of the given resource.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 5 | 20 |
    pub async fn update_schedule(
        &self,
        resource_id: &str,
        marketplace_ids: Vec<String>,
        body: models::services::UpdateScheduleRequest,
    ) -> Result<models::services::UpdateScheduleResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("service_v1/update_schedule", 5.0, 20)
            .await?;
        let res = crate::apis::service_v1::update_schedule(
            &configuration,
            resource_id,
            marketplace_ids,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
