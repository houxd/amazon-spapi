use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Creates a new A+ Content document.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |
    pub async fn create_content_document(
        &self,
        marketplace_id: &str,
        post_content_document_request: models::aplus_content_2020_11_01::PostContentDocumentRequest,
    ) -> Result<models::aplus_content_2020_11_01::PostContentDocumentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("aplus_content_2020_11_01/create_content_document", 10.0, 10)
            .await?;
        let res = crate::apis::aplus_content_2020_11_01::create_content_document(
            &configuration,
            marketplace_id,
            post_content_document_request,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns an A+ Content document, if available.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |
    pub async fn get_content_document(
        &self,
        content_reference_key: &str,
        marketplace_id: &str,
        included_data_set: Vec<String>,
    ) -> Result<models::aplus_content_2020_11_01::GetContentDocumentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("aplus_content_2020_11_01/get_content_document", 10.0, 10)
            .await?;
        let res = crate::apis::aplus_content_2020_11_01::get_content_document(
            &configuration,
            content_reference_key,
            marketplace_id,
            included_data_set,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of ASINs that are related to the specified A+ Content document, if available. If you don't include the `asinSet` parameter, this operation returns all ASINs related to the content document.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |
    pub async fn list_content_document_asin_relations(
        &self,
        content_reference_key: &str,
        marketplace_id: &str,
        included_data_set: Option<Vec<String>>,
        asin_set: Option<Vec<String>>,
        page_token: Option<&str>,
    ) -> Result<models::aplus_content_2020_11_01::ListContentDocumentAsinRelationsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("aplus_content_2020_11_01/list_content_document_asin_relations", 10.0, 10)
            .await?;
        let res = crate::apis::aplus_content_2020_11_01::list_content_document_asin_relations(
            &configuration,
            content_reference_key,
            marketplace_id,
            included_data_set,
            asin_set,
            page_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Submits an A+ Content document for review, approval, and publishing.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |
    pub async fn post_content_document_approval_submission(
        &self,
        content_reference_key: &str,
        marketplace_id: &str,
    ) -> Result<models::aplus_content_2020_11_01::PostContentDocumentApprovalSubmissionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("aplus_content_2020_11_01/post_content_document_approval_submission", 10.0, 10)
            .await?;
        let res = crate::apis::aplus_content_2020_11_01::post_content_document_approval_submission(
            &configuration,
            content_reference_key,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Replaces all ASINs related to the specified A+ Content document, if available. This operation can add or remove ASINs, depending on the current set of related ASINs. Removing an ASIN will suspend the content document from that ASIN.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |
    pub async fn post_content_document_asin_relations(
        &self,
        content_reference_key: &str,
        marketplace_id: &str,
        post_content_document_asin_relations_request: models::aplus_content_2020_11_01::PostContentDocumentAsinRelationsRequest,
    ) -> Result<models::aplus_content_2020_11_01::PostContentDocumentAsinRelationsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("aplus_content_2020_11_01/post_content_document_asin_relations", 10.0, 10)
            .await?;
        let res = crate::apis::aplus_content_2020_11_01::post_content_document_asin_relations(
            &configuration,
            content_reference_key,
            marketplace_id,
            post_content_document_asin_relations_request,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Submits a request to suspend visible A+ Content. This doesn't delete the content document or the ASIN relations.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |
    pub async fn post_content_document_suspend_submission(
        &self,
        content_reference_key: &str,
        marketplace_id: &str,
    ) -> Result<models::aplus_content_2020_11_01::PostContentDocumentSuspendSubmissionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("aplus_content_2020_11_01/post_content_document_suspend_submission", 10.0, 10)
            .await?;
        let res = crate::apis::aplus_content_2020_11_01::post_content_document_suspend_submission(
            &configuration,
            content_reference_key,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of all A+ Content documents, including metadata, that are assigned to a selling partner. To get the actual contents of the A+ Content documents, call the `getContentDocument` operation.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |
    pub async fn search_content_documents(
        &self,
        marketplace_id: &str,
        page_token: Option<&str>,
    ) -> Result<models::aplus_content_2020_11_01::SearchContentDocumentsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("aplus_content_2020_11_01/search_content_documents", 10.0, 10)
            .await?;
        let res = crate::apis::aplus_content_2020_11_01::search_content_documents(
            &configuration,
            marketplace_id,
            page_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Searches for A+ Content publishing records, if available.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |
    pub async fn search_content_publish_records(
        &self,
        marketplace_id: &str,
        asin: &str,
        page_token: Option<&str>,
    ) -> Result<models::aplus_content_2020_11_01::SearchContentPublishRecordsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("aplus_content_2020_11_01/search_content_publish_records", 10.0, 10)
            .await?;
        let res = crate::apis::aplus_content_2020_11_01::search_content_publish_records(
            &configuration,
            marketplace_id,
            asin,
            page_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates an existing A+ Content document.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |
    pub async fn update_content_document(
        &self,
        content_reference_key: &str,
        marketplace_id: &str,
        post_content_document_request: models::aplus_content_2020_11_01::PostContentDocumentRequest,
    ) -> Result<models::aplus_content_2020_11_01::PostContentDocumentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("aplus_content_2020_11_01/update_content_document", 10.0, 10)
            .await?;
        let res = crate::apis::aplus_content_2020_11_01::update_content_document(
            &configuration,
            content_reference_key,
            marketplace_id,
            post_content_document_request,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Checks if the A+ Content document is valid for use on a set of ASINs.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 10 | 10 |
    pub async fn validate_content_document_asin_relations(
        &self,
        marketplace_id: &str,
        post_content_document_request: models::aplus_content_2020_11_01::PostContentDocumentRequest,
        asin_set: Option<Vec<String>>,
    ) -> Result<models::aplus_content_2020_11_01::ValidateContentDocumentAsinRelationsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("aplus_content_2020_11_01/validate_content_document_asin_relations", 10.0, 10)
            .await?;
        let res = crate::apis::aplus_content_2020_11_01::validate_content_document_asin_relations(
            &configuration,
            marketplace_id,
            post_content_document_request,
            asin_set,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}