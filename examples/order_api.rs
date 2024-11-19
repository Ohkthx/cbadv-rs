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
use cbadv::order::{CreateOrderBuilder, ListOrdersQuery, OrderSide, OrderType, TimeInForce};
use cbadv::RestClient;

#[tokio::main]
async fn main() {
    let create_trade: bool = false;
    let cancel_open_orders: bool = false;
    let edit_open_order_id: Option<String> = None;
    let product_pair: &str = "ETH-USDC";
    let total_size: f64 = 0.005;
    let price: f64 = 100.00;
    let edit_price: f64 = 50.00;
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
    let mut client = match RestClient::from_config(&config) {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };

    if create_trade {
        println!("Creating Order for {}.", product_pair);

        // Create an order request using the `CreateOrderBuilder`.
        // This example creates a Limit Order that is Good-Til-Cancelled (GTC) and post-only.
        let order = match CreateOrderBuilder::new(product_pair, &side)
            .base_size(total_size)
            .limit_price(price)
            .post_only(true)
            .order_type(OrderType::Limit)
            .time_in_force(TimeInForce::Gtc)
            .build()
        {
            Ok(order) => order,
            Err(error) => {
                println!("Unable to build order: {}", error);
                exit(1);
            }
        };

        match client.order.create(&order).await {
            Ok(summary) => println!("Order creation result: {:#?}", summary),
            Err(error) => println!("Unable to create order: {}", error),
        }
    }

    // Edit and open order.
    if let Some(order_id) = edit_open_order_id {
        println!("\n\nEditing order for {}.", order_id);
        match client.order.edit(&order_id, total_size, edit_price).await {
            Ok(result) => println!("{:#?}", result),
            Err(error) => println!("Unable to edit order: {}", error),
        }
    }

    // Cancels all OPEN orders.
    if cancel_open_orders {
        println!("\n\nCancelling all OPEN orders for {}.", product_pair);
        match client.order.cancel_all(product_pair).await {
            Ok(result) => println!("{:#?}", result),
            Err(error) => println!("Unable to cancel orders: {}", error),
        }
    }

    println!("\n\nGetting all orders for {} (get_all).", product_pair);
    match client.order.get_all(product_pair, None).await {
        Ok(orders) => println!("Orders obtained: {:#?}", orders.len()),
        Err(error) => println!("Unable to obtain all orders: {}", error),
    }

    // Get all BUYING orders.
    let mut order_id = "".to_string();
    let query = ListOrdersQuery {
        product_ids: Some(vec![product_pair.to_string()]),
        order_side: Some(OrderSide::Buy),
        ..Default::default()
    };

    println!("\n\nObtaining Orders (bulk).");
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
