use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Create a notification for sellers in Seller Central.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_notification(
        &self,
        body: models::app_integrations_2024_04_01::CreateNotificationRequest,
    ) -> Result<models::app_integrations_2024_04_01::CreateNotificationResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("app_integrations_2024_04_01/create_notification", 1.0, 5)
            .await?;
        let res = crate::apis::app_integrations_2024_04_01::create_notification(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Remove your application's notifications from the Appstore notifications dashboard.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn delete_notifications(
        &self,
        body: models::app_integrations_2024_04_01::DeleteNotificationsRequest,
    ) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("app_integrations_2024_04_01/delete_notifications", 1.0, 5)
            .await?;
        let res = crate::apis::app_integrations_2024_04_01::delete_notifications(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Records the seller's response to a notification.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn record_action_feedback(
        &self,
        notification_id: &str,
        body: models::app_integrations_2024_04_01::RecordActionFeedbackRequest,
    ) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("app_integrations_2024_04_01/record_action_feedback", 1.0, 5)
            .await?;
        let res = crate::apis::app_integrations_2024_04_01::record_action_feedback(
            &configuration,
            notification_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}