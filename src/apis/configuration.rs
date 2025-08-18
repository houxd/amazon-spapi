use reqwest::{Client, Request, Response, Error};
use std::{cell::RefCell, ops::Deref};

use crate::client::{RateLimitGuard, SpapiClient};

#[derive(Debug, Clone)]
pub struct CustomClient {
    inner: Client,
}

impl CustomClient {
    pub fn new(client: Client) -> Self {
        Self { inner: client }
    }

    pub async fn execute(&self, request: Request) -> Result<Response, Error> {
        log::debug!("Executing request to: {}", request.url());
        log::debug!("Executing request method: {}", request.method());
        log::debug!("Executing request headers: {:?}", request.headers());
        log::debug!("Executing request body: {:?}", request.body());

        let result = self.inner.execute(request).await;
      
        log::debug!("Response status: {:?}", result.as_ref().map(|r| r.status()));
        log::debug!("Response headers: {:?}", result.as_ref().map(|r| r.headers()));

        result
    }
}

impl Deref for CustomClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: CustomClient,
}