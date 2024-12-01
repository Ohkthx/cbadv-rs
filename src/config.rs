//! # Configuration file creation, loading, and saving.
//!
//! `config` helps manage the optional configuration file for the crate. This gives access to
//! loading the credentials for API access without hardcoding them into source code.

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs;
use toml;

const CURRENT_CONFIG_VERSION: u8 = 2;

/// Generic configuration file with the minimum requirements for API access.
/// This is used to implement on custom configurations and to be passed when
/// creating REST and WebSocket clients.
pub trait ConfigFile {
    /// `[coinbase]` section of the configuration for the API settings.
    fn coinbase(&self) -> &ApiConfig;
}

/// Configuration settings for API, this should be in either a custom user configuration or
/// in the BaseConfig. See `BaseConfig` or `config.toml.sample` for
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfig {
    /// Version of the Configuration file.
    pub version: u8,
    /// API Key provided by the service.
    pub api_key: String,
    /// API Secret provided by the service.
    pub api_secret: String,
    /// Enable debug messages or not.
    pub debug: bool,
    /// Use sandbox or not.
    pub use_sandbox: bool,
}

impl ApiConfig {
    /// Creates the default configuration.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            version: CURRENT_CONFIG_VERSION,
            api_key: "YOUR_COINBASE_API_KEY_HERE".to_string(),
            api_secret: "YOUR_COINBASE_API_SECRET_HERE".to_string(),
            debug: false,
            use_sandbox: false,
        }
    }
}

/// Base configuration to be saved or read, used in place of custom user configuration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseConfig {
    /// `[coinbase]` section of the configuration for the API settings.
    coinbase: ApiConfig,
}

impl Default for BaseConfig {
    fn default() -> Self {
        Self {
            coinbase: ApiConfig::new(),
        }
    }
}

impl ConfigFile for BaseConfig {
    /// `[coinbase]` section of the configuration for the API settings.
    fn coinbase(&self) -> &ApiConfig {
        &self.coinbase
    }
}

/// Creates the default configuration. Wraps `ApiConfig::new()`
pub fn new() -> ApiConfig {
    ApiConfig::new()
}

/// Creates a local base configuration file.
///
/// # Arguments
///
/// * `path` - A string slice that holds the location for the file.
pub fn create_base_config(path: &str) -> Result<(), std::io::Error> {
    let contents = toml::to_string_pretty(&BaseConfig::default());
    fs::write(path, contents.unwrap())
}

/// Saves a configuration to a given path.
///
/// # Arguments
///
/// * `config` - Configuration that implement ConfigFile trait.
/// * `path` - A string slice that holds the location for the file.
pub fn save<T>(config: &T, path: &str) -> Result<(), std::io::Error>
where
    T: ConfigFile + Serialize,
{
    let contents = toml::to_string_pretty(&config);
    fs::write(path, contents.unwrap())
}

/// Loads a configuration from a given path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the location for the file.
pub fn load<T>(path: &str) -> Result<T, &str>
where
    T: ConfigFile + DeserializeOwned,
{
    // Load the raw version.
    match fs::read_to_string(path) {
        Ok(contents) => match toml::from_str::<T>(&contents) {
            Ok(value) => Ok(value),
            Err(_) => Err(
                "unable to parse configuration, check the syntax or sample version for reference.",
            ),
        },
        Err(_) => Err("unable to open the configuration file."),
    }
}

/// Checks if the configuration file exists.
///
/// # Arguments
///
/// * `path` - A string slice that holds the location for the file.
pub fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
