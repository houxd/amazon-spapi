use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Returns aggregated order metrics for given interval, broken down by granularity, for given buyer type.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | .5 | 15 |
    pub async fn get_order_metrics(
        &self,
        marketplace_ids: Vec<String>,
        interval: &str,
        granularity: &str,
        granularity_time_zone: Option<&str>,
        buyer_type: Option<&str>,
        fulfillment_network: Option<&str>,
        first_day_of_week: Option<&str>,
        asin: Option<&str>,
        sku: Option<&str>,
        amazon_program: Option<&str>,
    ) -> Result<models::sales::GetOrderMetricsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("sales_v1/get_order_metrics", 0.5, 15)
            .await?;
        let res = crate::apis::sales_v1::get_order_metrics(
            &configuration,
            marketplace_ids,
            interval,
            granularity,
            granularity_time_zone,
            buyer_type,
            fulfillment_network,
            first_day_of_week,
            asin,
            sku,
            amazon_program,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
