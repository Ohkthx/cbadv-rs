//! # Watch Candle Example
//!
//! Shows how to:
//! - Create a user-defined struct that receives updates.
//! - Implement required trait `CandleCallback` for the user-defined struct.
//! - Initialize and watch candles via WebSocket.
//! - Process candles coming from API.

use std::process::exit;

use cbadv::product::{Candle, ListProductsQuery};
use cbadv::traits::CandleCallback;
use cbadv::{PublicRestClient, WebSocketClientBuilder};

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
            "{:<5} {:>14} ({}): finished candle {}",
            self.processed, product_id, candle.start, is_same
        );
    }
}

/// Obtain product names of candles to be obtained.
async fn get_products(client: &mut PublicRestClient) -> Vec<String> {
    println!("Getting '*-USDC' products.");
    let query = ListProductsQuery {
        ..Default::default()
    };

    // Holds all of the product names.
    let mut product_names: Vec<String> = vec![];

    // Pull multiple products from the Product API.
    match client.public.products(&query).await {
        Ok(products) => {
            product_names = products
                .iter()
                // Filter products to only containing *-USDC pairs.
                .filter_map(|p| match p.quote_currency_id.as_str() {
                    "USDC" => Some(p.product_id.clone()),
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
    // Create a client to interact with the API.
    let mut rclient = match PublicRestClient::new(false) {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };

    // Create a client to interact with the API.
    let mut wsclient = match WebSocketClientBuilder::new().build() {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };

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
