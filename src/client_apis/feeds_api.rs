use crate::{apis, client::SpapiClient, models};
use anyhow::Result;

impl SpapiClient {
    /// Cancels the feed that you specify. Only feeds with `processingStatus=IN_QUEUE` can be cancelled. Cancelled feeds are returned in subsequent calls to the [`getFeed`](https://developer-docs.amazon.com/sp-api/reference/getfeed) and [`getFeeds`](https://developer-docs.amazon.com/sp-api/reference/getfeeds) operations.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 15 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn cancel_feed(&self, feed_id: &str) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/feeds/2021-06-30/cancel_feed", 2.0, 15)
            .await?;
        let res = apis::feeds_api::cancel_feed(&configuration, feed_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Creates a feed. Upload the contents of the feed document before calling this operation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0083 | 15 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).  The rate limit for the [`JSON_LISTINGS_FEED`](https://developer-docs.amazon.com/sp-api/docs/listings-feed-type-values#listings-feed) feed type differs from the rate limit for the [`createFeed`](https://developer-docs.amazon.com/sp-api/reference/createfeed) operation. For more information, refer to the [Building Listings Management Workflows Guide](https://developer-docs.amazon.com/sp-api/docs/building-listings-management-workflows-guide#should-i-submit-in-bulk-using-the-json_listings_feed-or-individually-with-the-listings-items-api).
    pub async fn create_feed(
        &self,
        body: models::feeds_2021_06_30::CreateFeedSpecification,
    ) -> Result<models::feeds_2021_06_30::CreateFeedResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/feeds/2021-06-30/create_feed", 0.0083, 15)
            .await?;
        let res = apis::feeds_api::create_feed(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Creates a feed document for the feed type that you specify. This operation returns a presigned URL for uploading the feed document contents. It also returns a `feedDocumentId` value that you can pass in with a subsequent call to the [`createFeed`](https://developer-docs.amazon.com/sp-api/reference/createfeed) operation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.5 | 15 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn create_feed_document(
        &self,
        body: models::feeds_2021_06_30::CreateFeedDocumentSpecification,
    ) -> Result<models::feeds_2021_06_30::CreateFeedDocumentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/feeds/2021-06-30/create_feed_document", 0.5, 15)
            .await?;
        let res = apis::feeds_api::create_feed_document(&configuration, body).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns feed details (including the `resultDocumentId`, if available) for the feed that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 15 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_feed(&self, feed_id: &str) -> Result<models::feeds_2021_06_30::Feed> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/feeds/2021-06-30/get_feed", 2.0, 15)
            .await?;
        let res = apis::feeds_api::get_feed(&configuration, feed_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the information required for retrieving a feed document's contents.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0222 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_feed_document(
        &self,
        feed_document_id: &str,
    ) -> Result<models::feeds_2021_06_30::FeedDocument> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/feeds/2021-06-30/get_feed_document", 0.0222, 10)
            .await?;
        let res = apis::feeds_api::get_feed_document(&configuration, feed_document_id).await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns feed details for the feeds that match the filters that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0222 | 10 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table indicates the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn get_feeds(
        &self,
        feed_types: Option<Vec<String>>,
        marketplace_ids: Option<Vec<String>>,
        page_size: Option<i32>,
        processing_statuses: Option<Vec<String>>,
        created_since: Option<String>,
        created_until: Option<String>,
        next_token: Option<&str>,
    ) -> Result<models::feeds_2021_06_30::GetFeedsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("/feeds/2021-06-30/get_feeds", 0.0222, 10)
            .await?;
        let res = apis::feeds_api::get_feeds(
            &configuration,
            feed_types,
            marketplace_ids,
            page_size,
            processing_statuses,
            created_since,
            created_until,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
