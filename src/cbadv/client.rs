use crate::cbadv::utils::Signer;

use crate::cbadv::account::AccountAPI;
use crate::cbadv::fee::FeeAPI;
use crate::cbadv::order::OrderAPI;
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
    /// Gives access to the Account API.
    pub account: AccountAPI,
    /// Gives access to the Product API.
    pub product: ProductAPI,
    /// Gives access to the Fee API.
    pub fee: FeeAPI,
    /// Gives access to the Order API.
    pub order: OrderAPI,
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
        let account = AccountAPI::new(signer.clone());
        let product = ProductAPI::new(signer.clone());
        let fee = FeeAPI::new(signer.clone());
        let order = OrderAPI::new(signer.clone());

        Self {
            api_key: String::from(key),
            api_secret: String::from(secret),
            signer,
            account,
            product,
            fee,
            order,
        }
    }
}
