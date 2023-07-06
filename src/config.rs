//! # Configuration file creation, loading, and saving.
//!
//! `config` helps manage the optional configuration file for the crate. This gives access to
//! loading the credentials for API access without hardcoding them into source code.

use serde::{Deserialize, Serialize};
use std::fs;
use toml;

/// Configuration for API settings. Loaded from a file.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// API Key provided by the service.
    pub cb_api_key: String,
    /// API Secret provided by the service.
    pub cb_api_secret: String,
    /// Enable debug messages or not.
    pub debug: bool,
}

impl Config {
    /// Creates the default configuration.
    pub fn new() -> Self {
        Self {
            cb_api_key: "YOUR_COINBASE_API_KEY_HERE".to_string(),
            cb_api_secret: "YOUR_COINBASE_API_SECRET_HERE".to_string(),
            debug: false,
        }
    }

    /// Loads a configuration from a given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the location for the file.
    pub fn load(path: &str) -> Result<Self, toml::de::Error> {
        let contents = fs::read_to_string(path);
        let config: Config = toml::from_str(&contents.unwrap())?;
        Ok(config)
    }

    /// Saves a configuration to a given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the location for the file.
    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let contents = toml::to_string_pretty(self);
        fs::write(path, contents.unwrap())?;
        Ok(())
    }
}

/// Creates the default configuration.
pub fn new() -> Config {
    Config::new()
}

/// Loads a configuration from a given path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the location for the file.
pub fn load(path: &str) -> Result<Config, toml::de::Error> {
    Config::load(path)
}
