use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Retrieve the topics that customers mention when they return items in a browse node.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_browse_node_return_topics(
        &self,
        browse_node_id: &str,
        marketplace_id: &str,
    ) -> Result<models::customer_feedback_2024_06_01::BrowseNodeReturnTopicsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("customer_feedback_2024_06_01/get_browse_node_return_topics", 1.0, 5)
            .await?;
        let res = crate::apis::customer_feedback_2024_06_01::get_browse_node_return_topics(
            &configuration,
            browse_node_id,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieve the trends of topics that customers mention when they return items in a browse node.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_browse_node_return_trends(
        &self,
        browse_node_id: &str,
        marketplace_id: &str,
    ) -> Result<models::customer_feedback_2024_06_01::BrowseNodeReturnTrendsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("customer_feedback_2024_06_01/get_browse_node_return_trends", 1.0, 5)
            .await?;
        let res = crate::apis::customer_feedback_2024_06_01::get_browse_node_return_trends(
            &configuration,
            browse_node_id,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieve a browse node's ten most positive and ten most negative review topics.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_browse_node_review_topics(
        &self,
        browse_node_id: &str,
        marketplace_id: &str,
        sort_by: &str,
    ) -> Result<models::customer_feedback_2024_06_01::BrowseNodeReviewTopicsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("customer_feedback_2024_06_01/get_browse_node_review_topics", 1.0, 5)
            .await?;
        let res = crate::apis::customer_feedback_2024_06_01::get_browse_node_review_topics(
            &configuration,
            browse_node_id,
            marketplace_id,
            sort_by,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieve the positive and negative review trends of items in a browse node for the past six months.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_browse_node_review_trends(
        &self,
        browse_node_id: &str,
        marketplace_id: &str,
    ) -> Result<models::customer_feedback_2024_06_01::BrowseNodeReviewTrendsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("customer_feedback_2024_06_01/get_browse_node_review_trends", 1.0, 5)
            .await?;
        let res = crate::apis::customer_feedback_2024_06_01::get_browse_node_review_trends(
            &configuration,
            browse_node_id,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// This API returns the associated browse node of the requested ASIN. A browse node is a location in a browse tree that is used for navigation, product classification, and website content on the Amazon retail website.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_item_browse_node(
        &self,
        asin: &str,
        marketplace_id: &str,
    ) -> Result<models::customer_feedback_2024_06_01::BrowseNodeResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("customer_feedback_2024_06_01/get_item_browse_node", 1.0, 5)
            .await?;
        let res = crate::apis::customer_feedback_2024_06_01::get_item_browse_node(
            &configuration,
            asin,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieve an item's ten most positive and ten most negative review topics.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_item_review_topics(
        &self,
        asin: &str,
        marketplace_id: &str,
        sort_by: &str,
    ) -> Result<models::customer_feedback_2024_06_01::ItemReviewTopicsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("customer_feedback_2024_06_01/get_item_review_topics", 1.0, 5)
            .await?;
        let res = crate::apis::customer_feedback_2024_06_01::get_item_review_topics(
            &configuration,
            asin,
            marketplace_id,
            sort_by,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieve an item's positive and negative review trends for the past six months.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_item_review_trends(
        &self,
        asin: &str,
        marketplace_id: &str,
    ) -> Result<models::customer_feedback_2024_06_01::ItemReviewTrendsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("customer_feedback_2024_06_01/get_item_review_trends", 1.0, 5)
            .await?;
        let res = crate::apis::customer_feedback_2024_06_01::get_item_review_trends(
            &configuration,
            asin,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
