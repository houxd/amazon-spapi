# Amazon SPAPI Rust SDK

[![Crates.io](https://img.shields.io/crates/v/amazon-spapi)](https://crates.io/crates/amazon-spapi)
[![Documentation](https://docs.rs/amazon-spapi/badge.svg)](https://docs.rs/amazon-spapi)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust client library for Amazon Selling Partner API (SP-API), providing complete API coverage and type-safe interfaces.

## Features

- **Complete API Coverage**: Auto-generated API models from OpenAPI specifications covering nearly all Amazon SP-API endpoints
- **Integrated Client**: High-level client wrapper with convenient methods for common API operations


## Quick Start

### 1. Environment Setup

First, set up your Amazon SP-API credentials as environment variables:

```bash
export SPAPI_CLIENT_ID="your_client_id"
export SPAPI_CLIENT_SECRET="your_client_secret"
export SPAPI_REFRESH_TOKEN="your_refresh_token"
export SPAPI_MARKETPLACE_ID="your_marketplace_id"
export SPAPI_SANDBOX="false"  # or "true" for sandbox environment
```

### 2. Basic Usage

```rust
use amazon_spapi::{
    client::{SpapiClient, SpapiConfig},
    models::fba_inventory::InventorySummary,
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create client from environment variables
    let client = SpapiClient::new(SpapiConfig::from_env()?)?;
    
    // Get FBA inventory summaries
    let inventory = client
        .get_inventory_summaries(
            "Marketplace",
            client.get_marketplace_id(),
            vec![client.get_marketplace_id().to_string()],
            Some(false),
            None,
            None,
            None,
            None,
        )
        .await?;
    
    println!("Inventory summaries: {:?}", inventory);
    Ok(())
}
```

### 3. Using Generated APIs

For advanced usage and access to all API methods, you can use the auto-generated APIs directly. This approach gives you access to every Amazon SP-API endpoint:

```rust
use amazon_spapi::apis::sellers_api::get_marketplace_participations;
use amazon_spapi::client::{SpapiClient, SpapiConfig};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create client and configuration
    let spapi_config = SpapiConfig::from_env()?;
    let client = SpapiClient::new(spapi_config.clone())?;
    let configuration = client.create_configuration().await?;
    
    // Wait for rate limit token
    client
        .limiter()
        .wait_for_token("get_marketplace_participations", 0.016, 15)
        .await?;
    
    // Call the generated API function
    let res = get_marketplace_participations(&configuration).await;
    
    // Record the API call for rate limiting
    client.limiter()
        .record_response("get_marketplace_participations")
        .await?;
    
    println!("Marketplace Participations: {:#?}", res);
    Ok(())
}
```

This approach provides:
- **Complete API Access**: All Amazon SP-API endpoints are available
- **Type Safety**: Full type checking for all request/response models
- **Rate Limiting**: Built-in rate limiting management
- **Flexibility**: Direct access to all API parameters and options

You can find all available API functions in the `amazon_spapi::apis` module, organized by API category.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.


## Related Links

- [Amazon SP-API Official Documentation](https://developer-docs.amazon.com/sp-api/)

---

**Note**: This is an unofficial Amazon SP-API Rust client library. Please ensure compliance with Amazon's terms of service and API limitations.
