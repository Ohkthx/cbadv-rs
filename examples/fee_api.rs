//! # Fee API Example, check out the Fee API for all functionality.
//!
//! Shows how to:
//! - Obtain Transaction Summary / Fees

use cbadv::fee::TransactionSummaryQuery;
use cbadv::{config, rest};
use std::process::exit;

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config: config::Config = match config::load("config.toml") {
        Ok(c) => c,
        Err(_) => {
            println!("Could not load configuration file.");
            if config::exists("config.toml") {
                println!("Make sure it is a valid configuration file.");
                exit(1);
            }

            // Create a new configuration file.
            config::new().save("config.toml").unwrap();
            println!("Empty configuration file created, please update it.");
            exit(1);
        }
    };

    // Create a client to interact with the API.
    let client = rest::Client::new(&config.cb_api_key, &config.cb_api_secret);

    // Parameters to send to the API.
    let params = TransactionSummaryQuery::default();

    // Get fee transaction summary.
    println!("Obtaining Transaction Fee Summary");
    match client.fee.get(&params).await {
        Ok(summary) => println!("{:#?}", summary),
        Err(error) => println!("Unable to get the Transaction Summary: {}", error),
    }
}
