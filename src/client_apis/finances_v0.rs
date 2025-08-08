use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    pub async fn list_financial_event_groups(
        &self,
        max_results_per_page: Option<i32>,
        financial_event_group_started_before: Option<String>,
        financial_event_group_started_after: Option<String>,
        next_token: Option<&str>,
    ) -> Result<models::finances_v0::ListFinancialEventGroupsResponse> {
        let configuration = self.create_configuration().await?;
        let _ = self
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
        Ok(res)
    }

    /// max_results_per_page: 1 to 100 default to 100
    pub async fn list_financial_events(
        &self,
        max_results_per_page: Option<i32>,
        posted_after: Option<String>,
        posted_before: Option<String>,
        next_token: Option<&str>,
    ) -> Result<models::finances_v0::ListFinancialEventsResponse> {
        let configuration = self.create_configuration().await?;
        let _ = self
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
        let _ = self
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
        Ok(res)
    }

    pub async fn list_financial_events_by_order_id(
        &self,
        order_id: &str,
        max_results_per_page: Option<i32>,
        next_token: Option<&str>,
    ) -> Result<models::finances_v0::ListFinancialEventsResponse> {
        let configuration = self.create_configuration().await?;
        let _ = self
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
        Ok(res)
    }
}
