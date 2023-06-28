use cbadv::order::{ListOrdersParams, OrderSide};
use cbadv::{client, config};

#[tokio::main]
async fn main() {
    let create_trade: bool = false;
    let cancel_open_orders: bool = false;
    let product_pair: &str = "DOT-USD";
    let total_size: &str = "3.00";
    let price: &str = "10.00";
    let side: &str = "SELL";

    // Load the configuration file.
    let config = config::load("config.toml").unwrap();

    // Create a client to interact with the API.
    let client = client::new(&config.cb_api_key, &config.cb_api_secret);

    if create_trade {
        println!("Creating Order for {}.", product_pair);
        match client
            .order
            .create_limit_gtc(product_pair, total_size, price, side, true)
            .await
        {
            Ok(summary) => println!("Order creation result: {:#?}", summary),
            Err(error) => println!("Unable to create order: {}", error),
        }
    }

    println!("\n\nCancelling all OPEN orders for {}.", product_pair);
    match client.order.cancel_all(&product_pair).await {
        Ok(result) => println!("{:#?}", result),
        Err(error) => println!("Unable to cancel orders: {}", error),
    }

    println!("\n\nGetting all orders for {}.", product_pair);
    match client.order.get_all(product_pair, None).await {
        Ok(orders) => println!("Orders obtained: {:#?}", orders.len()),
        Err(error) => println!("Unable to obtain all orders: {}", error),
    }

    // Get all BUYING orders.
    let mut order_id = "".to_string();
    let params = ListOrdersParams {
        product_id: Some(product_pair.to_string()),
        order_side: Some(OrderSide::SELL),
        ..Default::default()
    };

    println!("\n\nObtaining Orders.");
    match client.order.get_bulk(&params).await {
        Ok(orders) => {
            println!("Orders obtained: {:#?}", orders.orders.len());
            match orders.orders.get(0) {
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
            if cancel_open_orders && order_ids.len() > 0 {
                println!("\n\nCancelling open orders.");
                match client.order.cancel(order_ids).await {
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
