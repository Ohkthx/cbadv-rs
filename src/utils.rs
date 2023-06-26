use crate::time;
use hex;
use hmac::{Hmac, Mac};
use reqwest::{header, Method, Response, StatusCode};
use serde::Serialize;
use sha2::Sha256;
use std::fmt;

/// Types of errors that can occur.
#[derive(Debug)]
pub enum CBAdvError {
    /// Unable to parse JSON successfully.
    BadParse(String),
    /// Non-200 status code received.
    BadStatus(String),
    /// General unknown error.
    Unknown(String),
}

impl fmt::Display for CBAdvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CBAdvError::Unknown(value) => write!(f, "unknown error occured: {}", value),
            CBAdvError::BadParse(value) => write!(f, "could not parse: {}", value),
            CBAdvError::BadStatus(value) => write!(f, "non-zero status occurred: {}", value),
        }
    }
}

/// Used to return objects from the API to the end-user.
pub type Result<T> = std::result::Result<T, CBAdvError>;
type HmacSha256 = Hmac<Sha256>;

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

/// Responsible for signing and sending HTTP requests.
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
    pub fn get_signature(&self, method: Method, resource: &str, body: &str) -> header::HeaderMap {
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
    pub async fn get(&self, resource: &str, params: &str) -> Result<Response> {
        // Add the '?' to the beginning of the parameters if not empty.
        let prefix = match params.is_empty() {
            true => "",
            false => "?",
        };

        // Create the full URL being accessed.
        let target = format!("{}{}", prefix, params);
        let url = format!("{}{}{}", self.root, resource, target);

        // Create the signature and submit the request.
        let headers = self.get_signature(Method::GET, resource, &"".to_string());

        let result = self.client.get(url).headers(headers).send().await;
        match result {
            Ok(value) => match value.status() {
                StatusCode::OK => Ok(value),
                _ => {
                    let code = format!("Status Code: {}", value.status().as_u16());
                    match value.text().await {
                        Ok(text) => Err(CBAdvError::BadStatus(format!("{}, {}", code, text))),
                        Err(_) => Err(CBAdvError::BadStatus(format!(
                            "{}, could not parse error message",
                            code
                        ))),
                    }
                }
            },
            Err(_) => Err(CBAdvError::Unknown("GET request to API".to_string())),
        }
    }

    /// Performs a HTTP POST Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `params` - A string containing options / parameters for the URL.
    /// * `body` - An object to send to the URL via POST request.
    pub async fn post<T: Serialize>(
        &self,
        resource: &str,
        params: &str,
        body: T,
    ) -> Result<Response> {
        // Add the '?' to the beginning of the parameters if not empty.
        let prefix = match params.is_empty() {
            true => "",
            false => "?",
        };

        // Create the full URL being accessed.
        let target = format!("{}{}", prefix, params);
        let url = format!("{}{}{}", self.root, resource, target);

        // Create the signature and submit the request.
        let body_str = serde_json::to_string(&body).unwrap();
        let mut headers = self.get_signature(Method::POST, resource, &body_str);
        headers.insert("Content-Type", "application/json".parse().unwrap());

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
                        Ok(text) => Err(CBAdvError::BadStatus(format!("{}, {}", code, text))),
                        Err(_) => Err(CBAdvError::BadStatus(format!(
                            "{}, could not parse error message",
                            code
                        ))),
                    }
                }
            },
            Err(_) => Err(CBAdvError::Unknown("POST request to API".to_string())),
        }
    }
}
