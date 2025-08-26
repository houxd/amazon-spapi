use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Cancels the query specified by the `queryId` parameter. Only queries with a non-terminal `processingStatus` (`IN_QUEUE`, `IN_PROGRESS`) can be cancelled. Cancelling a query that already has a `processingStatus` of `CANCELLED` will no-op. Cancelled queries are returned in subsequent calls to the `getQuery` and `getQueries` operations.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0222 | 10 |
    pub async fn cancel_query(
        &self,
        query_id: &str,
    ) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("data_kiosk_2023_11_15/cancel_query", 0.0222, 10)
            .await?;
        let res = crate::apis::data_kiosk_2023_11_15::cancel_query(
            &configuration,
            query_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Creates a Data Kiosk query request.  **Note:** The retention of a query varies based on the fields requested. Each field within a schema is annotated with a `@resultRetention` directive that defines how long a query containing that field will be retained. When a query contains multiple fields with different retentions, the shortest (minimum) retention is applied. The retention of a query's resulting documents always matches the retention of the query.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0167 | 15 |
    pub async fn create_query(
        &self,
        body: models::data_kiosk_2023_11_15::CreateQuerySpecification,
    ) -> Result<models::data_kiosk_2023_11_15::CreateQueryResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("data_kiosk_2023_11_15/create_query", 0.0167, 15)
            .await?;
        let res = crate::apis::data_kiosk_2023_11_15::create_query(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the information required for retrieving a Data Kiosk document's contents. See the `createQuery` operation for details about document retention.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0167 | 15 |
    pub async fn get_document(
        &self,
        document_id: &str,
    ) -> Result<models::data_kiosk_2023_11_15::GetDocumentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("data_kiosk_2023_11_15/get_document", 0.0167, 15)
            .await?;
        let res = crate::apis::data_kiosk_2023_11_15::get_document(
            &configuration,
            document_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns details for the Data Kiosk queries that match the specified filters. See the `createQuery` operation for details about query retention.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0222 | 10 |
    pub async fn get_queries(
        &self,
        processing_statuses: Option<Vec<String>>,
        page_size: Option<i32>,
        created_since: Option<String>,
        created_until: Option<String>,
        pagination_token: Option<&str>,
    ) -> Result<models::data_kiosk_2023_11_15::GetQueriesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("data_kiosk_2023_11_15/get_queries", 0.0222, 10)
            .await?;
        let res = crate::apis::data_kiosk_2023_11_15::get_queries(
            &configuration,
            processing_statuses,
            page_size,
            created_since,
            created_until,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns query details for the query specified by the `queryId` parameter. See the `createQuery` operation for details about query retention.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2.0 | 15 |
    pub async fn get_query(
        &self,
        query_id: &str,
    ) -> Result<models::data_kiosk_2023_11_15::Query> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("data_kiosk_2023_11_15/get_query", 2.0, 15)
            .await?;
        let res = crate::apis::data_kiosk_2023_11_15::get_query(
            &configuration,
            query_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
