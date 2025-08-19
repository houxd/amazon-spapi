use crate::{
    client::SpapiClient,
    models::sellers::{GetAccountResponse, GetMarketplaceParticipationsResponse},
};
use anyhow::Result;

impl SpapiClient {
    pub async fn get_marketplace_participations(
        &self,
    ) -> Result<GetMarketplaceParticipationsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/sellers/v1/marketplaceParticipations", 0.016, 15)
            .await?;
        let res = crate::apis::sellers_api::get_marketplace_participations(&configuration).await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn get_account(&self) -> Result<GetAccountResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/sellers/v1/account", 0.016, 15)
            .await?;
        let res = crate::apis::sellers_api::get_account(&configuration).await?;
        guard.mark_response().await;
        Ok(res)
    }
}
