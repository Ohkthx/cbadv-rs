//! # Authentication and signing messages.
//!
//! `http_agent` contains the backbone of the API requests in the form of the SecureHttpAgent and PublicHttpAgent struct. This signs
//! all requests to the API for ensure proper authentication. The HttpAgents are also responsible for handling
//! the GET and POST requests.

use std::sync::Arc;

use futures::lock::Mutex;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use reqwest::{Method, Response, Url};
use serde::Serialize;

use crate::constants::{API_ROOT_URI, API_SANDBOX_ROOT_URI, CRATE_USER_AGENT};
use crate::errors::CbError;
use crate::jwt::Jwt;
use crate::token_bucket::TokenBucket;
use crate::traits::{HttpAgent, Query, Request};
use crate::types::CbResult;

/// Base HTTP Agent that is responsible for making requests and token bucket.
#[derive(Debug, Clone)]
pub(crate) struct HttpAgentBase {
    /// Wrapped client that is responsible for making the requests.
    client: reqwest::Client,
    /// Token bucket, used for rate limiting.
    bucket: Arc<Mutex<TokenBucket>>,
    /// Root URI for the API.
    root_uri: &'static str,
}

impl HttpAgentBase {
    /// Creates a new instance of SecureHttpAgent.
    ///
    /// # Arguments
    ///
    /// * `use_sandbox` - A boolean that determines if the sandbox should be used.
    /// * `shared_bucket` - Shared token bucket for all APIs.
    pub(crate) fn new(use_sandbox: bool, shared_bucket: Arc<Mutex<TokenBucket>>) -> CbResult<Self> {
        let root_uri = if use_sandbox {
            API_SANDBOX_ROOT_URI
        } else {
            API_ROOT_URI
        };

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| CbError::RequestError(e.to_string()))?;

        Ok(Self {
            client,
            bucket: shared_bucket,
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
        // Ensure the query is valid.
        query.check()?;

        let base_url = Url::parse(&format!("https://{}", self.root_uri))
            .map_err(|e| CbError::UrlParseError(e.to_string()))?;
        let mut url = base_url
            .join(resource)
            .map_err(|e| CbError::UrlParseError(e.to_string()))?;
        url.set_query(Some(&query.to_query()));
        Ok(url)
    }

    /// Converts the request to a JSON string.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to convert to a JSON string.
    fn convert_request<'a, T>(&self, request: &'a T) -> CbResult<String>
    where
        T: Request + Serialize + 'a,
    {
        request.check()?;
        let data = serde_json::to_string(&request)
            .map_err(|e| CbError::BadSerialization(e.to_string()))?;
        Ok(data)
    }

    /// Handles the response from the API.
    ///
    /// # Arguments
    ///
    /// * `response` - The response from the API.
    async fn handle_response(&self, response: Response) -> CbResult<Response> {
        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Could not parse error message".to_string());
            Err(CbError::BadStatus { code: status, body })
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
        {
            let mut locked_bucket = self.bucket.lock().await;
            locked_bucket.wait_on().await;
        }

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
            .map_err(|e| CbError::RequestError(e.to_string()))?;

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
    /// * `use_sandbox` - A boolean that determines if the sandbox should be used.
    /// * `shared_bucket` - Shared token bucket for all APIs.
    pub(crate) fn new(use_sandbox: bool, shared_bucket: Arc<Mutex<TokenBucket>>) -> CbResult<Self> {
        Ok(Self {
            base: HttpAgentBase::new(use_sandbox, shared_bucket)?,
        })
    }
}

impl HttpAgent for PublicHttpAgent {
    async fn get(&mut self, resource: &str, query: &impl Query) -> CbResult<Response> {
        let url = self.base.build_url(resource, query)?;
        self.base
            .execute_request(Method::GET, url, None, None)
            .await
    }

    async fn post<'a, T>(
        &mut self,
        resource: &str,
        query: &impl Query,
        body: &'a T,
    ) -> CbResult<Response>
    where
        T: Request + Serialize + 'a,
    {
        let url = self.base.build_url(resource, query)?;
        let data = self.base.convert_request(body)?;
        self.base
            .execute_request(Method::POST, url, Some(data), None)
            .await
    }

    async fn put<'a, T>(
        &mut self,
        resource: &str,
        query: &impl Query,
        body: &'a T,
    ) -> CbResult<Response>
    where
        T: Request + Serialize + 'a,
    {
        let url = self.base.build_url(resource, query)?;
        let data = self.base.convert_request(body)?;
        self.base
            .execute_request(Method::PUT, url, Some(data), None)
            .await
    }

    async fn delete(&mut self, resource: &str, query: &impl Query) -> CbResult<Response> {
        let url = self.base.build_url(resource, query)?;
        self.base
            .execute_request(Method::DELETE, url, None, None)
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
    /// * `use_sandbox` - A boolean that determines if the sandbox should be used.
    /// * `shared_bucket` - Shared token bucket for all APIs.
    pub(crate) fn new(
        api_key: &str,
        api_secret: &str,
        use_sandbox: bool,
        shared_bucket: Arc<Mutex<TokenBucket>>,
    ) -> CbResult<Self> {
        let jwt = if use_sandbox {
            // Do not generate JWT in sandbox mode.
            None
        } else {
            Some(
                Jwt::new(api_key, api_secret)
                    .map_err(|e| CbError::Unknown(format!("Error creating JWT: {}", e)))?,
            )
        };

        Ok(Self {
            jwt,
            base: HttpAgentBase::new(use_sandbox, shared_bucket)?,
        })
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
            Ok(Some(jwt.encode(Some(&uri))?))
        } else {
            Ok(None)
        }
    }
}

impl HttpAgent for SecureHttpAgent {
    async fn get(&mut self, resource: &str, query: &impl Query) -> CbResult<Response> {
        let url = self.base.build_url(resource, query)?;
        let token = self.build_token(Method::GET, resource)?;
        self.base
            .execute_request(Method::GET, url, None, token)
            .await
    }

    async fn post<'a, T>(
        &mut self,
        resource: &str,
        query: &impl Query,
        body: &'a T,
    ) -> CbResult<Response>
    where
        T: Request + Serialize + 'a,
    {
        let url = self.base.build_url(resource, query)?;
        let data = self.base.convert_request(body)?;
        let token = self.build_token(Method::POST, resource)?;
        self.base
            .execute_request(Method::POST, url, Some(data), token)
            .await
    }

    async fn put<'a, T>(
        &mut self,
        resource: &str,
        query: &impl Query,
        body: &'a T,
    ) -> CbResult<Response>
    where
        T: Request + Serialize + 'a,
    {
        let url = self.base.build_url(resource, query)?;
        let data = self.base.convert_request(body)?;
        let token = self.build_token(Method::PUT, resource)?;
        self.base
            .execute_request(Method::PUT, url, Some(data), token)
            .await
    }

    async fn delete(&mut self, resource: &str, query: &impl Query) -> CbResult<Response> {
        let url = self.base.build_url(resource, query)?;
        let token = self.build_token(Method::DELETE, resource)?;
        self.base
            .execute_request(Method::DELETE, url, None, token)
            .await
    }
}
