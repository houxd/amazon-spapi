use std::fmt;

/// Represents an SP-API endpoint with its configuration
#[derive(Clone)]
pub struct ApiEndpoint {
    /// API version identifier for rate limiting grouping
    pub version: &'static str,
    /// The request path with placeholders for parameters
    pub path: &'static str,
    /// Parameters Map
    pub path_params: Option<Vec<(&'static str, String)>>,
    /// HTTP method
    pub method: ApiMethod,
    /// Rate limit for this endpoint in requests per second
    pub rate: f64,
    /// Burst capacity
    pub burst: u32,
}

impl ApiEndpoint {
    /// Generate the rate limiting key using api_version + path template
    pub fn rate_limit_key(&self) -> String {
        format!("{}:{}", self.version, self.path)
    }

    /// Get the actual URL path with parameters substituted
    pub fn get_path(&self) -> String {
        let mut path = self.path.to_string();
        if let Some(path_params) = &self.path_params {
            for (key, value) in path_params {
                let placeholder = format!("{{{}}}", key);
                path = path.replace(&placeholder, value);
            }
        }
        path
    }
}

/// HTTP methods supported by SP-API
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl fmt::Display for ApiMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiMethod::Get => write!(f, "GET"),
            ApiMethod::Post => write!(f, "POST"),
            ApiMethod::Put => write!(f, "PUT"),
            ApiMethod::Delete => write!(f, "DELETE"),
            ApiMethod::Patch => write!(f, "PATCH"),
        }
    }
}
