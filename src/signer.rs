//! # Authentication and signing messages.
//!
//! `signer` contains the backbone of the API requests in the form of the Signer struct. This signs
//! all requests to the API for ensure proper authentication. Signer is also responsible for handling
//! the GET and POST requests.

use std::io::Write;

use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::{Method, Response, StatusCode};
use serde::Serialize;
use sha2::Sha256;

use crate::constants::ratelimits;
use crate::constants::API_ROOT_URI;
use crate::errors::CbAdvError;
use crate::time;
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
    /// API Key provided by the service.
    pub(crate) api_key: String,
    /// API Secret provided by the service.
    api_secret: String,
    /// Wrapped client that is responsible for making the requests.
    client: reqwest::Client,
    /// Token bucket, used for rate limiting.
    pub(crate) bucket: TokenBucket,
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
    pub(crate) fn new(api_key: String, api_secret: String, is_rest: bool) -> Self {
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
    fn get_http_signature(
        &self,
        method: Method,
        resource: &str,
        body: &str,
    ) -> Result<HeaderMap, Box<dyn std::error::Error>> {
        // Timestamp of the request, must be +/- 30 seconds of remote system.
        let timestamp = time::now().to_string();

        // Pre-hash, combines all of the request data.
        let mut prehash: Vec<u8> = Vec::new();
        write!(prehash, "{}{}{}{}", timestamp, method, resource, body)?;

        // Create the signature.
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())?;
        mac.update(&prehash);
        let signature = mac.finalize().into_bytes();
        let sign = hex::encode(signature);

        // Load the signature into the header map.
        let mut headers = HeaderMap::new();
        headers.insert("CB-ACCESS-KEY", HeaderValue::from_str(&self.api_key)?);
        headers.insert("CB-ACCESS-SIGN", HeaderValue::from_str(&sign)?);
        headers.insert("CB-ACCESS-TIMESTAMP", HeaderValue::from_str(&timestamp)?);

        Ok(headers)
    }

    /// Creates the signature for a websocket request.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Current timestamp as a string, must be +/- 30 seconds.
    /// * `channel` - Channel that is being modified (un/subscribe)
    /// * `product_ids` - Vector of product_ids that belong to the subscription.
    pub(crate) fn get_ws_signature(
        &self,
        timestamp: &str,
        channel: &str,
        product_ids: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Pre-hash, combines all of the request data.
        let mut prehash: Vec<u8> = Vec::new();
        write!(prehash, "{}{}", timestamp, channel)?;
        for id in product_ids {
            write!(prehash, ",{}", id)?;
        }

        // Create the signature.
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())?;
        mac.update(&prehash);
        let signature = mac.finalize().into_bytes();

        Ok(hex::encode(signature))
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
            value if !value.is_empty() => format!("{}{}{}", API_ROOT_URI, resource, value),
            _ => format!("{}{}", API_ROOT_URI, resource),
        };

        // Create the signature and submit the request.
        let headers = self
            .get_http_signature(Method::GET, resource, "")
            .map_err(|_| CbAdvError::BadSignature)?;

        // Wait until a token is available to make the request. Immediately consume it.
        self.bucket.wait_on().await;

        // Send the request and handle the response.
        let response = self
            .client
            .get(&url)
            .headers(headers)
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
            value if !value.is_empty() => format!("{}{}{}", API_ROOT_URI, resource, value),
            _ => format!("{}{}", API_ROOT_URI, resource),
        };

        // Serialize the body and handle potential serialization errors.
        let body_str = serde_json::to_string(&body).map_err(|_| CbAdvError::BadSerialization)?;

        // Create the signature and handle potential errors.
        let mut headers = self
            .get_http_signature(Method::POST, resource, &body_str)
            .map_err(|_| CbAdvError::BadSignature)?;

        // Set the Content-Type header.
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Wait until a token is available to make the request. Immediately consume it.
        self.bucket.wait_on().await;

        // Send the request and handle the response.
        let response = self
            .client
            .post(&url)
            .headers(headers)
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
