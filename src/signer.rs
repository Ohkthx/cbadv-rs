//! # Authentication and signing messages.
//!
//! `signer` contains the backbone of the API requests in the form of the Signer struct. This signs
//! all requests to the API for ensure proper authentication. Signer is also responsible for handling
//! the GET and POST requests.

use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use reqwest::{Response, StatusCode};
use serde::Serialize;

use crate::constants::{ratelimits, rest};
use crate::constants::{API_ROOT_URI, API_SANDBOX_ROOT_URI, CRATE_USER_AGENT};
use crate::errors::CbAdvError;
use crate::jwt::Jwt;
use crate::token_bucket::TokenBucket;
use crate::traits::Query;
use crate::types::CbResult;

/// Rate Limits for REST and WebSocket requests.
///
/// # Endpoint / Reference
///
/// * REST: <https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-rate-limits>
/// * WebSocket: <https://docs.cloud.coinbase.com/advanced-trade-api/docs/ws-rate-limits>
struct RateLimits {}
impl RateLimits {
    /// Maximum amount of tokens per bucket.
    const REST_MAX_TOKENS: f64 = ratelimits::REST_REFRESH_RATE;
    const WEBSOCKET_MAX_TOKENS: f64 = ratelimits::WEBSOCKET_REFRESH_RATE;

    /// Amount of tokens refreshed per second.
    ///
    /// # Arguments
    ///
    /// * `is_rest` - Requester is REST Client, true, otherwise false.
    fn refresh_rate(is_rest: bool) -> f64 {
        if is_rest {
            ratelimits::REST_REFRESH_RATE
        } else {
            ratelimits::WEBSOCKET_REFRESH_RATE
        }
    }

    /// Maximum amount of tokens for a bucket.
    ///
    /// # Arguments
    ///
    /// * `is_rest` - Requester is REST Client, true, otherwise false.
    fn max_tokens(is_rest: bool) -> f64 {
        if is_rest {
            RateLimits::REST_MAX_TOKENS
        } else {
            RateLimits::WEBSOCKET_MAX_TOKENS
        }
    }
}

/// Creates and signs HTTP Requests to the API.
#[derive(Debug, Clone)]
pub(crate) struct Signer {
    /// JSON Webtoken Generator
    jwt: Jwt,
    /// Wrapped client that is responsible for making the requests.
    client: reqwest::Client,
    /// Token bucket, used for rate limiting.
    pub(crate) bucket: TokenBucket,
    /// Root URI for the API.
    pub(crate) root_uri: &'static str,
}

/// Responsible for signing and sending HTTP requests.
impl Signer {
    /// Creates a new instance of Signer.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string that holds the key for the API service.
    /// * `api_secret` - A string that holds the secret for the API service.
    /// * `is_rest` - Signer for REST Client, true, otherwise false.
    /// * `use_sandbox` - A boolean that determines if the sandbox should be used.
    pub(crate) fn new(
        api_key: &str,
        api_secret: &str,
        is_rest: bool,
        use_sandbox: bool,
    ) -> CbResult<Self> {
        let root_uri = if use_sandbox {
            API_SANDBOX_ROOT_URI
        } else {
            API_ROOT_URI
        };

        Ok(Self {
            jwt: Jwt::new(api_key, api_secret)?,
            client: reqwest::Client::new(),
            bucket: TokenBucket::new(
                RateLimits::max_tokens(is_rest),
                RateLimits::refresh_rate(is_rest),
            ),
            root_uri,
        })
    }

    /// Gets a JSON Webtoken based on the service and URI provided.
    pub(crate) fn get_jwt(&self, service: &str, uri: Option<&str>) -> CbResult<String> {
        self.jwt.encode(service, uri)
    }

    /// Performs a HTTP GET Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    pub(crate) async fn get(&mut self, resource: &str, query: &impl Query) -> CbResult<Response> {
        // Efficiently construct the URL.
        let url = match query.to_query() {
            value if !value.is_empty() => format!("https://{}{}{}", self.root_uri, resource, value),
            _ => format!("https://{}{}", self.root_uri, resource),
        };

        // Create the signature and submit the request.
        // let uri = format!("{} {}", Method::GET, resource);
        let uri = Jwt::build_uri("GET", self.root_uri, resource);
        let token = self.get_jwt(rest::SERVICE, Some(&uri))?;

        // Wait until a token is available to make the request. Immediately consume it.
        self.bucket.wait_on().await;

        // Send the request and handle the response.
        let response = self
            .client
            .get(&url)
            .bearer_auth(token)
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, CRATE_USER_AGENT)
            .send()
            .await
            .map_err(|_| CbAdvError::Unknown("GET request to API".to_string()))?;

        if response.status() == StatusCode::OK {
            Ok(response)
        } else {
            let code = format!("Status Code: {}", response.status().as_u16());
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "could not parse error message".to_string());
            Err(CbAdvError::BadStatus(format!("{}, {}", code, text)))
        }
    }

    /// Performs a HTTP POST Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    /// * `body` - An object to send to the URL via POST request.
    pub(crate) async fn post<T: Serialize>(
        &mut self,
        resource: &str,
        query: &impl Query,
        body: T,
    ) -> CbResult<Response> {
        // Efficiently construct the URL.
        let url = match query.to_query() {
            value if !value.is_empty() => format!("https://{}{}{}", self.root_uri, resource, value),
            _ => format!("https://{}{}", self.root_uri, resource),
        };

        // Serialize the body and handle potential serialization errors.
        let body_str = serde_json::to_string(&body).map_err(|_| CbAdvError::BadSerialization)?;

        // Create the signature and handle potential errors.
        let uri = Jwt::build_uri("POST", self.root_uri, resource);
        let token = self.get_jwt(rest::SERVICE, Some(&uri))?;

        // Wait until a token is available to make the request. Immediately consume it.
        self.bucket.wait_on().await;

        // Send the request and handle the response.
        let response = self
            .client
            .post(&url)
            .bearer_auth(token)
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, CRATE_USER_AGENT)
            .body(body_str)
            .send()
            .await
            .map_err(|_| CbAdvError::Unknown("POST request to API".to_string()))?;

        if response.status() == StatusCode::OK {
            Ok(response)
        } else {
            let code = format!("Status Code: {}", response.status().as_u16());
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "could not parse error message".to_string());
            Err(CbAdvError::BadStatus(format!("{}, {}", code, text)))
        }
    }
}
