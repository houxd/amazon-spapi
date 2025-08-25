use anyhow::Result;

use crate::{apis::fba_inventory_v1::get_inventory_summaries, client::SpapiClient, models};

impl SpapiClient {
    /// Cancels an Inbound Plan. Charges may apply if the cancellation is performed outside of a void window. The window for Amazon Partnered Carriers is 24 hours for Small Parcel Delivery (SPD) and one hour for Less-Than-Truckload (LTL) carrier shipments.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |  The `x-amzn-RateLimit-Limit` response header returns the usage plan rate limits that were applied to the requested operation, when available. The preceding table contains the default rate and burst values for this operation. Selling partners whose business demands require higher throughput may have higher rate and burst values than those shown here. For more information, refer to [Usage Plans and Rate Limits in the Selling Partner API](https://developer-docs.amazon.com/sp-api/docs/usage-plans-and-rate-limits-in-the-sp-api).
    pub async fn cancel_inbound_plan(
        &self,
        inbound_plan_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::CancelInboundPlanResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_2024_03_20/cancel_inbound_plan", 2.0, 2)
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::cancel_inbound_plan(
            &configuration,
            inbound_plan_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
    
    /// Cancels a self-ship appointment slot against a shipment. Only available in the following [marketplaces](https://developer-docs.amazon.com/sp-api/docs/marketplace-ids): MX, BR, EG, SA, AE, IN.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn cancel_self_ship_appointment(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        body: models::fulfillment_inbound_2024_03_20::CancelSelfShipAppointmentRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::CancelSelfShipAppointmentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/cancel_self_ship_appointment",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::cancel_self_ship_appointment(
            &configuration,
            inbound_plan_id,
            shipment_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Confirms the delivery window option for chosen shipment within an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn confirm_delivery_window_options(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        delivery_window_option_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ConfirmDeliveryWindowOptionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/confirm_delivery_window_options",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::confirm_delivery_window_options(
            &configuration,
            inbound_plan_id,
            shipment_id,
            delivery_window_option_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Confirms the packing option for an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn confirm_packing_option(
        &self,
        inbound_plan_id: &str,
        packing_option_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ConfirmPackingOptionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/confirm_packing_option",
                2.0,
                2,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::confirm_packing_option(
            &configuration,
            inbound_plan_id,
            packing_option_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Confirms the placement option for an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn confirm_placement_option(
        &self,
        inbound_plan_id: &str,
        placement_option_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ConfirmPlacementOptionResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/confirm_placement_option",
                2.0,
                2,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::confirm_placement_option(
            &configuration,
            inbound_plan_id,
            placement_option_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Confirm a shipment content update preview and accept the changes in transportation cost.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn confirm_shipment_content_update_preview(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        content_update_preview_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ConfirmShipmentContentUpdatePreviewResponse>
    {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/confirm_shipment_content_update_preview",
                2.0,
                30,
            )
            .await?;
        let res =
            crate::apis::fulfillment_inbound_2024_03_20::confirm_shipment_content_update_preview(
                &configuration,
                inbound_plan_id,
                shipment_id,
                content_update_preview_id,
            )
            .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Confirms all the transportation options for an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn confirm_transportation_options(
        &self,
        inbound_plan_id: &str,
        body: models::fulfillment_inbound_2024_03_20::ConfirmTransportationOptionsRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ConfirmTransportationOptionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/confirm_transportation_options",
                2.0,
                2,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::confirm_transportation_options(
            &configuration,
            inbound_plan_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Creates an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn create_inbound_plan(
        &self,
        body: models::fulfillment_inbound_2024_03_20::CreateInboundPlanRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::CreateInboundPlanResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_2024_03_20/create_inbound_plan", 2.0, 2)
            .await?;
        let res =
            crate::apis::fulfillment_inbound_2024_03_20::create_inbound_plan(&configuration, body)
                .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// For a given marketplace - creates labels for a list of MSKUs.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn create_marketplace_item_labels(
        &self,
        body: models::fulfillment_inbound_2024_03_20::CreateMarketplaceItemLabelsRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::CreateMarketplaceItemLabelsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/create_marketplace_item_labels",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::create_marketplace_item_labels(
            &configuration,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Generates available delivery window options for a given shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn generate_delivery_window_options(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::GenerateDeliveryWindowOptionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/generate_delivery_window_options",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::generate_delivery_window_options(
            &configuration,
            inbound_plan_id,
            shipment_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Generates available packing options for the inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn generate_packing_options(
        &self,
        inbound_plan_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::GeneratePackingOptionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/generate_packing_options",
                2.0,
                2,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::generate_packing_options(
            &configuration,
            inbound_plan_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Generates placement options for the inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn generate_placement_options(
        &self,
        inbound_plan_id: &str,
        body: models::fulfillment_inbound_2024_03_20::GeneratePlacementOptionsRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::GeneratePlacementOptionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/generate_placement_options",
                2.0,
                2,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::generate_placement_options(
            &configuration,
            inbound_plan_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Initiates the process of generating the appointment slots list.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn generate_self_ship_appointment_slots(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        body: models::fulfillment_inbound_2024_03_20::GenerateSelfShipAppointmentSlotsRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::GenerateSelfShipAppointmentSlotsResponse>
    {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/generate_self_ship_appointment_slots",
                2.0,
                2,
            )
            .await?;
        let res =
            crate::apis::fulfillment_inbound_2024_03_20::generate_self_ship_appointment_slots(
                &configuration,
                inbound_plan_id,
                shipment_id,
                body,
            )
            .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Generate a shipment content update preview.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn generate_shipment_content_update_previews(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        body: models::fulfillment_inbound_2024_03_20::GenerateShipmentContentUpdatePreviewsRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::GenerateShipmentContentUpdatePreviewsResponse>
    {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/generate_shipment_content_update_previews",
                2.0,
                30,
            )
            .await?;
        let res =
            crate::apis::fulfillment_inbound_2024_03_20::generate_shipment_content_update_previews(
                &configuration,
                inbound_plan_id,
                shipment_id,
                body,
            )
            .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Generates available transportation options for a given placement option.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn generate_transportation_options(
        &self,
        inbound_plan_id: &str,
        body: models::fulfillment_inbound_2024_03_20::GenerateTransportationOptionsRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::GenerateTransportationOptionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/generate_transportation_options",
                2.0,
                2,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::generate_transportation_options(
            &configuration,
            inbound_plan_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provide delivery challan document for PCP transportation in IN marketplace.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn get_delivery_challan_document(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::GetDeliveryChallanDocumentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/get_delivery_challan_document",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::get_delivery_challan_document(
            &configuration,
            inbound_plan_id,
            shipment_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Gets the status of the processing of an asynchronous API call.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn get_inbound_operation_status(
        &self,
        operation_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::InboundOperationStatus> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/get_inbound_operation_status",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::get_inbound_operation_status(
            &configuration,
            operation_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Fetches the top level information about an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn get_inbound_plan(
        &self,
        inbound_plan_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::InboundPlan> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_2024_03_20/get_inbound_plan", 2.0, 6)
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::get_inbound_plan(
            &configuration,
            inbound_plan_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves a list of available self-ship appointment slots.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn get_self_ship_appointment_slots(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::GetSelfShipAppointmentSlotsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/get_self_ship_appointment_slots",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::get_self_ship_appointment_slots(
            &configuration,
            inbound_plan_id,
            shipment_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides the full details for a specific shipment within an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn get_shipment(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::Shipment> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_2024_03_20/get_shipment", 2.0, 6)
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::get_shipment(
            &configuration,
            inbound_plan_id,
            shipment_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieve a shipment content update preview.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn get_shipment_content_update_preview(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        content_update_preview_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ContentUpdatePreview> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/get_shipment_content_update_preview",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::get_shipment_content_update_preview(
            &configuration,
            inbound_plan_id,
            shipment_id,
            content_update_preview_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves all delivery window options for a shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn list_delivery_window_options(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListDeliveryWindowOptionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_delivery_window_options",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_delivery_window_options(
            &configuration,
            inbound_plan_id,
            shipment_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides a paginated list of box packages in an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn list_inbound_plan_boxes(
        &self,
        inbound_plan_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListInboundPlanBoxesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_inbound_plan_boxes",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_inbound_plan_boxes(
            &configuration,
            inbound_plan_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides a paginated list of item packages in an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn list_inbound_plan_items(
        &self,
        inbound_plan_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListInboundPlanItemsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_inbound_plan_items",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_inbound_plan_items(
            &configuration,
            inbound_plan_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides a paginated list of pallet packages in an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn list_inbound_plan_pallets(
        &self,
        inbound_plan_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListInboundPlanPalletsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_inbound_plan_pallets",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_inbound_plan_pallets(
            &configuration,
            inbound_plan_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides a list of inbound plans with minimal information.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn list_inbound_plans(
        &self,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
        status: Option<&str>,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListInboundPlansResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_2024_03_20/list_inbound_plans", 2.0, 6)
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_inbound_plans(
            &configuration,
            page_size,
            pagination_token,
            status,
            sort_by,
            sort_order,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// List the inbound compliance details for MSKUs in a given marketplace.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn list_item_compliance_details(
        &self,
        mskus: Vec<String>,
        marketplace_id: &str,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListItemComplianceDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_item_compliance_details",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_item_compliance_details(
            &configuration,
            mskus,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves a page of boxes from a given packing group.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn list_packing_group_boxes(
        &self,
        inbound_plan_id: &str,
        packing_group_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListPackingGroupBoxesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_packing_group_boxes",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_packing_group_boxes(
            &configuration,
            inbound_plan_id,
            packing_group_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves a page of items in a given packing group.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn list_packing_group_items(
        &self,
        inbound_plan_id: &str,
        packing_group_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListPackingGroupItemsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_packing_group_items",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_packing_group_items(
            &configuration,
            inbound_plan_id,
            packing_group_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves a list of all packing options for an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn list_packing_options(
        &self,
        inbound_plan_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListPackingOptionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_packing_options",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_packing_options(
            &configuration,
            inbound_plan_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides a list of all placement options for an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn list_placement_options(
        &self,
        inbound_plan_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListPlacementOptionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_placement_options",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_placement_options(
            &configuration,
            inbound_plan_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Get preparation details for a list of MSKUs in a specified marketplace.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn list_prep_details(
        &self,
        marketplace_id: &str,
        mskus: Vec<String>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListPrepDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_2024_03_20/list_prep_details", 2.0, 30)
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_prep_details(
            &configuration,
            marketplace_id,
            mskus,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides a paginated list of box packages in a shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn list_shipment_boxes(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListShipmentBoxesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_shipment_boxes",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_shipment_boxes(
            &configuration,
            inbound_plan_id,
            shipment_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieve a paginated list of shipment content update previews.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn list_shipment_content_update_previews(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListShipmentContentUpdatePreviewsResponse>
    {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_shipment_content_update_previews",
                2.0,
                30,
            )
            .await?;
        let res =
            crate::apis::fulfillment_inbound_2024_03_20::list_shipment_content_update_previews(
                &configuration,
                inbound_plan_id,
                shipment_id,
                page_size,
                pagination_token,
            )
            .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides a paginated list of item packages in a shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn list_shipment_items(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListShipmentItemsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_shipment_items",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_shipment_items(
            &configuration,
            inbound_plan_id,
            shipment_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Provides a paginated list of pallet packages in a shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn list_shipment_pallets(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListShipmentPalletsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_shipment_pallets",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_shipment_pallets(
            &configuration,
            inbound_plan_id,
            shipment_id,
            page_size,
            pagination_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieves all transportation options for a shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn list_transportation_options(
        &self,
        inbound_plan_id: &str,
        page_size: Option<i32>,
        pagination_token: Option<&str>,
        placement_option_id: Option<&str>,
        shipment_id: Option<&str>,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ListTransportationOptionsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/list_transportation_options",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::list_transportation_options(
            &configuration,
            inbound_plan_id,
            page_size,
            pagination_token,
            placement_option_id,
            shipment_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Confirms or reschedules a self-ship appointment slot against a shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn schedule_self_ship_appointment(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        slot_id: &str,
        body: models::fulfillment_inbound_2024_03_20::ScheduleSelfShipAppointmentRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::ScheduleSelfShipAppointmentResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/schedule_self_ship_appointment",
                2.0,
                2,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::schedule_self_ship_appointment(
            &configuration,
            inbound_plan_id,
            shipment_id,
            slot_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Sets packing information for an inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn set_packing_information(
        &self,
        inbound_plan_id: &str,
        body: models::fulfillment_inbound_2024_03_20::SetPackingInformationRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::SetPackingInformationResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/set_packing_information",
                2.0,
                2,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::set_packing_information(
            &configuration,
            inbound_plan_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Set the preparation details for a list of MSKUs in a specified marketplace.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn set_prep_details(
        &self,
        body: models::fulfillment_inbound_2024_03_20::SetPrepDetailsRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::SetPrepDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("fulfillment_inbound_2024_03_20/set_prep_details", 2.0, 30)
            .await?;
        let res =
            crate::apis::fulfillment_inbound_2024_03_20::set_prep_details(&configuration, body)
                .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates the name of an existing inbound plan.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn update_inbound_plan_name(
        &self,
        inbound_plan_id: &str,
        body: models::fulfillment_inbound_2024_03_20::UpdateInboundPlanNameRequest,
    ) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/update_inbound_plan_name",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::update_inbound_plan_name(
            &configuration,
            inbound_plan_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Update compliance details for a list of MSKUs.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 6 |
    pub async fn update_item_compliance_details(
        &self,
        marketplace_id: &str,
        body: models::fulfillment_inbound_2024_03_20::UpdateItemComplianceDetailsRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::UpdateItemComplianceDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/update_item_compliance_details",
                2.0,
                6,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::update_item_compliance_details(
            &configuration,
            marketplace_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates the name of an existing shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn update_shipment_name(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        body: models::fulfillment_inbound_2024_03_20::UpdateShipmentNameRequest,
    ) -> Result<()> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/update_shipment_name",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::update_shipment_name(
            &configuration,
            inbound_plan_id,
            shipment_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates the source address of an existing shipment.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 30 |
    pub async fn update_shipment_source_address(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        body: models::fulfillment_inbound_2024_03_20::UpdateShipmentSourceAddressRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::UpdateShipmentSourceAddressResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/update_shipment_source_address",
                2.0,
                30,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::update_shipment_source_address(
            &configuration,
            inbound_plan_id,
            shipment_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates a shipment's tracking details.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 2 | 2 |
    pub async fn update_shipment_tracking_details(
        &self,
        inbound_plan_id: &str,
        shipment_id: &str,
        body: models::fulfillment_inbound_2024_03_20::UpdateShipmentTrackingDetailsRequest,
    ) -> Result<models::fulfillment_inbound_2024_03_20::UpdateShipmentTrackingDetailsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait(
                "fulfillment_inbound_2024_03_20/update_shipment_tracking_details",
                2.0,
                2,
            )
            .await?;
        let res = crate::apis::fulfillment_inbound_2024_03_20::update_shipment_tracking_details(
            &configuration,
            inbound_plan_id,
            shipment_id,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
