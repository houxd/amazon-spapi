use crate::{
    client::{ApiEndpoint, ApiMethod, SpapiClient},
    models::sellers::{GetAccountResponse, GetMarketplaceParticipationsResponse},
};
use anyhow::Result;

impl SpapiClient {
    pub async fn get_marketplace_participations(
        &self,
    ) -> Result<GetMarketplaceParticipationsResponse> {
        let configuration = self.create_configuration().await?;
        let _ = self
            .limiter()
            .wait("/sellers/v1/marketplaceParticipations", 0.016, 15)
            .await?;
        let res = crate::apis::sellers_api::get_marketplace_participations(&configuration).await?;
        Ok(res)
    }

    pub async fn get_account(&self) -> Result<GetAccountResponse> {
        let endpoint = ApiEndpoint {
            version: "sellers_v1",
            path: "/sellers/v1/account",
            path_params: None,
            method: ApiMethod::Get,
            rate: 0.016,
            burst: 15,
        };
        let res = self.request(&endpoint, None, None, None).await?;
        Self::from_json(&res)
    }
}
