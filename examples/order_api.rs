//! # Order API Example, check out the Order API for all functionality.
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
use cbadv::order::{ListOrdersQuery, OrderSide};
use cbadv::RestClient;

#[tokio::main]
async fn main() {
    let create_trade: bool = false;
    let cancel_open_orders: bool = false;
    let edit_open_order_id: Option<String> = None;
    let product_pair: &str = "DOGE-USD";
    let total_size: f64 = 300.0;
    let price: f64 = 100.00;
    let edit_price: f64 = 50.00;
    let side: &str = "SELL";

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
    let mut client = RestClient::from_config(&config);

    if create_trade {
        println!("Creating Order for {}.", product_pair);
        match client
            .order
            .create_limit_gtc(product_pair, side, &total_size, &price, true)
            .await
        {
            Ok(summary) => println!("Order creation result: {:#?}", summary),
            Err(error) => println!("Unable to create order: {}", error),
        }
    }

    if let Some(order_id) = edit_open_order_id {
        println!("\n\nEditing order for {}.", order_id);
        match client.order.edit(&order_id, total_size, edit_price).await {
            Ok(result) => println!("{:#?}", result),
            Err(error) => println!("Unable to edit order: {}", error),
        }
    }

    if cancel_open_orders {
        println!("\n\nCancelling all OPEN orders for {}.", product_pair);
        match client.order.cancel_all(product_pair).await {
            Ok(result) => println!("{:#?}", result),
            Err(error) => println!("Unable to cancel orders: {}", error),
        }
    }

    println!("\n\nGetting all orders for {}.", product_pair);
    match client.order.get_all(product_pair, None).await {
        Ok(orders) => println!("Orders obtained: {:#?}", orders.len()),
        Err(error) => println!("Unable to obtain all orders: {}", error),
    }

    // Get all SELLING orders.
    let mut order_id = "".to_string();
    let query = ListOrdersQuery {
        product_id: Some(product_pair.to_string()),
        order_side: Some(OrderSide::Sell),
        ..Default::default()
    };

    println!("\n\nObtaining Orders.");
    match client.order.get_bulk(&query).await {
        Ok(orders) => {
            println!("Orders obtained: {:#?}", orders.orders.len());
            match orders.orders.first() {
                Some(order) => {
                    order_id = order.order_id.clone();
                    println!("{:#?}", order);
                }
                None => println!("Out of bounds, no orders exist."),
            }

            // Build list of orders to cancel.
            let mut order_ids: Vec<String> = vec![];
            for order in orders.orders {
                if order.status == "OPEN" {
                    order_ids.push(order.order_id);
                }
            }

            // Cancel the orders.
            if cancel_open_orders && !order_ids.is_empty() {
                println!("\n\nCancelling open orders.");
                match client.order.cancel(&order_ids).await {
                    Ok(summary) => println!("Order cancel result: {:#?}", summary),
                    Err(error) => println!("Unable to cancel order: {}", error),
                }
            }
        }
        Err(error) => println!("Unable to obtain bulk or cancel orders: {}", error),
    }

    // Get a singular order based on the ID.
    println!("\n\nObtaining single order: {}", order_id);
    match client.order.get(&order_id).await {
        Ok(order) => println!("{:#?}", order),
        Err(error) => println!("Unable to get single order: {}", error),
    }
}
