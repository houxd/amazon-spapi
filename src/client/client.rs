use anyhow::Result;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::client::{AuthClient, RateLimiter, Region, SpapiConfig};
use crate::apis::configuration::Configuration;

pub struct SpapiClient {
    client: Client,
    auth_client: Arc<Mutex<AuthClient>>,
    config: SpapiConfig,
    rate_limiter: RateLimiter,
}

impl SpapiClient {
    /// Create a new SP API client with the given configuration
    pub fn new(config: SpapiConfig) -> Result<Self> {
        let user_agent = if let Some(ua) = &config.user_agent {
            ua.clone()
        } else {
            // Default user agent if not provided
            Self::get_user_agent()
        };

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(
                config.timeout_sec.unwrap_or(30),
            ))
            .user_agent(&user_agent)
            .build()?;

        let auth_client = AuthClient::new(
            config.client_id.clone(),
            config.client_secret.clone(),
            config.refresh_token.clone(),
            &user_agent,
        )?;

        // Initialize rate limiter if enabled
        let rate_limiter = RateLimiter::new_with_safety_factor(config.rate_limit_factor.unwrap_or(1.05));

        Ok(Self {
            client, //: Client::new(),
            auth_client: Arc::new(Mutex::new(auth_client)),
            config,
            rate_limiter,
        })
    }

    pub fn limiter(&self) -> &RateLimiter {
        &self.rate_limiter
    }

    /// Get default user agent for the client
    pub fn get_user_agent() -> String {
        let platform = format!("{}/{}", std::env::consts::OS, std::env::consts::ARCH);
        format!(
            "amazon-spapi/v{} (Language=Rust; Platform={})",
            env!("CARGO_PKG_VERSION"),
            platform
        )
    }

    /// Get the base URL for the client
    pub fn get_base_url(&self) -> String {
        if self.config.sandbox.unwrap_or(false) {
            match self.config.region {
                Region::NorthAmerica => format!("https://sandbox.sellingpartnerapi-na.amazon.com"),
                Region::Europe => format!("https://sandbox.sellingpartnerapi-eu.amazon.com"),
                Region::FarEast => format!("https://sandbox.sellingpartnerapi-fe.amazon.com"),
            }
        } else {
            match self.config.region {
                Region::NorthAmerica => format!("https://sellingpartnerapi-na.amazon.com"),
                Region::Europe => format!("https://sellingpartnerapi-eu.amazon.com"),
                Region::FarEast => format!("https://sellingpartnerapi-fe.amazon.com"),
            }
        }
    }

    /// Get access token from the auth client
    pub async fn get_access_token(&self) -> Result<String> {
        let mut auth_client = self.auth_client.lock().await;
        auth_client.get_access_token().await
    }

    /// Check if the client is in sandbox mode
    pub fn is_sandbox(&self) -> bool {
        self.config.sandbox.unwrap_or(false)
    }

    /// Upload content to the feed document URL (direct S3 upload)
    pub async fn upload(&self, url: &str, content: &str, content_type: &str) -> Result<()> {
        let response = self
            .client
            .put(url)
            .header("Content-Type", content_type)
            .body(content.to_string())
            .send()
            .await?;

        if response.status().is_success() {
            log::info!("Feed document content uploaded successfully");
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await?;
            Err(anyhow::anyhow!(
                "Failed to upload feed document content: {} - Response: {}",
                status,
                error_text
            ))
        }
    }

    /// Download content from a feed document URL
    pub async fn download(&self, url: &str) -> Result<String> {
        let response = self.get_http_client().get(url).send().await?;

        if response.status().is_success() {
            let content = response.text().await?;
            log::info!("Feed document content downloaded successfully");
            Ok(content)
        } else {
            let status = response.status();
            let error_text = response.text().await?;
            Err(anyhow::anyhow!(
                "Failed to download feed document content: {} - Response: {}",
                status,
                error_text
            ))
        }
    }

    /// Check if rate limiting is enabled and get token status
    #[allow(unused)]
    #[deprecated]
    pub async fn get_rate_limit_status(&self) -> Result<HashMap<String, (f64, f64, u32)>> {
        Ok(self.rate_limiter.get_token_status().await?)
    }

    /// Check if a token is available for a specific endpoint without consuming it
    #[allow(unused)]
    #[deprecated]
    pub async fn check_rate_limit_availability(&self, endpoint_id: &String) -> Result<bool> {
        Ok(self
            .rate_limiter
            .check_token_availability(endpoint_id)
            .await?)
    }

    /// Refresh the access token if needed
    pub async fn refresh_access_token_if_needed(&self) -> Result<()> {
        let mut auth_client = self.auth_client.lock().await;
        if !auth_client.is_token_valid() {
            auth_client.refresh_access_token().await?;
        }
        Ok(())
    }

    /// Force refresh the access token
    pub async fn force_refresh_token(&self) -> Result<()> {
        let mut auth_client = self.auth_client.lock().await;
        auth_client.refresh_access_token().await?;
        Ok(())
    }

    /// Get access to the underlying HTTP client for direct requests
    pub fn get_http_client(&self) -> &Client {
        &self.client
    }

    /// Create a new configuration for the generated APIs
    /// This function refreshes the access token and sets up the configuration
    pub async fn create_configuration(&self) -> Result<Configuration> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json; charset=utf-8".parse()?);
        headers.insert("host", "sellingpartnerapi-na.amazon.com".parse()?);
        headers.insert(
            "x-amz-access-token",
            self.get_access_token().await?.parse()?,
        );
        headers.insert(
            "x-amz-date",
            {
                let now = time::OffsetDateTime::now_utc();
                format!(
                    "{:04}{:02}{:02}T{:02}{:02}{:02}Z",
                    now.year(),
                    now.month() as u8,
                    now.day(),
                    now.hour(),
                    now.minute(),
                    now.second()
                )
            }
            .parse()?,
        );
        headers.insert(
            "user-agent",
            self.config
                .user_agent
                .clone()
                .unwrap_or_else(|| Self::get_user_agent())
                .parse()?,
        );

        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(
                self.config.timeout_sec.unwrap_or(30),
            ))
            .default_headers(headers)
            .build()?;

        let configuration = Configuration {
            base_path: self.get_base_url(),
            client: crate::apis::configuration::CustomClient::new(http_client),
            user_agent: Some(
                self.config
                    .user_agent
                    .clone()
                    .unwrap_or_else(|| Self::get_user_agent()),
            ),
        };
        Ok(configuration)
    }

    pub fn from_json<'a, T>(s: &'a str) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        serde_json::from_str(s).map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}: {}", e, s))
    }
}
