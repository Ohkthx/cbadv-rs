//! # Payment API Example, check out the Payment API for all functionality.
//!
//! Shows how to:
//! - Get all payment methods.
//! - Get a single payment method.

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

    let mut payment_method_id = None;

    // Get payment methods.
    println!("Obtaining all payment methods.");
    match client.payment.get_all().await {
        Ok(methods) => {
            println!("{:#?}", methods);
            if let Some(method) = methods.first() {
                payment_method_id = Some(method.id.clone());
            }
        }
        Err(error) => println!("Unable to get the Payment Methods: {}", error),
    }

    // Obtain a single payment method.
    if let Some(payment_method_id) = payment_method_id {
        // Get a single payment method.
        println!("\n\nObtaining a single payment method.");
        match client.payment.get(&payment_method_id).await {
            Ok(method) => println!("{:#?}", method),
            Err(error) => println!("Unable to get the Payment Method: {}", error),
        }
    }
}
