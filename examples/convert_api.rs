//! # Convert API Example, check out the Convert API for all functionality.
//!
//! Shows how to:
//! - Create a convert quote.
//! - Obtain a convert quote.

use std::process::exit;

use cbadv::config::{self, BaseConfig};
use cbadv::convert::{ConvertQuery, ConvertQuoteRequest};
use cbadv::RestClientBuilder;

#[tokio::main]
async fn main() {
    let from_product: &str = "USDC";
    let to_product: &str = "USD";
    let amount: f64 = 0.05;

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

    // Create a quote to convert USDC to USD.
    println!(
        "Creating a quote to convert {} {} to {}.",
        amount, from_product, to_product
    );
    let request = ConvertQuoteRequest::new(from_product, to_product, amount);
    let quote = match client.convert.create_quote(&request).await {
        Ok(q) => q,
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };

    println!("Quote created: {:#?}", quote);
    println!("\n\nObtain the quote with the quote_id: {}", quote.id);
    let query = ConvertQuery::new(from_product, to_product);
    match client.convert.get(&quote.id, &query).await {
        Ok(q) => {
            println!("Quote obtained: {:#?}", q);
        }
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };
}
