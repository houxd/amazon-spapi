use anyhow::Result;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::apis::configuration::Configuration;
use crate::client::{ApiEndpoint, ApiMethod, AuthClient, RateLimiter, SpapiConfig};

// use amazon_spapi_gen::apis::configuration::Configuration;

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
        let rate_limiter = RateLimiter::new();

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
        if self.config.sandbox {
            format!("https://sandbox.sellingpartnerapi-na.amazon.com")
        } else {
            format!("https://sellingpartnerapi-na.amazon.com")
        }
    }

    /// Get the region for the client
    pub fn get_marketplace_id(&self) -> &str {
        &self.config.marketplace_id
    }

    /// Get access token from the auth client
    pub async fn get_access_token(&self) -> Result<String> {
        let mut auth_client = self.auth_client.lock().await;
        auth_client.get_access_token().await
    }

    /// Check if the client is in sandbox mode
    pub fn is_sandbox(&self) -> bool {
        self.config.sandbox
    }

    /// Make a request to the SP API
    pub async fn request(
        &self,
        endpoint: &ApiEndpoint,
        query: Option<Vec<(String, String)>>,
        header: Option<Vec<(&'static str, String)>>,
        body: Option<&str>,
    ) -> Result<String> {
        // Get access token
        let access_token = {
            let mut auth_client = self.auth_client.lock().await;
            auth_client.get_access_token().await?
        };

        let full_url = if query.is_none() {
            format!("{}{}", self.get_base_url(), endpoint.get_path())
        } else {
            let query_str = serde_urlencoded::to_string(&query)?;
            format!(
                "{}{}?{}",
                self.get_base_url(),
                endpoint.get_path(),
                query_str
            )
        };

        log::debug!("Making {} request to: {}", endpoint.method, full_url);

        // Create initial headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json; charset=utf-8".parse()?);
        headers.insert("host", "sellingpartnerapi-na.amazon.com".parse()?);
        headers.insert("x-amz-access-token", access_token.parse()?);
        headers.insert(
            "x-amz-date",
            // 时间格式 YYYYMMDDTHHMMSSZ
            chrono::Utc::now()
                .format("%Y%m%dT%H%M%SZ")
                .to_string()
                .parse()?,
        );
        headers.insert("user-agent", Self::get_user_agent().parse()?);
        if let Some(custom_headers) = header {
            for (key, value) in custom_headers {
                headers.insert(key, value.parse()?);
            }
        }

        // Build the request
        let mut request_builder = match endpoint.method {
            ApiMethod::Get => self.client.get(&full_url),
            ApiMethod::Post => self.client.post(&full_url),
            ApiMethod::Put => self.client.put(&full_url),
            ApiMethod::Delete => self.client.delete(&full_url),
            ApiMethod::Patch => self.client.patch(&full_url),
        };

        // Add headers
        request_builder = request_builder.headers(headers);

        // Add query parameters if provided
        if let Some(query_params) = query {
            request_builder = request_builder.query(&query_params);
        }

        // Add body if provided
        if let Some(body_content) = body {
            request_builder = request_builder.body(body_content.to_string());
        }

        // Apply rate limiting if enabled
        self.rate_limiter
            .wait_for_token(&endpoint.rate_limit_key(), endpoint.rate, endpoint.burst)
            .await?;

        let response = request_builder.send().await;

        // Record the response time for rate limiting
        self.rate_limiter
            .record_response(&endpoint.rate_limit_key())
            .await?;

        let response = response?;
        log::debug!("Response status: {}", response.status());

        let response_status = response.status();
        if response_status.is_success() {
            let text = response.text().await?;
            Ok(text)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!(
                "Request {} failed with status {}: {}",
                endpoint.get_path(),
                response_status,
                error_text
            ))
        }
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
    pub async fn get_rate_limit_status(&self) -> Result<HashMap<String, (f64, f64, u32)>> {
        Ok(self.rate_limiter.get_token_status().await?)
    }

    /// Check if a token is available for a specific endpoint without consuming it
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

    /// Create a new configuration for the apis
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
            chrono::Utc::now()
                .format("%Y%m%dT%H%M%SZ")
                .to_string()
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
            client: http_client,
            ..Default::default()
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
