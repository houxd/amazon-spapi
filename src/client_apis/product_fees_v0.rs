use crate::client::{ApiEndpoint, ApiMethod, SpapiClient};
use crate::models::product_fees_v0::{
    FeesEstimateByIdRequest, FeesEstimateRequest, FeesEstimateResult, GetMyFeesEstimateRequest,
    GetMyFeesEstimateResponse, IdType, MoneyType, OptionalFulfillmentProgram, PriceToEstimateFees,
};
use anyhow::Result;

impl SpapiClient {
    /// Get fees estimate for a product by seller SKU
    pub async fn get_my_fees_estimate_for_sku(
        &self,
        sku: &str,
        request: GetMyFeesEstimateRequest,
    ) -> Result<GetMyFeesEstimateResponse> {
        let endpoint = ApiEndpoint {
            version: "product_fees_v0",
            path: "/products/fees/v0/listings/{sku}/feesEstimate",
            path_params: Some(vec![("sku", sku.to_string())]),
            method: ApiMethod::Post,
            rate: 1.0,
            burst: 2,
        };
        let body = serde_json::to_string(&request)?;

        let res = self.request(&endpoint, None, None, Some(&body)).await?;
        Self::from_json(&res)
    }

    /// Get fees estimate for a product by ASIN
    pub async fn get_my_fees_estimate_for_asin(
        &self,
        asin: &str,
        request: GetMyFeesEstimateRequest,
    ) -> Result<GetMyFeesEstimateResponse> {
        let endpoint = ApiEndpoint {
            version: "product_fees_v0",
            path: "/products/fees/v0/items/{asin}/feesEstimate",
            path_params: Some(vec![("asin", asin.to_string())]),
            method: ApiMethod::Post,
            rate: 1.0,
            burst: 2,
        };
        let body = serde_json::to_string(&request)?;
        let res = self.request(&endpoint, None, None, Some(&body)).await?;
        Self::from_json(&res)
    }

    /// Get fees estimates for multiple products (batch operation)
    pub async fn get_my_fees_estimates(
        &self,
        requests: Vec<FeesEstimateByIdRequest>,
    ) -> Result<Vec<FeesEstimateResult>> {
        let endpoint = ApiEndpoint {
            version: "product_fees_v0",
            path: "/products/fees/v0/feesEstimate",
            path_params: None,
            method: ApiMethod::Post,
            rate: 0.5,
            burst: 1,
        };

        // Convert the request array to JSON string for the POST body
        let body = serde_json::to_string(&requests)?;

        let res = self.request(&endpoint, None, None, Some(&body)).await?;
        Self::from_json(&res)
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
