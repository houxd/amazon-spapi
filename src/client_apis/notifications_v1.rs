use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Creates a destination resource to receive notifications. The `createDestination` operation is grantless. For more information, refer to [Grantless operations](https://developer-docs.amazon.com/sp-api/docs/grantless-operations).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_destination(
        &self,
        body: models::notifications::CreateDestinationRequest,
    ) -> Result<models::notifications::CreateDestinationResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("notifications_v1/create_destination", 1.0, 5)
            .await?;
        let res = crate::apis::notifications_v1::create_destination(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Creates a subscription for the specified notification type to be delivered to the specified destination. Before you can subscribe, you must first create the destination by calling the `createDestination` operation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_subscription(
        &self,
        notification_type: &str,
        body: models::notifications::CreateSubscriptionRequest,
    ) -> Result<models::notifications::CreateSubscriptionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("notifications_v1/create_subscription", 1.0, 5)
            .await?;
        let res = crate::apis::notifications_v1::create_subscription(
            &configuration,
            notification_type,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Deletes the destination that you specify. The `deleteDestination` operation is grantless. For more information, refer to [Grantless operations](https://developer-docs.amazon.com/sp-api/docs/grantless-operations).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn delete_destination(
        &self,
        destination_id: &str,
    ) -> Result<models::notifications::DeleteDestinationResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("notifications_v1/delete_destination", 1.0, 5)
            .await?;
        let res = crate::apis::notifications_v1::delete_destination(
            &configuration,
            destination_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Deletes the subscription indicated by the subscription identifier and notification type that you specify. The `deleteSubscriptionById` operation is grantless.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn delete_subscription_by_id(
        &self,
        subscription_id: &str,
        notification_type: &str,
    ) -> Result<models::notifications::DeleteSubscriptionByIdResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("notifications_v1/delete_subscription_by_id", 1.0, 5)
            .await?;
        let res = crate::apis::notifications_v1::delete_subscription_by_id(
            &configuration,
            subscription_id,
            notification_type,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns information about the destination that you specify. The `getDestination` operation is grantless.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_destination(
        &self,
        destination_id: &str,
    ) -> Result<models::notifications::GetDestinationResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("notifications_v1/get_destination", 1.0, 5)
            .await?;
        let res = crate::apis::notifications_v1::get_destination(
            &configuration,
            destination_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns information about all destinations. The `getDestinations` operation is grantless.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_destinations(
        &self,
    ) -> Result<models::notifications::GetDestinationsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("notifications_v1/get_destinations", 1.0, 5)
            .await?;
        let res = crate::apis::notifications_v1::get_destinations(
            &configuration,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns information about subscription of the specified notification type and payload version. `payloadVersion` is an optional parameter.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_subscription(
        &self,
        notification_type: &str,
        payload_version: Option<&str>,
    ) -> Result<models::notifications::GetSubscriptionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("notifications_v1/get_subscription", 1.0, 5)
            .await?;
        let res = crate::apis::notifications_v1::get_subscription(
            &configuration,
            notification_type,
            payload_version,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns information about a subscription for the specified notification type. The `getSubscriptionById` operation is grantless.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_subscription_by_id(
        &self,
        subscription_id: &str,
        notification_type: &str,
    ) -> Result<models::notifications::GetSubscriptionByIdResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("notifications_v1/get_subscription_by_id", 1.0, 5)
            .await?;
        let res = crate::apis::notifications_v1::get_subscription_by_id(
            &configuration,
            subscription_id,
            notification_type,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
