use crate::{
    client::{ApiEndpoint, ApiMethod, SpapiClient},
    models,
};
use anyhow::Result;

impl SpapiClient {
    pub async fn get_listings_restrictions(
        &self,
        asin: &str,
        seller_id: &str,
        marketplace_ids: Vec<String>,
        condition_type: Option<&str>,
        reason_locale: Option<&str>,
    ) -> Result<models::listings_restrictions_2021_08_01::RestrictionList> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/listings/2021-08-01/restrictions", 5.0, 10)
            .await?;
        
        let res = crate::apis::listings_restrictions_2021_08_01::get_listings_restrictions(
            &configuration,
            asin,
            seller_id,
            marketplace_ids,
            condition_type,
            reason_locale,
        )
        .await?;
        
        guard.mark_response().await;
        Ok(res)
    }
}
