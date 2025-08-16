use anyhow::Result;

use crate::{
    apis::fba_inventory_api::get_inventory_summaries,
    client::{ApiEndpoint, ApiMethod, SpapiClient},
    models::{
        self,
        fba_inventory::{GetInventorySummariesResponse, InventoryItem},
    },
};

impl SpapiClient {
    pub async fn list_transactions(
        &self,
        posted_after: String,
        posted_before: Option<String>,
        marketplace_id: Option<&str>,
        transaction_status: Option<&str>,
        next_token: Option<&str>,
    ) -> Result<models::finances_2024_06_19::ListTransactionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/finances/2024-06-19/listTransactions", 0.5, 10)
            .await?;
        let res = crate::apis::finances_2024_06_19::list_transactions(
            &configuration,
            posted_after,
            posted_before,
            marketplace_id,
            transaction_status,
            next_token,
        )
        .await;
        guard.mark_response().await;
        Ok(res?)
    }
}
