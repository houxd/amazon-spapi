use crate::{client::SpapiClient, models};
use anyhow::Result;

impl SpapiClient {
    pub async fn get_selling_partner_metrics(
        &self,
        body: Option<models::replenishment_2022_11_07::GetSellingPartnerMetricsRequest>,
    ) -> Result<models::replenishment_2022_11_07::GetSellingPartnerMetricsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/replenishment/2022-11-07/sellingPartners/metrics/search", 1.0, 1)
            .await?;
        let res = crate::apis::replenishment_2022_11_07::get_selling_partner_metrics(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn list_offer_metrics(
        &self,
        body: Option<models::replenishment_2022_11_07::ListOfferMetricsRequest>,
    ) -> Result<models::replenishment_2022_11_07::ListOfferMetricsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/replenishment/2022-11-07/offers/metrics/search", 1.0, 1)
            .await?;
        let res = crate::apis::replenishment_2022_11_07::list_offer_metrics(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn list_offers(
        &self,
        body: Option<models::replenishment_2022_11_07::ListOffersRequest>,
    ) -> Result<models::replenishment_2022_11_07::ListOffersResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/replenishment/2022-11-07/offers/search", 1.0, 1)
            .await?;
        let res = crate::apis::replenishment_2022_11_07::list_offers(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
