use crate::cbadv::time;
use hex;
use hmac::{Hmac, Mac};
use reqwest::{header, Method, Response};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Root URI for the API service.
const ROOT_URI: &str = "https://api.coinbase.com";

/// Creates and signs HTTP Requests to the API.
#[derive(Debug, Clone)]
pub struct Signer {
    /// API Key provided by the service.
    api_key: String,
    /// API Secret provided by the service.
    api_secret: String,
    /// Wrapped client that is responsible for making the requests.
    pub client: reqwest::Client,
    /// Root URL for the service.
    pub root: String,
}

impl Signer {
    /// Creates a new instance of Signer.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string that holds the key for the API service.
    /// * `api_secret` - A string that holds the secret for the API service.
    pub fn new(api_key: String, api_secret: String) -> Self {
        let client = reqwest::Client::new();
        let root = String::from(ROOT_URI);

        Self {
            api_key,
            api_secret,
            client,
            root,
        }
    }

    /// Creates the signature headers for a request.
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP Method as to which action to perform (GET, POST, etc.).
    /// * `resource` - A string slice representing the resource that is being accessed.
    /// * `body` - A string representing a body data.
    pub fn get_signature(
        &self,
        method: Method,
        resource: &String,
        body: &String,
    ) -> header::HeaderMap {
        // Timestamp of the request, must be +/- 30 seconds of remote system.
        let timestamp = time::now().to_string();

        // Pre-hash, combines all of the request data.
        let prehash = format!("{}{}{}{}", timestamp, method, resource, body);

        // Create the signature.
        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes())
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

    /// Performs a HTTP GET Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `params` - A string containing options / parameters for the URL.
    pub async fn get(&self, resource: String, params: String) -> Result<Response> {
        // Add the '?' to the beginning of the parameters if not empty.
        let prefix = match params.is_empty() {
            true => "",
            false => "?",
        };

        // Create the full URL being accessed.
        let target = format!("{}{}", prefix, params);
        let url = format!("{}{}{}", self.root, resource, target);

        // Create the signature and submit the request.
        let headers = self.get_signature(Method::GET, &resource, &"".to_string());
        match self.client.get(url).headers(headers).send().await {
            Ok(res) => Ok(res),
            Err(error) => Err(Box::new(error)),
        }
    }

    /// Performs a HTTP POST Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `params` - A string containing options / parameters for the URL.
    pub async fn post(&self, resource: String, params: String) -> Result<Response> {
        // Add the '?' to the beginning of the parameters if not empty.
        let prefix = match params.is_empty() {
            true => "",
            false => "?",
        };

        // Create the full URL being accessed.
        let target = format!("{}{}", prefix, params);
        let url = format!("{}{}{}", self.root, resource, target);

        // Create the signature and submit the request.
        let headers = self.get_signature(Method::GET, &resource, &"".to_string());
        let res = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .headers(headers)
            .send()
            .await;

        match res {
            Ok(value) => Ok(value),
            Err(error) => Err(Box::new(error)),
        }
    }
}
