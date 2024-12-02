//! # Custom Configuration File usage.
//!
//! Shows how to:
//! - Define a custom configuration file and use it with the API.

use std::process::exit;

use serde::{Deserialize, Serialize};

use cbadv::config::{self, ApiConfig, ConfigFile};
use cbadv::RestClientBuilder;

/// `[general]` section in the configuration file.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct GeneralConfig {
    /// Product being obtained for testing purposes.
    pub product_id: String,
}

/// Custom configuration with a `[general]` section containing a `product_id`.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MyConfig {
    /// `[general]` section.
    pub general: GeneralConfig,
    /// `[coinbase]` section of the configuration.
    pub coinbase: ApiConfig,
}

impl MyConfig {
    /// Creates a new instance of the custom configuration.
    pub fn new() -> Self {
        let general = GeneralConfig {
            product_id: "BTC-USD".to_string(),
        };

        Self {
            general,
            coinbase: config::new(),
        }
    }
}

impl ConfigFile for MyConfig {
    fn coinbase(&self) -> &ApiConfig {
        &self.coinbase
    }
}

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config: MyConfig = match config::load("config.toml") {
        Ok(c) => c,
        Err(err) => {
            println!("Could not load configuration file.");
            if config::exists("config.toml") {
                println!("File exists, {err}");
                exit(1);
            }

            // Create a new configuration file with defaults.
            config::save(&MyConfig::new(), "config.toml").unwrap();
            println!("Empty configuration file created, please update it.");
            exit(1);
        }
    };

    // Create a client to interact with the API.
    let mut client = match RestClientBuilder::new().with_config(&config).build() {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {why}");
            exit(1)
        }
    };

    // Pull a singular product from the Product API.
    println!("Getting product: {}.", config.general.product_id);
    let product = match client.product.get(&config.general.product_id).await {
        Ok(p) => p,
        Err(err) => {
            println!("{err}");
            exit(1);
        }
    };
    println!("{product:#?}\n\n");
}
