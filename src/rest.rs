//! # Coinbase Advanced Client
//!
//! `rest` gives access to all of the APIs via `Client` for the Coinbase Advanced API.
//! This is the primary method of accessing the endpoints and handles all of the configurations and
//! negotiations for the user.

use crate::account::AccountAPI;
use crate::fee::FeeAPI;
use crate::order::OrderAPI;
use crate::product::ProductAPI;
use crate::signer::Signer;

#[cfg(feature = "config")]
use crate::config::ConfigFile;

/// Represents a Client for the API.
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
    /// Creates a new instance of a Client. This is a wrapper for the various APIs.
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

    /// Creates a new instance of a Client using a configuration file. This is a wrapper for the various APIs and Signer.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration that implements ConfigFile trait.
    #[cfg(feature = "config")]
    pub fn from_config<T>(config: &T) -> Self
    where
        T: ConfigFile,
    {
        Self::new(&config.coinbase().api_key, &config.coinbase().api_secret)
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

/// Creates a new instance of a Client using a configuration file. This is a wrapper for the various APIs and Signer.
///
/// # Arguments
///
/// * `config` - Configuration that implements ConfigFile trait.
#[cfg(feature = "config")]
pub fn from_config<T>(config: &T) -> Client
where
    T: ConfigFile,
{
    Client::from_config(config)
}
