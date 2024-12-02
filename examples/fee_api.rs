//! # Fee API Example, check out the Fee API for all functionality.
//!
//! Shows how to:
//! - Obtain Transaction Summary / Fees

use std::process::exit;

use cbadv::config::{self, BaseConfig};
use cbadv::models::fee::FeeTransactionSummaryQuery;
use cbadv::models::product::ProductType;
use cbadv::RestClientBuilder;

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config: BaseConfig = match config::load("config.toml") {
        Ok(c) => c,
        Err(err) => {
            println!("Could not load configuration file.");
            if config::exists("config.toml") {
                println!("File exists, {err}");
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
            eprintln!("!ERROR! {why}");
            exit(1)
        }
    };

    // Parameters to send to the API.
    let params = FeeTransactionSummaryQuery::new().product_type(ProductType::Spot);

    // Get fee transaction summary.
    println!("Obtaining Transaction Fee Summary");
    match client.fee.get(&params).await {
        Ok(summary) => println!("{summary:#?}"),
        Err(error) => println!("Unable to get the Transaction Summary: {error}"),
    }
}
