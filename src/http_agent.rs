//! # Authentication and signing messages.
//!
//! `http_agent` contains the backbone of the API requests in the form of the SecureHttpAgent and PublicHttpAgent struct. This signs
//! all requests to the API for ensure proper authentication. The HttpAgents are also responsible for handling
//! the GET and POST requests.

use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use reqwest::{Method, Response, StatusCode, Url};
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

/// Trait for the HttpAgent that is responsible for making HTTP requests and managing the token bucket.
pub(crate) trait HttpAgent {
    /// Returns a mutable reference to the token bucket.
    fn bucket_mut(&mut self) -> &mut TokenBucket;

    /// Performs a HTTP GET Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    async fn get(&mut self, resource: &str, query: &impl Query) -> CbResult<Response>;

    /// Performs a HTTP POST Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    /// * `body` - An object to send to the URL via POST request.
    async fn post<T: Serialize>(
        &mut self,
        resource: &str,
        query: &impl Query,
        body: T,
    ) -> CbResult<Response>;
}

/// Base HTTP Agent that is responsible for making requests and token bucket.
#[derive(Debug, Clone)]
pub(crate) struct HttpAgentBase {
    /// Wrapped client that is responsible for making the requests.
    client: reqwest::Client,
    /// Token bucket, used for rate limiting.
    bucket: TokenBucket,
    /// Root URI for the API.
    root_uri: &'static str,
}

impl HttpAgentBase {
    /// Creates a new instance of SecureHttpAgent.
    ///
    /// # Arguments
    ///
    /// * `is_rest` - SecureHttpAgent for REST Client, true, otherwise false.
    /// * `use_sandbox` - A boolean that determines if the sandbox should be used.
    pub(crate) fn new(is_rest: bool, use_sandbox: bool) -> CbResult<Self> {
        let root_uri = if use_sandbox {
            API_SANDBOX_ROOT_URI
        } else {
            API_ROOT_URI
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| CbAdvError::Unknown(format!("Client Error: {}", e)))?;

        Ok(Self {
            client,
            bucket: TokenBucket::new(
                RateLimits::max_tokens(is_rest),
                RateLimits::refresh_rate(is_rest),
            ),
            root_uri,
        })
    }

    /// Constructs a URL for the request being made.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    fn build_url(&self, resource: &str, query: &impl Query) -> CbResult<Url> {
        let base_url = Url::parse(&format!("https://{}", self.root_uri))
            .map_err(|_| CbAdvError::Unknown("Invalid Base URL".to_string()))?;
        let mut url = base_url
            .join(resource)
            .map_err(|_| CbAdvError::Unknown("Invalid Resource Path".to_string()))?;
        url.set_query(Some(&query.to_query()));
        Ok(url)
    }

    /// Handles the response from the API.
    ///
    /// # Arguments
    ///
    /// * `response` - The response from the API.
    async fn handle_response(&self, response: Response) -> CbResult<Response> {
        if response.status() == StatusCode::OK {
            Ok(response)
        } else {
            let code = response.status().as_u16();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not parse error message".to_string());
            Err(CbAdvError::BadStatus(format!(
                "Status Code: {}, Error: {}",
                code, error_text
            )))
        }
    }

    /// Executes the request to the API.
    ///
    /// # Arguments
    ///
    /// * `method` - The method of the request, GET, POST, etc.
    /// * `url` - The URL to make the request to.
    /// * `body` - The body of the request, if any.
    /// * `token` - The token to authenticate the request.
    pub(crate) async fn execute_request(
        &mut self,
        method: Method,
        url: Url,
        body: Option<String>,
        token: Option<String>,
    ) -> CbResult<Response> {
        self.bucket.wait_on().await;

        let mut request = self
            .client
            .request(method, url)
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, CRATE_USER_AGENT);

        if let Some(token) = token {
            request = request.bearer_auth(token);
        }

        if let Some(body) = body {
            request = request.body(body);
        }

        let response = request
            .send()
            .await
            .map_err(|e| CbAdvError::Unknown(format!("Request Error: {}", e)))?;

        self.handle_response(response).await
    }
}

/// Unsigned HTTP Agent that is responsible for making requests without authentication.
#[derive(Debug, Clone)]
pub(crate) struct PublicHttpAgent {
    /// Base client that is responsible for making the requests.
    pub(crate) base: HttpAgentBase,
}

impl PublicHttpAgent {
    /// Creates a new instance of PublicHttpAgent.
    ///
    /// # Arguments
    ///
    /// * `is_rest` - SecureHttpAgent for REST Client, true, otherwise false.
    /// * `use_sandbox` - A boolean that determines if the sandbox should be used.
    pub(crate) fn new(is_rest: bool, use_sandbox: bool) -> CbResult<Self> {
        Ok(Self {
            base: HttpAgentBase::new(is_rest, use_sandbox)?,
        })
    }
}

impl HttpAgent for PublicHttpAgent {
    fn bucket_mut(&mut self) -> &mut TokenBucket {
        &mut self.base.bucket
    }

    async fn get(&mut self, resource: &str, query: &impl Query) -> CbResult<Response> {
        let url = self.base.build_url(resource, query)?;
        self.base
            .execute_request(Method::GET, url, None, None)
            .await
    }

    async fn post<T: Serialize>(
        &mut self,
        resource: &str,
        query: &impl Query,
        body: T,
    ) -> CbResult<Response> {
        let url = self.base.build_url(resource, query)?;
        let body_str = serde_json::to_string(&body).map_err(|_| CbAdvError::BadSerialization)?;
        self.base
            .execute_request(Method::POST, url, Some(body_str), None)
            .await
    }
}

/// Creates and signs HTTP Requests to the API.
#[derive(Debug, Clone)]
pub(crate) struct SecureHttpAgent {
    /// JSON Webtoken Generator, disabled in sandbox mode.
    jwt: Option<Jwt>,
    /// Base client that is responsible for making the requests.
    base: HttpAgentBase,
}

/// Responsible for signing and sending HTTP requests.
impl SecureHttpAgent {
    /// Creates a new instance of SecureHttpAgent.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string that holds the key for the API service.
    /// * `api_secret` - A string that holds the secret for the API service.
    /// * `is_rest` - SecureHttpAgent for REST Client, true, otherwise false.
    /// * `use_sandbox` - A boolean that determines if the sandbox should be used.
    pub(crate) fn new(
        api_key: &str,
        api_secret: &str,
        is_rest: bool,
        use_sandbox: bool,
    ) -> CbResult<Self> {
        let jwt = if use_sandbox {
            // Do not generate JWT in sandbox mode.
            None
        } else {
            Some(
                Jwt::new(api_key, api_secret)
                    .map_err(|e| CbAdvError::Unknown(format!("Error creating JWT: {}", e)))?,
            )
        };

        Ok(Self {
            jwt,
            base: HttpAgentBase::new(is_rest, use_sandbox)?,
        })
    }

    /// Generates a JSON Web Token (JWT) for authentication.
    ///
    /// # Arguments
    ///
    /// * `service` - The service name for which the JWT is generated.
    /// * `uri` - Optional URI to include in the JWT payload.
    pub(crate) fn get_jwt(&self, service: &str, uri: Option<&str>) -> CbResult<String> {
        if let Some(jwt) = &self.jwt {
            jwt.encode(service, uri)
        } else {
            Err(CbAdvError::Unknown(
    "JWT not setup. This request requires authentication, but JWT is disabled (likely sandbox mode).".to_string(),
))
        }
    }

    /// Builds a token for the request. If JWT is not enabled, returns None.
    ///
    /// # Arguments
    ///
    /// * `method` - The method of the request, GET, POST, etc.
    /// * `resource` - The resource being accessed.
    fn build_token(&self, method: Method, resource: &str) -> CbResult<Option<String>> {
        if let Some(jwt) = &self.jwt {
            let uri = Jwt::build_uri(method.as_str(), self.base.root_uri, resource);
            Ok(Some(jwt.encode(rest::SERVICE, Some(&uri))?))
        } else {
            Ok(None)
        }
    }
}

impl HttpAgent for SecureHttpAgent {
    fn bucket_mut(&mut self) -> &mut TokenBucket {
        &mut self.base.bucket
    }

    async fn get(&mut self, resource: &str, query: &impl Query) -> CbResult<Response> {
        let url = self.base.build_url(resource, query)?;
        let token = self.build_token(Method::GET, resource)?;
        self.base
            .execute_request(Method::GET, url, None, token)
            .await
    }

    async fn post<T: Serialize>(
        &mut self,
        resource: &str,
        query: &impl Query,
        body: T,
    ) -> CbResult<Response> {
        let url = self.base.build_url(resource, query)?;
        let body_str = serde_json::to_string(&body).map_err(|_| CbAdvError::BadSerialization)?;
        let token = self.build_token(Method::POST, resource)?;
        self.base
            .execute_request(Method::POST, url, Some(body_str), token)
            .await
    }
}
