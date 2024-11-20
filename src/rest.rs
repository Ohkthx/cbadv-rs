//! # Coinbase Advanced Client
//!
//! `rest` gives access to all of the APIs via `Client` for the Coinbase Advanced API.
//! This is the primary method of accessing the endpoints and handles all of the configurations and
//! negotiations for the user.

use crate::apis::{
    AccountApi, ConvertApi, DataApi, FeeApi, OrderApi, PaymentApi, PortfolioApi, ProductApi,
    PublicApi,
};
use crate::http_agent::{PublicHttpAgent, SecureHttpAgent};

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
    /// Gives access to the Portfolio API.
    pub portfolio: PortfolioApi,
    /// Gives access to the Convert API.
    pub convert: ConvertApi,
    /// Gives access to the Payment API.
    pub payment: PaymentApi,
    /// Gives access to the Data API.
    pub data: DataApi,
    /// Gives access to the Public API.
    pub public: PublicApi,
}

impl RestClient {
    /// Creates a new instance of a Client. This is a wrapper for the various APIs.
    ///
    /// # Arguments
    ///
    /// * `key` - A string that holds the key for the API service.
    /// * `secret` - A string that holds the secret for the API service.
    /// * `use_sandbox` - A boolean that determines if the sandbox should be used.
    pub fn new(key: &str, secret: &str, use_sandbox: bool) -> CbResult<Self> {
        let agent = SecureHttpAgent::new(key, secret, true, use_sandbox)?;

        Ok(Self {
            account: AccountApi::new(agent.clone()),
            product: ProductApi::new(agent.clone()),
            fee: FeeApi::new(agent.clone()),
            order: OrderApi::new(agent.clone()),
            portfolio: PortfolioApi::new(agent.clone()),
            convert: ConvertApi::new(agent.clone()),
            payment: PaymentApi::new(agent.clone()),
            data: DataApi::new(agent),
            public: PublicApi::new(PublicHttpAgent::new(true, use_sandbox)?),
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
        Self::new(
            &config.coinbase().api_key,
            &config.coinbase().api_secret,
            config.coinbase().use_sandbox,
        )
    }
}

/// Represents a Public Client for the API.
pub struct PublicRestClient {
    /// Gives access to the Public API.
    pub public: PublicApi,
}

impl PublicRestClient {
    /// Creates a new instance of a Client. This is a wrapper for the various APIs.
    ///
    /// # Arguments
    ///
    /// * `use_sandbox` - A boolean that determines if the sandbox should be used.
    fn new(use_sandbox: bool) -> CbResult<Self> {
        Ok(Self {
            public: PublicApi::new(PublicHttpAgent::new(true, use_sandbox)?),
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
        Self::new(config.coinbase().use_sandbox)
    }
}
