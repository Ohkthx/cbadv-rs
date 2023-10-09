//! # Custom Configuration File usage.
//!
//! Shows how to:
//! - Define a custom configuration file and use it with the API.

use cbadv::{config, rest, traits::ConfigFile};
use serde::{Deserialize, Serialize};
use std::process::exit;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MyConfig {
    pub version: u8,
    pub cb_api_key: String,
    pub cb_api_secret: String,
    pub debug: bool,
    pub product_id: String,
}

impl MyConfig {
    pub fn new() -> Self {
        Self {
            cb_api_key: "YOUR_COINBASE_API_KEY_HERE".to_string(),
            cb_api_secret: "YOUR_COINBASE_API_SECRET_HERE".to_string(),
            product_id: "BTC-USD".to_string(),
            ..Default::default()
        }
    }
}

impl ConfigFile for MyConfig {
    /// API Key provided by the service.
    fn cb_api_key(&self) -> &str {
        &self.cb_api_key
    }

    /// API Secret provided by the service.
    fn cb_api_secret(&self) -> &str {
        &self.cb_api_secret
    }
}

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config: MyConfig = match config::load("config.toml") {
        Ok(c) => c,
        Err(_) => {
            println!("Could not load configuration file.");
            if config::exists("config.toml") {
                println!("Make sure it is a valid configuration file.");
                exit(1);
            }

            // Create a new configuration file with defaults.
            config::save(&MyConfig::new(), "config.toml").unwrap();
            println!("Empty configuration file created, please update it.");
            exit(1);
        }
    };

    // Create a client to interact with the API.
    let client = rest::Client::from_config(&config);

    // Pull a singular product from the Product API.
    println!("Getting product: {}.", config.product_id);
    let product = match client.product.get(&config.product_id).await {
        Ok(p) => p,
        Err(err) => {
            println!("{}", err);
            exit(1);
        }
    };
    println!("{:#?}\n\n", product);
}
