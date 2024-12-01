//! # Data API Example, check out the Data API for all functionality.
//!
//! Shows how to:
//! - Obtain API Key Permissions.

use std::process::exit;

use cbadv::config::{self, BaseConfig};
use cbadv::RestClientBuilder;

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config: BaseConfig = match config::load("config.toml") {
        Ok(c) => c,
        Err(err) => {
            println!("Could not load configuration file.");
            if config::exists("config.toml") {
                println!("File exists, {}", err);
                exit(1);
            }

            // Create a new configuration file.
            config::create_base_config("config.toml").unwrap();
            println!("Empty configuration file created, please update it.");
            exit(1);
        }
    };

    // Create a client to interact with the API.
    let mut client = match RestClientBuilder::new().with_config(&config).build() {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };

    // Get the API key permissions.
    println!("Obtaining Key Permissions for the API key.");
    match client.data.key_permissions().await {
        Ok(perm) => println!("{:#?}", perm),
        Err(error) => println!("Unable to get the API key permissions: {}", error),
    }
}
