use anyhow::Result;

use crate::{
    apis::fba_inventory_v1::get_inventory_summaries,
    client::SpapiClient,
    models::{
        self,
        fba_inventory::{GetInventorySummariesResponse, InventoryItem},
    },
};

impl SpapiClient {
    /// Returns transactions for the given parameters. Financial events might not include orders from the last 48 hours.  **Usage plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits).
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
