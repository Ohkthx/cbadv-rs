//! # Coinbase Advanced Client
//!
//! `rest` gives access to all of the APIs via `Client` for the Coinbase Advanced API.
//! This is the primary method of accessing the endpoints and handles all of the configurations and
//! negotiations for the user.

use crate::apis::{AccountApi, ConvertApi, FeeApi, OrderApi, ProductApi, UtilApi};
use crate::signer::Signer;

#[cfg(feature = "config")]
use crate::config::ConfigFile;
use crate::types::CbResult;

/// Represents a Client for the API.
pub struct RestClient {
    /// Gives access to the Account API.
    pub account: AccountApi,
    /// Gives access to the Product API.
    pub product: ProductApi,
    /// Gives access to the Fee API.
    pub fee: FeeApi,
    /// Gives access to the Order API.
    pub order: OrderApi,
    /// Gives access to the Convert API.
    pub convert: ConvertApi,
    /// Gives access to the Util API.
    pub util: UtilApi,
}

impl RestClient {
    /// Creates a new instance of a Client. This is a wrapper for the various APIs.
    ///
    /// # Arguments
    ///
    /// * `key` - A string that holds the key for the API service.
    /// * `secret` - A string that holds the secret for the API service.
    pub fn new(key: &str, secret: &str) -> CbResult<Self> {
        Ok(Self {
            account: AccountApi::new(Signer::new(key, secret, true)?),
            product: ProductApi::new(Signer::new(key, secret, true)?),
            fee: FeeApi::new(Signer::new(key, secret, true)?),
            order: OrderApi::new(Signer::new(key, secret, true)?),
            convert: ConvertApi::new(Signer::new(key, secret, true)?),
            util: UtilApi::new(Signer::new(key, secret, true)?),
        })
    }

    /// Creates a new instance of a Client using a configuration file. This is a wrapper for the various APIs and Signer.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration that implements ConfigFile trait.
    #[cfg(feature = "config")]
    pub fn from_config<T>(config: &T) -> CbResult<Self>
    where
        T: ConfigFile,
    {
        Self::new(&config.coinbase().api_key, &config.coinbase().api_secret)
    }
}
