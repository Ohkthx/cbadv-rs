use crate::cbadv::utils::Signer;

use crate::cbadv::product::ProductAPI;

/// Represents a Client for the API.
#[allow(dead_code)]
pub struct Client {
    /// API Key provided by the service to the user.
    api_key: String,
    /// API Secret provided by the service to the user.
    api_secret: String,
    /// Responsible for making all HTTP requests.
    signer: Signer,
    /// Gives access to the Product API.
    pub product: ProductAPI,
}

impl Client {
    /// Creates a new instance of a Client. This is a wrapper for the various APIs and Signer.
    ///
    /// # Arguments
    ///
    /// * `key` - A string that holds the key for the API service.
    /// * `secret` - A string that holds the secret for the API service.
    pub fn new(key: String, secret: String) -> Self {
        let signer = Signer::new(key.clone(), secret.clone());
        let product = ProductAPI::new(signer.clone());

        Self {
            api_key: String::from(key),
            api_secret: String::from(secret),
            signer,
            product,
        }
    }
}
