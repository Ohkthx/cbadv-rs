//! # Authentication and signing messages.
//!
//! `signer` contains the backbone of the API requests in the form of the Signer struct. This signs
//! all requests to the API for ensure proper authentication. Signer is also responsible for handling
//! the GET and POST requests.

use crate::time;
use crate::token_bucket::TokenBucket;
use crate::utils::{CbAdvError, Result};
use hex;
use hmac::{Hmac, Mac};
use reqwest::{header, Method, Response, StatusCode};
use serde::Serialize;
use sha2::Sha256;

/// Root URI for the API service.
const ROOT_URI: &str = "https://api.coinbase.com";

/// Rate Limits for REST and WebSocket requests.
///
/// # Endpoint / Reference
///
/// * REST: <https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-rate-limits>
/// * WebSocket: <https://docs.cloud.coinbase.com/advanced-trade-api/docs/ws-rate-limits>
struct RateLimits {}
impl RateLimits {
    /// Amount of tokens per second refilled.
    const REST_REFRESH_RATE: f64 = 30.0;
    const WEBSOCKET_REFRESH_RATE: f64 = 750.0;

    /// Maximum amount of tokens per bucket.
    const REST_MAX_TOKENS: f64 = RateLimits::REST_REFRESH_RATE;
    const WEBSOCKET_MAX_TOKENS: f64 = RateLimits::WEBSOCKET_REFRESH_RATE;

    /// Amount of tokens refreshed per second.
    ///
    /// # Arguments
    ///
    /// * `is_rest` - Requester is REST Client, true, otherwise false.
    fn refresh_rate(is_rest: bool) -> f64 {
        match is_rest {
            true => RateLimits::REST_REFRESH_RATE,
            false => RateLimits::WEBSOCKET_REFRESH_RATE,
        }
    }

    /// Maximum amount of tokens for a bucket.
    ///
    /// # Arguments
    ///
    /// * `is_rest` - Requester is REST Client, true, otherwise false.
    fn max_tokens(is_rest: bool) -> f64 {
        match is_rest {
            true => RateLimits::REST_MAX_TOKENS,
            false => RateLimits::WEBSOCKET_MAX_TOKENS,
        }
    }
}

/// Creates and signs HTTP Requests to the API.
#[derive(Debug, Clone)]
pub(crate) struct Signer {
    /// API Key provided by the service.
    pub api_key: String,
    /// API Secret provided by the service.
    api_secret: String,
    /// Wrapped client that is responsible for making the requests.
    client: reqwest::Client,
    /// Token bucket, used for rate limiting.
    pub bucket: TokenBucket,
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
    pub fn new(api_key: String, api_secret: String, is_rest: bool) -> Self {
        Self {
            api_key,
            api_secret,
            client: reqwest::Client::new(),
            bucket: TokenBucket::new(
                RateLimits::max_tokens(is_rest),
                RateLimits::refresh_rate(is_rest),
            ),
        }
    }

    /// Creates the signature headers for a request.
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP Method as to which action to perform (GET, POST, etc.).
    /// * `resource` - A string slice representing the resource that is being accessed.
    /// * `body` - A string representing a body data.
    fn get_http_signature(&self, method: Method, resource: &str, body: &str) -> header::HeaderMap {
        // Timestamp of the request, must be +/- 30 seconds of remote system.
        let timestamp = time::now().to_string();

        // Pre-hash, combines all of the request data.
        let prehash = format!("{}{}{}{}", timestamp, method, resource, body);

        // Create the signature.
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("Failed to generate a signature.");
        mac.update(prehash.as_bytes());
        let signature = mac.finalize();
        let sign = hex::encode(signature.into_bytes());

        // Load the signature into the header map.
        let mut headers = header::HeaderMap::new();
        headers.insert("CB-ACCESS-KEY", self.api_key.parse().unwrap());
        headers.insert("CB-ACCESS-SIGN", sign.parse().unwrap());
        headers.insert("CB-ACCESS-TIMESTAMP", timestamp.parse().unwrap());
        headers
    }

    /// Creates the signature for a websocket request.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Current timestamp as a string, must be +/- 30 seconds.
    /// * `channel` - Channel that is being modified (un/subscribe)
    /// * `product_ids` - Vector of product_ids that belong to the subscription.
    pub fn get_ws_signature(
        &self,
        timestamp: &str,
        channel: &str,
        product_ids: &Vec<String>,
    ) -> String {
        // Pre-hash, combines all of the request data.
        let prehash = format!("{}{}{}", timestamp, channel, product_ids.join(","));

        // Create the signature.
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("Failed to generate a signature.");
        mac.update(prehash.as_bytes());
        let signature = mac.finalize();
        hex::encode(signature.into_bytes())
    }

    /// Performs a HTTP GET Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    pub async fn get(&mut self, resource: &str, query: &str) -> Result<Response> {
        // Add the '?' to the beginning of the parameters if not empty.
        let prefix = match query.is_empty() {
            true => "",
            false => "?",
        };

        // Create the full URL being accessed.
        let target = format!("{}{}", prefix, query);
        let url = format!("{}{}{}", ROOT_URI, resource, target);

        // Create the signature and submit the request.
        let headers = self.get_http_signature(Method::GET, resource, &"".to_string());

        // Wait until a token is available to make the request. Immediately consume it.
        self.bucket.wait_on();

        let result = self.client.get(url).headers(headers).send().await;
        match result {
            Ok(value) => match value.status() {
                StatusCode::OK => Ok(value),
                _ => {
                    let code = format!("Status Code: {}", value.status().as_u16());
                    match value.text().await {
                        Ok(text) => Err(CbAdvError::BadStatus(format!("{}, {}", code, text))),
                        Err(_) => Err(CbAdvError::BadStatus(format!(
                            "{}, could not parse error message",
                            code
                        ))),
                    }
                }
            },
            Err(_) => Err(CbAdvError::Unknown("GET request to API".to_string())),
        }
    }

    /// Performs a HTTP POST Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    /// * `body` - An object to send to the URL via POST request.
    pub async fn post<T: Serialize>(
        &mut self,
        resource: &str,
        query: &str,
        body: T,
    ) -> Result<Response> {
        // Add the '?' to the beginning of the parameters if not empty.
        let prefix = match query.is_empty() {
            true => "",
            false => "?",
        };

        // Create the full URL being accessed.
        let target = format!("{}{}", prefix, query);
        let url = format!("{}{}{}", ROOT_URI, resource, target);

        // Create the signature and submit the request.
        let body_str = serde_json::to_string(&body).unwrap();
        let mut headers = self.get_http_signature(Method::POST, resource, &body_str);
        headers.insert("Content-Type", "application/json".parse().unwrap());

        // Wait until a token is available to make the request. Immediately consume it.
        self.bucket.wait_on();

        let result = self
            .client
            .post(url)
            .headers(headers)
            .body(body_str)
            .send()
            .await;

        match result {
            Ok(value) => match value.status() {
                StatusCode::OK => Ok(value),
                _ => {
                    let code = format!("Status Code: {}", value.status().as_u16());
                    match value.text().await {
                        Ok(text) => Err(CbAdvError::BadStatus(format!("{}, {}", code, text))),
                        Err(_) => Err(CbAdvError::BadStatus(format!(
                            "{}, could not parse error message",
                            code
                        ))),
                    }
                }
            },
            Err(_) => Err(CbAdvError::Unknown("POST request to API".to_string())),
        }
    }
}
