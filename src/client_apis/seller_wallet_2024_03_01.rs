use anyhow::Result;

use crate::{client::SpapiClient, models};

impl SpapiClient {
    /// Retrieve a Seller Wallet bank account by Amazon account identifier.
    pub async fn get_account_seller_wallet_2024_03_01(
        &self,
        account_id: &str,
    ) -> Result<models::seller_wallet_2024_03_01::BankAccount> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/get_account", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::get_account(
            &configuration,
            account_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieve the balance in a given Seller Wallet bank account.
    pub async fn list_account_balances(
        &self,
        account_id: &str,
    ) -> Result<models::seller_wallet_2024_03_01::BalanceListing> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/list_account_balances", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::list_account_balances(
            &configuration,
            account_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Get Seller Wallet accounts for a seller.
    pub async fn list_accounts(
        &self,
        marketplace_id: &str,
    ) -> Result<models::seller_wallet_2024_03_01::BankAccountListing> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/list_accounts", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::list_accounts(
            &configuration,
            marketplace_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Create a transaction request from a Seller Wallet account to another customer-provided account.
    pub async fn create_transaction(
        &self,
        dest_account_digital_signature: &str,
        amount_digital_signature: &str,
        body: models::seller_wallet_2024_03_01::TransactionInitiationRequest,
    ) -> Result<models::seller_wallet_2024_03_01::Transaction> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/create_transaction", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::create_transaction(
            &configuration,
            dest_account_digital_signature,
            amount_digital_signature,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a transaction
    pub async fn get_transaction(
        &self,
        transaction_id: &str,
    ) -> Result<models::seller_wallet_2024_03_01::Transaction> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/get_transaction", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::get_transaction(
            &configuration,
            transaction_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieve a list of transactions for a given Seller Wallet bank account.
    pub async fn list_account_transactions(
        &self,
        account_id: &str,
        next_page_token: Option<&str>,
    ) -> Result<models::seller_wallet_2024_03_01::TransactionListing> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/list_account_transactions", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::list_account_transactions(
            &configuration,
            account_id,
            next_page_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns list of potential fees on a transaction based on the source and destination country currency code
    pub async fn get_transfer_preview(
        &self,
        source_country_code: &str,
        source_currency_code: &str,
        destination_country_code: &str,
        destination_currency_code: &str,
        base_amount: f64,
    ) -> Result<models::seller_wallet_2024_03_01::TransferRatePreview> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/get_transfer_preview", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::get_transfer_preview(
            &configuration,
            source_country_code,
            source_currency_code,
            destination_country_code,
            destination_currency_code,
            base_amount,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Create a transfer schedule request from a Seller Wallet account to another customer-provided account.
    pub async fn create_transfer_schedule(
        &self,
        dest_account_digital_signature: &str,
        amount_digital_signature: &str,
        body: models::seller_wallet_2024_03_01::TransferScheduleRequest,
    ) -> Result<models::seller_wallet_2024_03_01::TransferSchedule> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/create_transfer_schedule", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::create_transfer_schedule(
            &configuration,
            dest_account_digital_signature,
            amount_digital_signature,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Delete a transaction request that is scheduled from a Seller Wallet account to another customer-provided account.
    pub async fn delete_schedule_transaction(
        &self,
        transfer_schedule_id: &str,
    ) -> Result<models::seller_wallet_2024_03_01::DeleteTransferSchedule> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/delete_schedule_transaction", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::delete_schedule_transaction(
            &configuration,
            transfer_schedule_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Find a particular Seller Wallet account transfer schedule.
    pub async fn get_transfer_schedule(
        &self,
        transfer_schedule_id: &str,
    ) -> Result<models::seller_wallet_2024_03_01::TransferSchedule> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/get_transfer_schedule", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::get_transfer_schedule(
            &configuration,
            transfer_schedule_id,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Retrieve transfer schedules of a Seller Wallet bank account.
    pub async fn list_transfer_schedules(
        &self,
        account_id: &str,
        next_page_token: Option<&str>,
    ) -> Result<models::seller_wallet_2024_03_01::TransferScheduleListing> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/list_transfer_schedules", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::list_transfer_schedules(
            &configuration,
            account_id,
            next_page_token,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }

    /// Returns a transfer belonging to the updated scheduled transfer request
    pub async fn update_transfer_schedule(
        &self,
        dest_account_digital_signature: &str,
        amount_digital_signature: &str,
        body: models::seller_wallet_2024_03_01::TransferSchedule,
    ) -> Result<models::seller_wallet_2024_03_01::TransferSchedule> {
        let configuration = self.create_configuration().await?;
        let guard = self
            .limiter()
            .wait("seller_wallet_2024_03_01/update_transfer_schedule", 1.0, 5)
            .await?;
        let res = crate::apis::seller_wallet_2024_03_01::update_transfer_schedule(
            &configuration,
            dest_account_digital_signature,
            amount_digital_signature,
            body,
        )
        .await?;
        guard.mark_response().await;
        Ok(res)
    }
}
