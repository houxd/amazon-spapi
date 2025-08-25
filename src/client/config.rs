use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpapiConfig {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
    pub region: String,
    pub sandbox: bool,
    pub user_agent: Option<String>,
    pub timeout_sec: Option<u64>,
    pub rate_limit_factor: Option<f64>,
}

impl SpapiConfig {
    pub fn from_env() -> Result<Self> {
        let client_id = std::env::var("SPAPI_CLIENT_ID").map_err(|_| {
            anyhow::anyhow!("SPAPI_CLIENT_ID environment variable is not set")
        })?;
        let client_secret = std::env::var("SPAPI_CLIENT_SECRET").map_err(|_| {
            anyhow::anyhow!("SPAPI_CLIENT_SECRET environment variable is not set")
        })?;
        let refresh_token = std::env::var("SPAPI_REFRESH_TOKEN").map_err(|_| {
            anyhow::anyhow!("SPAPI_REFRESH_TOKEN environment variable is not set")
        })?;
        let region = std::env::var("SPAPI_REGION").map_err(|_| {
            anyhow::anyhow!("SPAPI_REGION environment variable is not set")
        })?;
        let sandbox = std::env::var("SPAPI_SANDBOX").map_err(|_| {
            anyhow::anyhow!("SPAPI_SANDBOX environment variable is not set or invalid")
        })?;
        let sandbox = sandbox == "true" || sandbox == "1";
        Ok(Self {
            client_id,
            client_secret,
            refresh_token,
            region,
            sandbox,
            user_agent: None,
            timeout_sec: Some(30),
            rate_limit_factor: None,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub spapi: SpapiConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn from_default_file() -> Result<Self> {
        Self::from_file("config.toml")
    }
}
