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

use cbadv::config::{self, BaseConfig};
use cbadv::order::{OrderCreateBuilder, OrderEditRequest, OrderSide, OrderType, TimeInForce};
use cbadv::RestClientBuilder;

#[tokio::main]
async fn main() {
    let product_pair: &str = "BTC-USD";
    let total_size: f64 = 0.005;
    let price: f64 = 100.00;
    let side: OrderSide = OrderSide::Buy;

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
    let mut client = match RestClientBuilder::new()
        .with_config(&config)
        .use_sandbox(true)
        .build()
    {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };

    // Create an order request using the `OrderCreateBuilder`.
    // This example creates a Limit Order that is Good-Til-Cancelled (GTC) and post-only.
    let order = match OrderCreateBuilder::new(product_pair, &side)
        .base_size(total_size)
        .limit_price(price)
        .post_only(true)
        .order_type(OrderType::Limit)
        .time_in_force(TimeInForce::GoodUntilCancelled)
        .preview(true)
        .build()
    {
        Ok(order) => order,
        Err(error) => {
            println!("Unable to build order: {}", error);
            exit(1);
        }
    };

    println!("Creating Order for {}.", product_pair);
    match client.order.create(&order).await {
        Ok(summary) => println!("Order creation result: {:#?}", summary),
        Err(error) => println!("Unable to create order: {}", error),
    }

    println!("\n\nPreviewing an order creation.");
    match client.order.preview_create(&order).await {
        Ok(summary) => println!("Order preview result: {:#?}", summary),
        Err(error) => println!("Unable to preview order: {}", error),
    }

    println!("\n\nPreviewing an order edit.");
    let edit_preview = OrderEditRequest::new("order_id", 100.00, 0.005);
    match client.order.preview_edit(&edit_preview).await {
        Ok(summary) => println!("Order edit preview result: {:#?}", summary),
        Err(error) => println!("Unable to preview order edit: {}", error),
    }
}
