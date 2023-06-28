//! # Coinbase Advanced Client
//!
//! `client` gives access to all of the APIs for the Coinbase Advanced API.
//! This is the primary method of accessing the endpoints and handles all of the configurations and
//! negotiations for the user.

use crate::utils::Signer;

use crate::account::AccountAPI;
use crate::fee::FeeAPI;
use crate::order::OrderAPI;
use crate::product::ProductAPI;

/// Represents a Client for the API.
#[allow(dead_code)]
pub struct Client {
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
    pub fn new(key: &str, secret: &str) -> Self {
        Self {
            account: AccountAPI::new(Signer::new(key.to_string(), secret.to_string())),
            product: ProductAPI::new(Signer::new(key.to_string(), secret.to_string())),
            fee: FeeAPI::new(Signer::new(key.to_string(), secret.to_string())),
            order: OrderAPI::new(Signer::new(key.to_string(), secret.to_string())),
        }
    }
}

/// Creates a new instance of a Client. This is a wrapper for the various APIs and Signer.
///
/// # Arguments
///
/// * `key` - A string that holds the key for the API service.
/// * `secret` - A string that holds the secret for the API service.
pub fn new(key: &str, secret: &str) -> Client {
    Client::new(key, secret)
}
