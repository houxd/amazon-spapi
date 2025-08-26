use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Rotates application client secrets for a developer application. Developers must register a destination queue in the developer console before calling this operation. When this operation is called a new client secret is generated and sent to the developer-registered queue. For more information, refer to [Rotate your application client secret](https://developer-docs.amazon.com/sp-api/v0/docs/application-management-api-v2023-11-30-use-case-guide#tutorial-rotate-your-applications-client-secret).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0167 | 1 |
    pub async fn rotate_application_client_secret(
        &self,
    ) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("application_2023_11_30/rotate_application_client_secret", 0.0167, 1)
            .await?;
        let res = crate::apis::application_2023_11_30::rotate_application_client_secret(
            &configuration,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
