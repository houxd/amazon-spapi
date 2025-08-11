use crate::{
    client::{ApiEndpoint, ApiMethod, SpapiClient},
    models::{
        self,
        product_fees_v0::{
            FeesEstimateByIdRequest, FeesEstimateRequest, GetMyFeesEstimateRequest, IdType,
            MoneyType, OptionalFulfillmentProgram, PriceToEstimateFees,
        },
        sellers::{GetAccountResponse, GetMarketplaceParticipationsResponse},
    },
};
use anyhow::Result;

impl SpapiClient {
    pub async fn get_my_fees_estimate_for_asin(
        &self,
        asin: &str,
        body: models::product_fees_v0::GetMyFeesEstimateRequest,
    ) -> Result<models::product_fees_v0::GetMyFeesEstimateResponse> {
        let configuration = self.create_configuration().await?;
        let _ = self
            .limiter()
            .wait("/products/fees/v0/items/feesEstimate", 1.0, 2)
            .await?;
        let res = crate::apis::fees_api::get_my_fees_estimate_for_asin(&configuration, asin, body)
            .await?;
        Ok(res)
    }

    pub async fn get_my_fees_estimate_for_sku(
        &self,
        seller_sku: &str,
        body: models::product_fees_v0::GetMyFeesEstimateRequest,
    ) -> Result<models::product_fees_v0::GetMyFeesEstimateResponse> {
        let configuration = self.create_configuration().await?;
        let _ = self
            .limiter()
            .wait("/products/fees/v0/listings/feesEstimate", 1.0, 2)
            .await?;
        let res =
            crate::apis::fees_api::get_my_fees_estimate_for_sku(&configuration, seller_sku, body)
                .await?;
        Ok(res)
    }

    pub async fn get_my_fees_estimates(
        &self,
        body: Vec<models::product_fees_v0::FeesEstimateByIdRequest>,
    ) -> Result<Vec<models::product_fees_v0::FeesEstimateResult>> {
        let configuration = self.create_configuration().await?;
        let _ = self
            .limiter()
            .wait("/products/fees/v0/feesEstimate", 0.2, 1) // 0.5 always get 429
            .await?;
        let res = crate::apis::fees_api::get_my_fees_estimates(&configuration, body).await?;
        Ok(res)
    }

    /// Convenience method to get fees estimate for a single ASIN with price
    pub async fn get_fee_for_asin(
        &self,
        asin: &str,
        price: f64,
        is_amazon_fulfilled: bool,
    ) -> Result<f64> {
        let request = GetMyFeesEstimateRequest {
            fees_estimate_request: Some(Box::new(FeesEstimateRequest {
                marketplace_id: self.get_marketplace_id().to_string(),
                is_amazon_fulfilled: Some(is_amazon_fulfilled),
                price_to_estimate_fees: Box::new(PriceToEstimateFees {
                    listing_price: Box::new(MoneyType {
                        currency_code: Some("USD".to_string()),
                        amount: Some(price),
                    }),
                    shipping: None,
                    points: None,
                }),
                identifier: asin.to_string(),
                optional_fulfillment_program: Some(OptionalFulfillmentProgram::FbaCore),
            })),
        };

        let response = self.get_my_fees_estimate_for_asin(asin, request).await?;

        if let Some(payload) = response.payload {
            if let Some(result) = payload.fees_estimate_result {
                if result.status == Some("Success".to_string()) {
                    if let Some(estimate) = result.fees_estimate {
                        if let Some(total_fees) = estimate.total_fees_estimate {
                            return Ok(total_fees.amount.unwrap_or(0.0));
                        }
                    }
                }
            }
        }

        Err(anyhow::anyhow!(
            "Failed to get fee estimate for ASIN: {}",
            asin
        ))
    }

    /// Get fees estimates for multiple ASINs (batch operation)
    pub async fn get_fees_for_asins(
        &self,
        asins_with_prices: Vec<(String, f64)>,
        is_amazon_fulfilled: bool,
    ) -> Result<Vec<(String, f64)>> {
        let requests: Vec<FeesEstimateByIdRequest> = asins_with_prices
            .iter()
            .map(|(asin, price)| FeesEstimateByIdRequest {
                fees_estimate_request: Some(Box::new(FeesEstimateRequest {
                    marketplace_id: self.get_marketplace_id().to_string(),
                    is_amazon_fulfilled: Some(is_amazon_fulfilled),
                    price_to_estimate_fees: Box::new(PriceToEstimateFees {
                        listing_price: Box::new(MoneyType {
                            currency_code: Some("USD".to_string()),
                            amount: Some(*price),
                        }),
                        shipping: None,
                        points: None,
                    }),
                    identifier: asin.clone(),
                    optional_fulfillment_program: None,
                })),
                id_type: IdType::Asin,
                id_value: asin.clone(),
            })
            .collect();

        let results = self.get_my_fees_estimates(requests).await?;

        let mut fees: Vec<(String, f64)> = Vec::new();
        for result in results {
            if let Some(identifier) = result.fees_estimate_identifier {
                let asin = identifier.id_value.unwrap_or_default();
                let fee = if result.status == Some("Success".to_string()) {
                    if let Some(estimate) = result.fees_estimate {
                        if let Some(total_fees) = estimate.total_fees_estimate {
                            total_fees.amount.unwrap_or(0.0)
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                fees.push((asin, fee));
            }
        }

        Ok(fees)
    }

    /// Convenience method to get fees estimate for a single SKU with price
    pub async fn get_fee_for_sku(
        &self,
        sku: &str,
        price: f64,
        is_amazon_fulfilled: bool,
    ) -> Result<f64> {
        let request = GetMyFeesEstimateRequest {
            fees_estimate_request: Some(Box::new(FeesEstimateRequest {
                marketplace_id: self.get_marketplace_id().to_string(),
                is_amazon_fulfilled: Some(is_amazon_fulfilled),
                price_to_estimate_fees: Box::new(PriceToEstimateFees {
                    listing_price: Box::new(MoneyType {
                        currency_code: Some("USD".to_string()),
                        amount: Some(price),
                    }),
                    shipping: None,
                    points: None,
                }),
                identifier: sku.to_string(),
                optional_fulfillment_program: Some(OptionalFulfillmentProgram::FbaCore),
            })),
        };

        let response = self.get_my_fees_estimate_for_sku(sku, request).await?;

        if let Some(payload) = response.payload {
            if let Some(result) = payload.fees_estimate_result {
                if result.status == Some("Success".to_string()) {
                    if let Some(estimate) = result.fees_estimate {
                        if let Some(total_fees) = estimate.total_fees_estimate {
                            return Ok(total_fees.amount.unwrap_or(0.0));
                        }
                    }
                }
            }
        }

        Err(anyhow::anyhow!(
            "Failed to get fee estimate for SKU: {}",
            sku
        ))
    }

    /// Get fees estimates for multiple SKUs (batch operation)
    pub async fn get_fees_for_skus(
        &self,
        skus_with_prices: Vec<(String, f64)>,
        is_amazon_fulfilled: bool,
    ) -> Result<Vec<(String, f64)>> {
        let requests: Vec<FeesEstimateByIdRequest> = skus_with_prices
            .iter()
            .map(|(sku, price)| FeesEstimateByIdRequest {
                fees_estimate_request: Some(Box::new(FeesEstimateRequest {
                    marketplace_id: self.get_marketplace_id().to_string(),
                    is_amazon_fulfilled: Some(is_amazon_fulfilled),
                    price_to_estimate_fees: Box::new(PriceToEstimateFees {
                        listing_price: Box::new(MoneyType {
                            currency_code: Some("USD".to_string()),
                            amount: Some(*price),
                        }),
                        shipping: None,
                        points: None,
                    }),
                    identifier: sku.clone(),
                    optional_fulfillment_program: None,
                })),
                id_type: IdType::SellerSku,
                id_value: sku.clone(),
            })
            .collect();
        let results = self.get_my_fees_estimates(requests).await?;
        let mut fees: Vec<(String, f64)> = Vec::new();
        for result in results {
            if let Some(identifier) = result.fees_estimate_identifier {
                let sku = identifier.id_value.unwrap_or_default();
                let fee = if result.status == Some("Success".to_string()) {
                    if let Some(estimate) = result.fees_estimate {
                        if let Some(total_fees) = estimate.total_fees_estimate {
                            total_fees.amount.unwrap_or(0.0)
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                fees.push((sku, fee));
            }
        }
        Ok(fees)
    }
}
