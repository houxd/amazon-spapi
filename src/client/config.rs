use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

/// Configuration for the Selling Partner API client.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SpapiConfig {
    /// The client ID provided by Amazon.
    pub client_id: String,
    /// The client secret provided by Amazon.
    pub client_secret: String,
    /// The refresh token for obtaining access tokens.
    pub refresh_token: String,
    /// The AWS region for the Selling Partner API.
    pub region: Region,
    /// Whether to use the sandbox environment.
    pub sandbox: Option<bool>,
    /// Custom user agent string. If not set, a default user agent will be used.
    pub user_agent: Option<String>,
    /// Request timeout in seconds. Defaults to 30 seconds if not set.
    pub timeout_sec: Option<u64>,
    /// Rate limit safety factor. Defaults to 1.1 if not set.
    pub rate_limit_factor: Option<f64>,
}

/// AWS Region for the Selling Partner API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Region {
    #[serde(rename = "us-east-1")]
    NorthAmerica,
    #[serde(rename = "eu-west-1")]
    Europe,
    #[serde(rename = "us-west-2")]
    FarEast,
}

impl Default for Region {
    fn default() -> Self {
        Region::NorthAmerica
    }
}

impl Region {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "na" | "us-east-1" => Ok(Region::NorthAmerica),
            "eu" | "eu-west-1" => Ok(Region::Europe),
            "fe" | "us-west-2" => Ok(Region::FarEast),
            _ => Err(anyhow::anyhow!("Invalid region string: {}", s)),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Region::NorthAmerica => "us-east-1",
            Region::Europe => "eu-west-1",
            Region::FarEast => "us-west-2",
        }
    }
}

impl SpapiConfig {
    pub fn from_env() -> Result<Self> {
        let client_id = std::env::var("SPAPI_CLIENT_ID")
            .map_err(|_| anyhow::anyhow!("SPAPI_CLIENT_ID environment variable is not set"))?;
        let client_secret = std::env::var("SPAPI_CLIENT_SECRET")
            .map_err(|_| anyhow::anyhow!("SPAPI_CLIENT_SECRET environment variable is not set"))?;
        let refresh_token = std::env::var("SPAPI_REFRESH_TOKEN")
            .map_err(|_| anyhow::anyhow!("SPAPI_REFRESH_TOKEN environment variable is not set"))?;
        let region = std::env::var("SPAPI_REGION")
            .map_err(|_| anyhow::anyhow!("SPAPI_REGION environment variable is not set"))?;
        let sandbox = std::env::var("SPAPI_SANDBOX").map_err(|_| {
            anyhow::anyhow!("SPAPI_SANDBOX environment variable is not set or invalid")
        })?;
        let sandbox = sandbox == "true" || sandbox == "1";
        Ok(Self {
            client_id,
            client_secret,
            refresh_token,
            region: Region::from_str(&region)?,
            sandbox: Some(sandbox),
            user_agent: None,
            timeout_sec: Some(30),
            rate_limit_factor: None,
        })
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn from_default_file() -> Result<Self> {
        Self::from_file("config.toml")
    }
}
