use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Returns financial event groups for a given date range. It may take up to 48 hours for orders to appear in your financial events.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn list_financial_event_groups(
        &self,
        max_results_per_page: Option<i32>,
        financial_event_group_started_before: Option<String>,
        financial_event_group_started_after: Option<String>,
        next_token: Option<&str>,
    ) -> Result<models::finances_v0::ListFinancialEventGroupsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/finances/v0/financialEventGroups", 0.5, 30)
            .await?;
        let res = crate::apis::finances_v0::list_financial_event_groups(
            &configuration,
            max_results_per_page,
            financial_event_group_started_before,
            financial_event_group_started_after,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns financial events for the specified data range. It may take up to 48 hours for orders to appear in your financial events. **Note:** in `ListFinancialEvents`, deferred events don't show up in responses until in they are released.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn list_financial_events(
        &self,
        max_results_per_page: Option<i32>,
        posted_after: Option<String>,
        posted_before: Option<String>,
        next_token: Option<&str>,
    ) -> Result<models::finances_v0::ListFinancialEventsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/finances/v0/financialEvents", 0.5, 30)
            .await?;
        let res = crate::apis::finances_v0::list_financial_events(
            &configuration,
            max_results_per_page,
            posted_after,
            posted_before,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    pub async fn list_financial_events_by_group_id(
        &self,
        event_group_id: &str,
        max_results_per_page: Option<i32>,
        posted_after: Option<String>,
        posted_before: Option<String>,
        next_token: Option<&str>,
    ) -> Result<models::finances_v0::ListFinancialEventsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/finances/v0/financialEventsByGroupId", 0.5, 30)
            .await?;
        let res = crate::apis::finances_v0::list_financial_events_by_group_id(
            &configuration,
            event_group_id,
            max_results_per_page,
            posted_after,
            posted_before,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns all financial events for the specified financial event group. It may take up to 48 hours for orders to appear in your financial events.  **Note:** This operation will only retrieve group's data for the past two years. If a request is submitted for data spanning more than two years, an empty response is returned.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 30 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The table above indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may see higher rate and burst values than those shown here. For more information, see [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn list_financial_events_by_order_id(
        &self,
        order_id: &str,
        max_results_per_page: Option<i32>,
        next_token: Option<&str>,
    ) -> Result<models::finances_v0::ListFinancialEventsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/finances/v0/financialEventsByOrderId", 0.5, 30)
            .await?;
        let res = crate::apis::finances_v0::list_financial_events_by_order_id(
            &configuration,
            order_id,
            max_results_per_page,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
