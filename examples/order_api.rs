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
use std::thread;
use std::time::Duration;

use cbadv::config::{self, BaseConfig};
use cbadv::models::order::{
    OrderCancelRequest, OrderCreateBuilder, OrderCreateRequest, OrderEditRequest, OrderListQuery,
    OrderSide, OrderStatus, OrderType, TimeInForce,
};
use cbadv::{RestClient, RestClientBuilder};

fn init_client() -> RestClient {
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
    match RestClientBuilder::new().with_config(&config).build() {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {why}");
            exit(1)
        }
    }
}

async fn create_new_order(
    client: &mut RestClient,
    new_order: &OrderCreateRequest,
) -> Option<String> {
    let mut created_order_id: Option<String> = None;
    println!(
        "Creating Order with Client ID: {}",
        new_order.client_order_id
    );

    match client.order.create(new_order).await {
        Ok(summary) => {
            if let Some(success) = &summary.success_response {
                created_order_id = Some(success.order_id.clone());
            }
            println!("Order creation result: {summary:#?}");
        }
        Err(error) => println!("Unable to create order: {error}"),
    }

    created_order_id
}

async fn edit_created_order(client: &mut RestClient, order_id: &str) {
    let edit_order = OrderEditRequest::new(order_id, 50.0, 0.006);
    println!("\n\nEditing order for {order_id}.");
    match client.order.edit(&edit_order).await {
        Ok(result) => println!("{result:#?}"),
        Err(error) => println!("Unable to edit order: {error}"),
    }
}

async fn cancel_created_order(client: &mut RestClient, order_id: &str) {
    println!("\n\nCancelling Order with ID: {order_id}");
    match client
        .order
        .cancel(&OrderCancelRequest::new(&[order_id.to_string()]))
        .await
    {
        Ok(summary) => println!("Order cancel result: {summary:#?}"),
        Err(error) => println!("Unable to cancel order: {error}"),
    }
}

#[tokio::main]
async fn main() {
    let create_new: bool = false;
    let edit_created: bool = true;
    let cancel_created: bool = true;
    let cancel_all: bool = false;
    let product_id: &str = "ETH-USDC";
    let mut created_order_id: Option<String> = None;
    let new_order = match OrderCreateBuilder::new(product_id, OrderSide::Buy)
        .base_size(0.005)
        .limit_price(100.0)
        .post_only(true)
        .order_type(OrderType::Limit)
        .time_in_force(TimeInForce::GoodUntilCancelled)
        .build()
    {
        Ok(order) => order,
        Err(error) => {
            println!("Unable to build order: {error}");
            exit(1);
        }
    };

    let mut client = init_client();

    // Creates a new order from scratch, the resulting order id will be used for other operations.
    if create_new {
        created_order_id = create_new_order(&mut client, &new_order).await;
    }

    // Edits the created order.
    if let Some(order_id) = &created_order_id {
        if create_new && edit_created {
            thread::sleep(Duration::from_secs(1));
            edit_created_order(&mut client, order_id).await;
        }
    }

    // Cancels the created order.
    if let Some(order_id) = &created_order_id {
        if create_new && cancel_created {
            cancel_created_order(&mut client, order_id).await;
        }
    }

    // Cancels all OPEN orders.
    if cancel_all {
        println!("\n\nCancelling all OPEN orders for {product_id}.");
        match client.order.cancel_all(product_id).await {
            Ok(result) => println!("{result:#?}"),
            Err(error) => println!("Unable to cancel orders: {error}"),
        }
    }

    println!("\n\nGetting all orders for {product_id} (get_all).");
    match client
        .order
        .get_all(product_id, &OrderListQuery::new())
        .await
    {
        Ok(orders) => println!("Orders obtained: {:#?}", orders.len()),
        Err(error) => println!("Unable to obtain all orders: {error}"),
    }

    // Get all BUYING orders.
    let mut order_id = String::new();
    let query = OrderListQuery {
        product_ids: Some(vec![product_id.to_string()]),
        order_side: Some(OrderSide::Buy),
        ..Default::default()
    };

    println!("\n\nObtaining Orders (bulk).");
    match client.order.get_bulk(&query).await {
        Ok(orders) => {
            println!("Orders obtained: {:#?}", orders.orders.len());
            match orders.orders.first() {
                Some(order) => {
                    order_id.clone_from(&order.order_id);
                    println!("{order:#?}");
                }
                None => println!("Out of bounds, no orders exist."),
            }

            // Build list of orders to cancel.
            let mut order_ids: Vec<String> = vec![];
            for order in orders.orders {
                if order.status == OrderStatus::Open {
                    order_ids.push(order.order_id);
                }
            }

            // Cancel the orders.
            if cancel_all && !order_ids.is_empty() {
                println!("\n\nCancelling open orders.");
                match client
                    .order
                    .cancel(&OrderCancelRequest::new(&order_ids))
                    .await
                {
                    Ok(summary) => println!("Order cancel result: {summary:#?}"),
                    Err(error) => println!("Unable to cancel order: {error}"),
                }
            }
        }
        Err(error) => println!("Unable to obtain bulk or cancel orders: {error}"),
    }

    // Get a singular order based on the ID.
    println!("\n\nObtaining single order: {order_id}");
    match client.order.get(&order_id).await {
        Ok(order) => println!("{order:#?}"),
        Err(error) => println!("Unable to get single order: {error}"),
    }
}
