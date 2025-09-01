use reqwest::{retry, Client, Error, Request, Response};
use std::{cell::RefCell, ops::Deref};

#[derive(Debug, Clone)]
pub struct CustomClient {
    inner: Client,
    retry_count: usize,
}

impl CustomClient {
    pub fn new(client: Client, retry_count: usize) -> Self {
        Self { inner: client, retry_count }
    }

    pub async fn execute(&self, request: Request) -> Result<Response, Error> {
        log::debug!("Executing request to: {}", request.url());
        log::debug!("Executing request method: {}", request.method());
        log::debug!("Executing request headers: {:?}", request.headers());
        log::debug!("Executing request body: {:?}", request.body());

        if self.retry_count == 0 {
            let result = self.inner.execute(request).await;
            log::debug!("Response status: {:?}", result.as_ref().map(|r| r.status()));
            log::debug!(
                "Response headers: {:?}",
                result.as_ref().map(|r| r.headers())
            );
            return result;
        } else {
            let mut last_result = None;

            for i in 0..self.retry_count {
                // Clone the request for retry attempts
                let request_clone = match request.try_clone() {
                    Some(req) => req,
                    None => {
                        // If we can't clone the request, return the result of the original request
                        return self.inner.execute(request).await;
                    }
                };

                let result = self.inner.execute(request_clone).await;

                match &result {
                    Ok(response) => {
                        let status = response.status();
                        // If status is 429, sleep and retry
                        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                            log::warn!(
                                "Received 429 status, retrying in {} seconds (attempt {}/{})",
                                (i + 1) * 2,
                                i + 1,
                                self.retry_count
                            );
                            tokio::time::sleep(tokio::time::Duration::from_secs(((i + 1) * 2) as u64)).await;
                            last_result = Some(result);
                            continue;
                        } else {
                            // For other statuses, return the result immediately
                            return result;
                        }
                    }
                    Err(_) => {
                        // Request error, return immediately
                        return result;
                    }
                }
            }

            // If all retries exhausted, return the last result or execute the original request as a fallback
            match last_result {
                Some(res) => res,
                None => self.inner.execute(request).await,
            }
        }
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
