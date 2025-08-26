use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// This operation gets an eligibility preview for an item that you specify. You can specify the type of eligibility preview that you want (INBOUND or COMMINGLING). For INBOUND previews, you can specify the marketplace in which you want to determine the item's eligibility.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 1 |
    pub async fn get_item_eligibility_preview(
        &self,
        asin: &str,
        program: &str,
        marketplace_ids: Option<Vec<String>>,
    ) -> Result<models::fba_inbound::GetItemEligibilityPreviewResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fba_inbound_eligibility_v1/get_item_eligibility_preview", 1.0, 1)
            .await?;
        let res = crate::apis::fba_inbound_eligibility_v1::get_item_eligibility_preview(
            &configuration,
            asin,
            program,
            marketplace_ids,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
