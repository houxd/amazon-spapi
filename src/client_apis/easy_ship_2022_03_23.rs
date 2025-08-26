use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Schedules an Easy Ship order and returns the scheduled package information.  This operation does the following:  *  Specifies the time slot and handover method for the order to be scheduled for delivery.  * Updates the Easy Ship order status.  * Generates a shipping label and an invoice. Calling `createScheduledPackage` also generates a warranty document if you specify a `SerialNumber` value. To get these documents, see [How to get invoice, shipping label, and warranty documents](doc:easyship-api-v2022-03-23-use-case-guide).  * Shows the status of Easy Ship orders when you call the `getOrders` operation of the Selling Partner API for Orders and examine the `EasyShipShipmentStatus` property in the response body.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_scheduled_package(
        &self,
        create_scheduled_package_request: models::easy_ship_2022_03_23::CreateScheduledPackageRequest,
    ) -> Result<models::easy_ship_2022_03_23::Package> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("easy_ship_2022_03_23/create_scheduled_package", 1.0, 5)
            .await?;
        let res = crate::apis::easy_ship_2022_03_23::create_scheduled_package(
            &configuration,
            create_scheduled_package_request,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// This operation automatically schedules a time slot for all the `amazonOrderId`s given as input, generating the associated shipping labels, along with other compliance documents according to the marketplace (refer to the [marketplace document support table](doc:easyship-api-v2022-03-23-use-case-guide#marketplace-support-table)).  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn create_scheduled_package_bulk(
        &self,
        create_scheduled_packages_request: models::easy_ship_2022_03_23::CreateScheduledPackagesRequest,
    ) -> Result<models::easy_ship_2022_03_23::CreateScheduledPackagesResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("easy_ship_2022_03_23/create_scheduled_package_bulk", 1.0, 5)
            .await?;
        let res = crate::apis::easy_ship_2022_03_23::create_scheduled_package_bulk(
            &configuration,
            create_scheduled_packages_request,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns information about a package, including dimensions, weight, time slot information for handover, invoice and item information, and status.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn get_scheduled_package(
        &self,
        amazon_order_id: &str,
        marketplace_id: &str,
    ) -> Result<models::easy_ship_2022_03_23::Package> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("easy_ship_2022_03_23/get_scheduled_package", 1.0, 5)
            .await?;
        let res = crate::apis::easy_ship_2022_03_23::get_scheduled_package(
            &configuration,
            amazon_order_id,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns time slots available for Easy Ship orders to be scheduled based on the package weight and dimensions that the seller specifies.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn list_handover_slots(
        &self,
        list_handover_slots_request: Option<models::easy_ship_2022_03_23::ListHandoverSlotsRequest>,
    ) -> Result<models::easy_ship_2022_03_23::ListHandoverSlotsResponse> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("easy_ship_2022_03_23/list_handover_slots", 1.0, 5)
            .await?;
        let res = crate::apis::easy_ship_2022_03_23::list_handover_slots(
            &configuration,
            list_handover_slots_request,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Updates the time slot for handing over the package indicated by the specified `scheduledPackageId`. You can get the new `slotId` value for the time slot by calling the `listHandoverSlots` operation before making another `patch` call.  **Usage Plan:**  | Rate (requests per second) | Burst | | ---- | ---- | | 1 | 5 |
    pub async fn update_scheduled_packages(
        &self,
        update_scheduled_packages_request: Option<models::easy_ship_2022_03_23::UpdateScheduledPackagesRequest>,
    ) -> Result<models::easy_ship_2022_03_23::Packages> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("easy_ship_2022_03_23/update_scheduled_packages", 1.0, 5)
            .await?;
        let res = crate::apis::easy_ship_2022_03_23::update_scheduled_packages(
            &configuration,
            update_scheduled_packages_request,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
