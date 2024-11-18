//! # Sandbox API Example
//!
//! Shows how to:
//! - Create an order.
//! - Edit an order.
//! - Cancel all OPEN orders.
//! - Obtain ALL orders.
//! - Obtain multiple orders.
//! - Obtain specific order by ID.

use std::process::exit;

use cbadv::{order::OrderSide, RestClient};

#[tokio::main]
async fn main() {
    let product_pair: &str = "BTC-USD";
    let side = OrderSide::Buy;
    let size = 300.0;
    let price = 0.37;

    // Create a client to interact with the API.
    let mut client = match RestClient::new("", "", true) {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };

    println!("Creating Order for {}.", product_pair);
    match client
        .order
        .create_limit_gtc(product_pair, &side, &size, &price, true)
        .await
    {
        Ok(summary) => println!("Order creation result: {:#?}", summary),
        Err(error) => println!("Unable to create order: {}", error),
    }
}
