//! # Configuration file creation, loading, and saving.
//!
//! `config` helps manage the optional configuration file for the crate. This gives access to
//! loading the credentials for API access without hardcoding them into source code.

use crate::debugln;
use crate::traits::ConfigFile;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs;
use toml;

const CURRENT_CONFIG_VERSION: u8 = 1;

// NOTE: This should match `Config` except new fields are set as 'Option'.
//       Do not forget to update `Config::from_raw()`
/// Configuration used for loading and setting defaults.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct RawConfig {
    /// Version of the Configuration file.
    pub version: Option<u8>,
    /// API Key provided by the service.
    pub cb_api_key: String,
    /// API Secret provided by the service.
    pub cb_api_secret: String,
    /// Enable debug messages or not.
    pub debug: bool,
}

// NOTE: Do not forget to update `RawConfig`, `CURRENT_CONFIG_VERSION`,
//       and `Config::from_raw()` if new fields are added.
/// Configuration for API settings. Loaded from a file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Version of the Configuration file.
    pub version: u8,
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
            version: CURRENT_CONFIG_VERSION,
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
    pub fn load(path: &str) -> Result<Self, &str> {
        // Load the raw version.
        let raw_config: RawConfig = match fs::read_to_string(path) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(value) => value,
                Err(_) => return Err("unable to open the configuration file."),
            },
            Err(_) => return Err("unable to open the configuration file."),
        };

        // Convert the raw format to usable configuration file.
        Ok(Self::from_raw(&raw_config, path))
    }

    /// Saves a configuration to a given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the location for the file.
    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let contents = toml::to_string_pretty(self);
        fs::write(path, contents.unwrap())
    }

    /// Converts a `RawConfig` to `Config`, filling in the missing values with new defaults.
    ///
    /// # Arguments
    ///
    /// * `raw` - A raw representation of a configuration file.
    /// * `path` - A string slice that holds the location for the file to be save to.
    fn from_raw(raw: &RawConfig, path: &str) -> Self {
        let mut updated: bool = false;
        let mut config = Self::new();

        config.cb_api_key = raw.cb_api_key.clone();
        config.cb_api_secret = raw.cb_api_secret.clone();
        config.debug = raw.debug;

        // Set a missing default version.
        config.version = match raw.version {
            Some(version) => {
                if version != CURRENT_CONFIG_VERSION {
                    updated = true;
                }
                CURRENT_CONFIG_VERSION
            }
            None => {
                updated = true;
                CURRENT_CONFIG_VERSION
            }
        };

        // Save the updated fields.
        if updated {
            if config.save(path).is_err() {
                println!("could not save updated configuration file");
            } else if config.debug {
                debugln!("Configuration file updated with new defaults.");
            }
        }

        config
    }
}

impl ConfigFile for Config {
    /// API Key provided by the service.
    fn cb_api_key(&self) -> &str {
        &self.cb_api_key
    }

    /// API Secret provided by the service.
    fn cb_api_secret(&self) -> &str {
        &self.cb_api_key
    }
}

/// Creates the default configuration. Wraps `Client::new()`
pub fn new() -> Config {
    Config::new()
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
            Err(_) => return Err("unable to open the configuration file."),
        },
        Err(_) => return Err("unable to open the configuration file."),
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
