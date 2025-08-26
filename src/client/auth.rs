use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::client::{SpapiClient, SpapiConfig};

#[derive(Debug, Serialize)]
pub struct LwaTokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LwaTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone)]
pub struct CachedToken {
    pub access_token: String,
    pub expires_at: u64, // Unix timestamp
}

pub struct AuthClient {
    client: Client,
    config: SpapiConfig,
    cached_token: Option<CachedToken>,
}

impl AuthClient {
    pub fn new(config: SpapiConfig) -> Result<Self> {
        let user_agent = if let Some(ua) = &config.user_agent {
            ua.clone()
        } else {
            // Default user agent if not provided
            SpapiClient::get_default_user_agent()
        };

        let mut client_builder = Client::builder()
            .timeout(std::time::Duration::from_secs(
                config.timeout_sec.unwrap_or(30),
            ))
            .user_agent(&user_agent);

        if let Some(proxy_url) = &config.proxy {
            let proxy = reqwest::Proxy::all(proxy_url)?;
            client_builder = client_builder.proxy(proxy);
        }

        let client = client_builder.build()?;

        Ok(Self {
            client,
            config,
            cached_token: None,
        })
    }

    fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn is_token_valid(&self) -> bool {
        if let Some(ref cached) = self.cached_token {
            let current_time = Self::get_current_timestamp();
            // Expire 5 minutes early to ensure token is still valid when used
            let buffer_time = 300; // 5 Minutes
            cached.expires_at > current_time + buffer_time
        } else {
            false
        }
    }

    pub async fn get_access_token(&mut self) -> Result<String> {
        // Check if we have a cached token
        if self.is_token_valid() {
            if let Some(ref cached) = self.cached_token {
                return Ok(cached.access_token.clone());
            }
        }

        // If no valid cached token, get a new token
        self.refresh_access_token().await
    }

    pub async fn refresh_access_token(&mut self) -> Result<String> {
        log::debug!("Refreshing access token...");
        let lwa_url = "https://api.amazon.com/auth/o2/token";

        let request_body = LwaTokenRequest {
            grant_type: "refresh_token".to_string(),
            client_id: self.config.client_id.clone(),
            client_secret: self.config.client_secret.clone(),
            refresh_token: self.config.refresh_token.clone(),
        };

        log::debug!("Request Body: {:?}", request_body);

        let response = self
            .client
            .post(lwa_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&request_body)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: LwaTokenResponse = response.json().await?;

            log::debug!("Response: {:?}", token_response);

            // Calculate expiration time
            let current_time = Self::get_current_timestamp();
            let expires_at = current_time + token_response.expires_in;

            // Cache the new token
            self.cached_token = Some(CachedToken {
                access_token: token_response.access_token.clone(),
                expires_at,
            });

            // println!("New access token cached, Valid until: {}, ({} seconds remaining)",
            //     expires_at,
            //     token_response.expires_in
            // );

            Ok(token_response.access_token)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!(
                "Failed to get access token: {}",
                error_text
            ))
        }
    }

    /// Get the remaining time (in seconds) for the current cached token
    pub fn get_token_remaining_time(&self) -> Option<u64> {
        if let Some(ref cached) = self.cached_token {
            let current_time = Self::get_current_timestamp();
            if cached.expires_at > current_time {
                Some(cached.expires_at - current_time)
            } else {
                Some(0)
            }
        } else {
            None
        }
    }
}
