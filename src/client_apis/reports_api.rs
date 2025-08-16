use std::time::Duration;

use crate::{
    client::{ApiEndpoint, ApiMethod, SpapiClient},
    models::{
        self,
        reports_2021_06_30::{report::ProcessingStatus, CreateReportSpecification},
        sellers::{GetAccountResponse, GetMarketplaceParticipationsResponse},
    },
};
use anyhow::Result;
use tokio::time::sleep;

impl SpapiClient {
    pub async fn cancel_report(&self, report_id: &str) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/reports/v2021-06-30/reports/{reportId}/cancel", 0.016, 15)
            .await?;
        let res = crate::apis::reports_api::cancel_report(&configuration, report_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn cancel_report_schedule(&self, report_schedule_id: &str) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "/reports/v2021-06-30/schedules/{reportScheduleId}/cancel",
                0.016,
                15,
            )
            .await?;
        let res =
            crate::apis::reports_api::cancel_report_schedule(&configuration, report_schedule_id)
                .await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn create_report(
        &self,
        body: models::reports_2021_06_30::CreateReportSpecification,
    ) -> Result<models::reports_2021_06_30::CreateReportResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/reports/v2021-06-30/reports", 0.016, 15)
            .await?;
        let res = crate::apis::reports_api::create_report(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn create_report_schedule(
        &self,
        body: models::reports_2021_06_30::CreateReportScheduleSpecification,
    ) -> Result<models::reports_2021_06_30::CreateReportScheduleResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/reports/v2021-06-30/schedules", 0.016, 15)
            .await?;
        let res = crate::apis::reports_api::create_report_schedule(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn get_report(&self, report_id: &str) -> Result<models::reports_2021_06_30::Report> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/reports/v2021-06-30/reports/{reportId}", 0.016, 15)
            .await?;
        let res = crate::apis::reports_api::get_report(&configuration, report_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn get_report_document(
        &self,
        report_document_id: &str,
    ) -> Result<models::reports_2021_06_30::ReportDocument> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "/reports/v2021-06-30/reports/{reportId}/document",
                0.016,
                15,
            )
            .await?;
        let res = crate::apis::reports_api::get_report_document(&configuration, report_document_id)
            .await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn get_report_schedule(
        &self,
        report_schedule_id: &str,
    ) -> Result<models::reports_2021_06_30::ReportSchedule> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "/reports/v2021-06-30/schedules/{reportScheduleId}",
                0.016,
                15,
            )
            .await?;
        let res =
            crate::apis::reports_api::get_report_schedule(&configuration, report_schedule_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn get_report_schedules(
        &self,
        report_types: Vec<String>,
    ) -> Result<models::reports_2021_06_30::ReportScheduleList> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/reports/v2021-06-30/schedules", 0.016, 15)
            .await?;
        let res =
            crate::apis::reports_api::get_report_schedules(&configuration, report_types).await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn get_reports(
        &self,
        report_types: Option<Vec<String>>,
        processing_statuses: Option<Vec<String>>,
        marketplace_ids: Option<Vec<String>>,
        page_size: Option<i32>,
        created_since: Option<String>,
        created_until: Option<String>,
        next_token: Option<&str>,
    ) -> Result<models::reports_2021_06_30::GetReportsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/reports/v2021-06-30/reports", 0.016, 15)
            .await?;
        let res = crate::apis::reports_api::get_reports(
            &configuration,
            report_types,
            processing_statuses,
            marketplace_ids,
            page_size,
            created_since,
            created_until,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Convenience method to fetch a report by type and marketplace IDs, defaulting to a 30-minute wait.
    pub async fn fetch_report(
        &self,
        report_type: &str,
        marketplace_ids: Vec<String>,
        max_wait_minutes: Option<u32>,
        progress_callback: Option<impl Fn(u32, ProcessingStatus) + Send + 'static>,
    ) -> Result<String> {
        let max_wait_minutes = max_wait_minutes.unwrap_or(30);
        let check_interval_seconds = 10;
        let max_attempts = (max_wait_minutes * 60 / check_interval_seconds) as u32;

        let create_report_spec = CreateReportSpecification {
            report_type: report_type.to_string(),
            marketplace_ids,
            report_options: None,
            data_start_time: None,
            data_end_time: None,
        };

        let create_response = self.create_report(create_report_spec).await?;
        let report_id = &create_response.report_id;

        for attempt_count in 1..=max_attempts {
            let report = self.get_report(report_id).await?;

            if let Some(callback) = &progress_callback {
                callback(attempt_count, report.processing_status.clone());
            }

            match report.processing_status {
                ProcessingStatus::Done => {
                    log::debug!("Report generation completed!");

                    if let Some(document_id) = report.report_document_id {
                        let document = self.get_report_document(&document_id).await?;
                        log::info!("Report document ID: {}", document.report_document_id);

                        let report_content =
                            if let Some(compression) = &document.compression_algorithm {
                                log::info!("Report uses compression algorithm: {:?}", compression);
                                // TODO: decompress the report content
                                self.download(&document.url).await?
                            } else {
                                self.download(&document.url).await?
                            };

                        log::debug!("Report content downloaded successfully.");
                        return Ok(report_content);
                    } else {
                        return Err(anyhow::anyhow!(
                            "Report generation completed but no document ID found"
                        ));
                    }
                }
                ProcessingStatus::Cancelled => {
                    return Err(anyhow::anyhow!("Report generation was cancelled"));
                }
                ProcessingStatus::Fatal => {
                    return Err(anyhow::anyhow!("Report generation failed"));
                }
                ProcessingStatus::InProgress => {
                    log::debug!("Report is being generated, waiting 30 seconds before retrying...");
                }
                ProcessingStatus::InQueue => {
                    log::debug!("Report is in queue waiting to be processed, waiting 30 seconds before retrying...");
                }
            }

            if attempt_count < max_attempts {
                log::debug!(
                    "Waiting {} seconds before next check...",
                    check_interval_seconds
                );
                sleep(Duration::from_secs(check_interval_seconds as u64)).await;
            }
        }

        Err(anyhow::anyhow!(
            "Report generation timed out after {} minutes ({} attempts)",
            max_wait_minutes,
            max_attempts
        ))
    }
}
