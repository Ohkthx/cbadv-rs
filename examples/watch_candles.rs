//! # Watch Candle Example
//!
//! Shows how to:
//! - Create a user-defined struct that receives updates.
//! - Implement required trait `CandleCallback` for the user-defined struct.
//! - Initialize and watch candles via WebSocket.
//! - Process candles coming from API.

use cbadv::config::{self, BaseConfig};
use cbadv::product::{Candle, ListProductsQuery};
use cbadv::rest::{self, RestClient};
use cbadv::websocket::{self, CandleCallback};

use std::process::exit;

/// Example of user-defined struct to pass to the candle watcher.
pub struct UserStruct {
    /// Total amount of candles seen.
    processed: usize,
}

impl CandleCallback for UserStruct {
    fn candle_callback(&mut self, current_start: u64, product_id: String, candle: Candle) {
        self.processed += 1;

        let mut is_same = "";
        if current_start == candle.start {
            is_same = "[MATCHES CURRENT START]";
        }

        // Processed | Product_Id | Candle Start | Current
        println!(
            "{:<5} {:>11} ({}): finished candle {}",
            self.processed, product_id, candle.start, is_same
        );
    }
}

/// Obtain product names of candles to be obtained.
async fn get_products(client: &mut RestClient) -> Vec<String> {
    println!("Getting '*-USD' products.");
    let query = ListProductsQuery {
        ..Default::default()
    };

    // Holds all of the product names.
    let mut product_names: Vec<String> = vec![];

    // Pull multiple products from the Product API.
    match client.product.get_bulk(&query).await {
        Ok(products) => {
            product_names = products
                .iter()
                // Filter products to only containing *-USD pairs.
                .filter_map(|p| match p.quote_currency_id.as_str() {
                    "USD" => Some(p.product_id.clone()),
                    _ => None,
                })
                .collect();
        }
        Err(error) => println!("Unable to get products: {}", error),
    }

    product_names
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // Load the configuration file.
    let config: BaseConfig = match config::load("config.toml") {
        Ok(c) => c,
        Err(err) => {
            println!("Could not load configuration file.");
            if config::exists("config.toml") {
                println!("File exists, {}", err);
                exit(1);
            }

            // Create a new configuration file with defaults.
            config::create_base_config("config.toml").unwrap();
            println!("Empty configuration file created, please update it.");
            exit(1);
        }
    };

    // Create clients to interact with the API.
    let mut rclient = rest::from_config(&config);
    let mut wsclient = websocket::from_config(&config);

    // Products of interest.
    let products = get_products(&mut rclient).await;
    println!("Obtained {} products.\n", products.len());

    // User struct to be passed to the watcher.
    let mystruct: UserStruct = UserStruct { processed: 0 };

    // Start watching candles.
    println!("Starting candle watcher for {} products.", products.len());
    // let task = match websocket::watch_candles(&mut wsclient, &products, mystruct).await {
    let task = match wsclient.watch_candles(&products, mystruct).await {
        Ok(value) => value,
        Err(err) => {
            println!("Could not watch candles: {}", err);
            exit(1);
        }
    };

    // Wait to join the task.
    match task.await {
        Ok(_) => println!("Task is complete."),
        Err(err) => println!("Task ended in error: {}", err),
    };

    Ok(())
}
