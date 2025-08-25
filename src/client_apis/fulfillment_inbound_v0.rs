use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Returns a bill of lading for a Less Than Truckload/Full Truckload (LTL/FTL) shipment. The getBillOfLading operation returns PDF document data for printing a bill of lading for an Amazon-partnered Less Than Truckload/Full Truckload (LTL/FTL) inbound shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_bill_of_lading(
        &self,
        shipment_id: &str,
    ) -> Result<models::fulfillment_inbound_v0::GetBillOfLadingResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_v0/get_bill_of_lading", 2.0, 30)
            .await?;
        let res = crate::apis::fulfillment_inbound_v0::get_bill_of_lading(
            &configuration,
            shipment_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns package/pallet labels for faster and more accurate shipment processing at the Amazon fulfillment center.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_labels(
        &self,
        shipment_id: &str,
        page_type: &str,
        label_type: &str,
        number_of_packages: Option<i32>,
        package_labels_to_print: Option<Vec<String>>,
        number_of_pallets: Option<i32>,
        page_size: Option<i32>,
        page_start_index: Option<i32>,
    ) -> Result<models::fulfillment_inbound_v0::GetLabelsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_v0/get_labels", 2.0, 30)
            .await?;
        let res = crate::apis::fulfillment_inbound_v0::get_labels(
            &configuration,
            shipment_id,
            page_type,
            label_type,
            number_of_packages,
            package_labels_to_print,
            number_of_pallets,
            page_size,
            page_start_index,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns labeling requirements and item preparation instructions to help prepare items for shipment to Amazon's fulfillment network.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_prep_instructions(
        &self,
        ship_to_country_code: &str,
        seller_sku_list: Option<Vec<String>>,
        asin_list: Option<Vec<String>>,
    ) -> Result<models::fulfillment_inbound_v0::GetPrepInstructionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_v0/get_prep_instructions", 2.0, 30)
            .await?;
        let res = crate::apis::fulfillment_inbound_v0::get_prep_instructions(
            &configuration,
            ship_to_country_code,
            seller_sku_list,
            asin_list,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of items in a specified inbound shipment, or a list of items that were updated within a specified time frame.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_shipment_items(
        &self,
        query_type: &str,
        marketplace_id: &str,
        last_updated_after: Option<String>,
        last_updated_before: Option<String>,
        next_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_v0::GetShipmentItemsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_v0/get_shipment_items", 2.0, 30)
            .await?;
        let res = crate::apis::fulfillment_inbound_v0::get_shipment_items(
            &configuration,
            query_type,
            marketplace_id,
            last_updated_after,
            last_updated_before,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of items in a specified inbound shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_shipment_items_by_shipment_id(
        &self,
        shipment_id: &str,
        marketplace_id: Option<&str>,
    ) -> Result<models::fulfillment_inbound_v0::GetShipmentItemsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_v0/get_shipment_items_by_shipment_id", 2.0, 30)
            .await?;
        let res = crate::apis::fulfillment_inbound_v0::get_shipment_items_by_shipment_id(
            &configuration,
            shipment_id,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a list of inbound shipments based on criteria that you specify.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_shipments(
        &self,
        query_type: &str,
        marketplace_id: &str,
        shipment_status_list: Option<Vec<String>>,
        shipment_id_list: Option<Vec<String>>,
        last_updated_after: Option<String>,
        last_updated_before: Option<String>,
        next_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_v0::GetShipmentsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_v0/get_shipments", 2.0, 30)
            .await?;
        let res = crate::apis::fulfillment_inbound_v0::get_shipments(
            &configuration,
            query_type,
            marketplace_id,
            shipment_status_list,
            shipment_id_list,
            last_updated_after,
            last_updated_before,
            next_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}