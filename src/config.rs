use serde::{Deserialize, Serialize};
use std::fs;
use toml;

/// Configuration for API settings. Loaded from a file.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub cb_api_key: String,
    pub cb_api_secret: String,
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
    pub fn read(path: &str) -> Result<Self, toml::de::Error> {
        let contents = fs::read_to_string(path);
        let config: Config = toml::from_str(&contents.unwrap())?;
        Ok(config)
    }

    /// Writes a configuration to a given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the location for the file.
    pub fn write(&self, path: &str) -> Result<(), std::io::Error> {
        let contents = toml::to_string_pretty(self);
        fs::write(path, contents.unwrap())?;
        Ok(())
    }
}
