//! # Coinbase Advanced Client
//!
//! `rest` gives access to all of the APIs via `Client` for the Coinbase Advanced API.
//! This is the primary method of accessing the endpoints and handles all of the configurations and
//! negotiations for the user.

use std::sync::Arc;

use futures::lock::Mutex;

use crate::apis::{
    AccountApi, ConvertApi, DataApi, FeeApi, OrderApi, PaymentApi, PortfolioApi, ProductApi,
    PublicApi,
};
use crate::http_agent::{PublicHttpAgent, SecureHttpAgent};

#[cfg(feature = "config")]
use crate::config::ConfigFile;
use crate::token_bucket::{RateLimits, TokenBucket};
use crate::types::CbResult;

/// Builds a new REST Client (RestClient) that directly interacts with the Coinbase Advanced API.
#[derive(Default)]
pub struct RestClientBuilder {
    api_key: Option<String>,
    api_secret: Option<String>,
    use_sandbox: bool,
}

impl RestClientBuilder {
    /// Creates a new instance of a RestClientBuilder.
    pub fn new() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            use_sandbox: false,
        }
    }

    /// Uses the configuration file to set up the client.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration that implements ConfigFile trait.
    #[cfg(feature = "config")]
    pub fn with_config<T>(mut self, config: &T) -> Self
    where
        T: ConfigFile,
    {
        self.api_key = Some(config.coinbase().api_key.clone());
        self.api_secret = Some(config.coinbase().api_secret.clone());
        self.use_sandbox = config.coinbase().use_sandbox;
        self
    }

    /// Uses the provided key and secret to initialize the authentication.
    ///
    /// # Arguments
    ///
    /// * `key` - API key.
    /// * `secret` - API secret.
    pub fn with_authentication(mut self, key: &str, secret: &str) -> Self {
        self.api_key = Some(key.to_string());
        self.api_secret = Some(secret.to_string());
        self
    }

    /// Sets the use_sandbox flag for the client.
    ///
    /// # Arguments
    ///
    /// * `use_sandbox` - A boolean that determines if the sandbox should be enabled.
    pub fn use_sandbox(mut self, use_sandbox: bool) -> Self {
        self.use_sandbox = use_sandbox;
        self
    }

    /// Builds the RestClient.
    pub fn build(self) -> CbResult<RestClient> {
        // Initialize token buckets
        let secure_bucket = Arc::new(Mutex::new(TokenBucket::new(
            RateLimits::max_tokens(true, false),
            RateLimits::refresh_rate(true, false),
        )));

        let public_bucket = Arc::new(Mutex::new(TokenBucket::new(
            RateLimits::max_tokens(true, true),
            RateLimits::refresh_rate(true, true),
        )));

        // Determine if authentication is enabled.
        let is_authenticated = self.api_key.is_some() && self.api_secret.is_some();

        // Initialize agents.
        let secure_agent = if is_authenticated {
            Some(SecureHttpAgent::new(
                &self.api_key.unwrap(),
                &self.api_secret.unwrap(),
                self.use_sandbox,
                secure_bucket,
            )?)
        } else {
            None
        };

        // Public agent used to access public endpoints.
        let public_agent = PublicHttpAgent::new(self.use_sandbox, public_bucket)?;

        // Initialize APIs.
        Ok(RestClient {
            account: AccountApi::new(secure_agent.clone()),
            product: ProductApi::new(secure_agent.clone()),
            fee: FeeApi::new(secure_agent.clone()),
            order: OrderApi::new(secure_agent.clone()),
            portfolio: PortfolioApi::new(secure_agent.clone()),
            convert: ConvertApi::new(secure_agent.clone()),
            payment: PaymentApi::new(secure_agent.clone()),
            data: DataApi::new(secure_agent.clone()),
            public: PublicApi::new(public_agent),
        })
    }
}

/// Represents a REST Client for interacting with the Coinbase Advanced API.
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
