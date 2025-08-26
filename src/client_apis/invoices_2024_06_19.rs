use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Creates an invoice export request.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.167 | 1 |
    pub async fn create_invoices_export(
        &self,
        body: models::invoices_2024_06_19::ExportInvoicesRequest,
    ) -> Result<models::invoices_2024_06_19::ExportInvoicesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("invoices_2024_06_19/create_invoices_export", 0.167, 1)
            .await?;
        let res = crate::apis::invoices_2024_06_19::create_invoices_export(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns invoice data for the specified invoice. This operation returns only a subset of the invoices data; refer to the response definition to get all the possible attributes. To get the full invoice, use the `createInvoicesExport` operation to start an export request.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 15 |
    pub async fn get_invoice(
        &self,
        marketplace_id: &str,
        invoice_id: &str,
    ) -> Result<models::invoices_2024_06_19::GetInvoiceResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("invoices_2024_06_19/get_invoice", 2.0, 15)
            .await?;
        let res = crate::apis::invoices_2024_06_19::get_invoice(
            &configuration,
            marketplace_id,
            invoice_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns invoice details for the invoices that match the filters that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.1 | 20 |
    pub async fn get_invoices(
        &self,
        marketplace_id: &str,
        transaction_identifier_name: Option<&str>,
        page_size: Option<i32>,
        date_end: Option<String>,
        transaction_type: Option<&str>,
        transaction_identifier_id: Option<&str>,
        date_start: Option<String>,
        series: Option<&str>,
        next_token: Option<&str>,
        sort_order: Option<&str>,
        invoice_type: Option<&str>,
        statuses: Option<Vec<String>>,
        external_invoice_id: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<models::invoices_2024_06_19::GetInvoicesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("invoices_2024_06_19/get_invoices", 0.1, 20)
            .await?;
        let res = crate::apis::invoices_2024_06_19::get_invoices(
            &configuration,
            marketplace_id,
            transaction_identifier_name,
            page_size,
            date_end,
            transaction_type,
            transaction_identifier_id,
            date_start,
            series,
            next_token,
            sort_order,
            invoice_type,
            statuses,
            external_invoice_id,
            sort_by,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns marketplace-dependent schemas and their respective set of possible values.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn get_invoices_attributes(
        &self,
        marketplace_id: &str,
    ) -> Result<models::invoices_2024_06_19::GetInvoicesAttributesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("invoices_2024_06_19/get_invoices_attributes", 1.0, 1)
            .await?;
        let res = crate::apis::invoices_2024_06_19::get_invoices_attributes(
            &configuration,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns the invoice document's ID and URL. Use the URL to download the ZIP file, which contains the invoices from the corresponding `createInvoicesExport` request.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.0167 | 1 |
    pub async fn get_invoices_document(
        &self,
        invoices_document_id: &str,
    ) -> Result<models::invoices_2024_06_19::GetInvoicesDocumentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("invoices_2024_06_19/get_invoices_document", 0.0167, 1)
            .await?;
        let res = crate::apis::invoices_2024_06_19::get_invoices_document(
            &configuration,
            invoices_document_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns invoice export details (including the `exportDocumentId`, if available) for the export that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 15 |
    pub async fn get_invoices_export(
        &self,
        export_id: &str,
    ) -> Result<models::invoices_2024_06_19::GetInvoicesExportResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("invoices_2024_06_19/get_invoices_export", 2.0, 15)
            .await?;
        let res = crate::apis::invoices_2024_06_19::get_invoices_export(
            &configuration,
            export_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns invoice exports details for exports that match the filters that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 0.1 | 20 |
    pub async fn get_invoices_exports(
        &self,
        marketplace_id: &str,
        date_start: Option<String>,
        next_token: Option<&str>,
        page_size: Option<i32>,
        date_end: Option<String>,
        status: Option<&str>,
    ) -> Result<models::invoices_2024_06_19::GetInvoicesExportsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("invoices_2024_06_19/get_invoices_exports", 0.1, 20)
            .await?;
        let res = crate::apis::invoices_2024_06_19::get_invoices_exports(
            &configuration,
            marketplace_id,
            date_start,
            next_token,
            page_size,
            date_end,
            status,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
