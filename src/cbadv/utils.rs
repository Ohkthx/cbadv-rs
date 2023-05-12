use hex;
use hmac::{Hmac, Mac};
use reqwest::{header, Method, Response};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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
        let root = String::from("https://api.coinbase.com");

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
    /// * `item` - A string for the exact item being accessed within a resource.
    pub fn get_signature(&self, method: Method, resource: &str, item: String) -> header::HeaderMap {
        // Timestamp of the request, must be +/- 30 seconds of remote system.
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        // Pre-hash, combines all of the request data.
        let prehash = format!("{}{}{}{}", timestamp, method, resource, item);

        // Create the signature.
        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
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

    /// Performs a HTTP Reques.
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP Method as to which action to perform (GET, POST, etc.).
    /// * `resource` - A string slice representing the resource that is being accessed.
    /// * `item` - A string for the exact item being accessed within a resource.
    pub async fn request(&self, method: Method, resource: &str, item: String) -> Result<Response> {
        // Create the full URL being accessed.
        let target = format!("/{}", item);
        let url = format!("{}{}{}", self.root, resource, target);

        // Create the signature and submit the request.
        let headers = self.get_signature(method, resource, target);
        let res = self.client.get(url).headers(headers).send().await?;
        Ok(res)
    }
}
