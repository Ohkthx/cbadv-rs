use crate::cbadv::account::ListAccountsParams;
use crate::cbadv::client::Client;
use crate::cbadv::fee::TransactionSummaryParams;
use crate::cbadv::order::{ListOrdersParams, OrderSide};
use crate::cbadv::product::{ListProductsParams, TickerParams};
use crate::cbadv::time;
pub mod cbadv;
pub mod config;

#[allow(dead_code)]
async fn check_product_api(client: &Client, product_pair: &String) {
    // Pull a singular product from the Product API.
    let product = client.product.get(product_pair.clone()).await.unwrap();
    println!("{:#?}\n\n", product);

    // Pull multiple products from the Product API.
    let params = ListProductsParams {
        limit: Some(5),
        product_type: Some("SPOT".to_string()),
        ..Default::default()
    };

    match client.product.get_all(params).await {
        Ok(products) => match products.get(1) {
            Some(value) => println!("{:#?}\n\n", value),
            None => println!("Out of bounds."),
        },
        Err(error) => {
            println!("\n\nTHIS IS THE ERROR LARGE: {}", error);
        }
    }

    // Pull candles.
    let end = time::now();
    let start = time::before(end, 60 * 300);
    let time_span = time::Span::new(start, end, time::Granularity::OneMinute);
    match client
        .product
        .candles(product_pair.clone(), time_span)
        .await
    {
        Ok(candles) => match candles.get(0) {
            Some(candle) => println!("{:#?}\n\n", candle),
            None => println!("Out of bounds."),
        },
        Err(error) => {
            println!("\n\nIN-MAIN ERROR: {}", error);
        }
    }

    // Pull ticker.
    let params = TickerParams { limit: 200 };
    match client.product.ticker(product_pair.clone(), params).await {
        Ok(ticker) => {
            println!(
                "best bid: {:#?}\nbest ask: {:#?}\ntrades: {:#?}",
                ticker.best_bid,
                ticker.best_ask,
                ticker.trades.len()
            );
            match ticker.trades.get(0) {
                Some(trade) => println!("{:#?}\n\n", trade),
                None => println!("Out of bounds."),
            }
        }
        Err(error) => {
            println!("\n\nIN-MAIN ERROR: {}", error);
        }
    }
}

#[allow(dead_code)]
async fn check_account_api(client: &Client, product_name: &String) -> String {
    // Pull accounts.
    let params = ListAccountsParams {
        // limit: 50,
        // cursor: "".to_string(),
        ..Default::default()
    };

    let mut account_uuid = "".to_string();
    match client.account.get_all(params).await {
        Ok(accounts) => {
            println!("Accounts obtained: {:#?}", accounts.accounts.len());
            let index = accounts
                .accounts
                .iter()
                .position(|r| &r.currency == product_name)
                .unwrap();

            match accounts.accounts.get(index) {
                Some(account) => {
                    account_uuid = account.uuid.clone();
                    println!("{:#?}\n\n", account);
                }
                None => println!("Out of bounds."),
            }
        }
        Err(error) => {
            println!("\n\nIN-MAIN ERROR: {}", error);
        }
    }

    // Get a singular account based on the UUID.
    println!("Obtaining Account: {}", account_uuid);
    match client.account.get(account_uuid.clone()).await {
        Ok(account) => {
            println!("{:#?}\n\n", account);
            account.available_balance.value.clone()
        }
        Err(error) => {
            println!("\n\nIN-MAIN ERROR: {}", error);
            "0".to_string()
        }
    }
}

#[allow(dead_code)]
async fn check_fee_api(client: &Client) {
    // Get fee transaction summary.
    println!("Obtaining Transaction Fee Summary");
    let params = TransactionSummaryParams {
        ..Default::default()
    };
    match client.fee.get(params).await {
        Ok(summary) => {
            println!("{:#?}\n\n", summary);
        }
        Err(error) => {
            println!("\n\nIN-MAIN ERROR: {}", error);
        }
    }
}

#[allow(dead_code)]
async fn check_order_api(client: &Client, product_pair: String, total_size: String, cancel: bool) {
    if !cancel {
        println!("Creating Order");
        match client
            .order
            .create_limit_gtc(
                &product_pair,
                &total_size,
                &"10.00".to_string(),
                &"SELL".to_string(),
                true,
            )
            .await
        {
            Ok(summary) => {
                println!("Order creation result: {:#?}\n\n", summary);
            }
            Err(error) => {
                println!("\n\nIN-MAIN ERROR: {}", error);
            }
        }
    }

    println!("Obtaining Orders, cancelling OPEN orders.");
    let mut order_id = "".to_string();
    let params = ListOrdersParams {
        product_id: Some(product_pair),
        order_side: Some(OrderSide::BUY),
        ..Default::default()
    };
    println!("Order params: {}", params.to_params());

    match client.order.get_all(params).await {
        Ok(orders) => {
            println!("Orders obtained: {:#?}\n\n", orders.orders.len());
            match orders.orders.get(0) {
                Some(order) => {
                    order_id = order.order_id.clone();
                    println!("{:#?}\n\n", order);
                }
                None => println!("Out of bounds."),
            }

            // Build list of orders to cancel.
            let mut order_ids: Vec<String> = vec![];
            for order in orders.orders {
                if order.status == "OPEN" {
                    order_ids.push(order_id.clone());
                }
            }

            // Cancel the orders.
            if cancel && order_ids.len() > 0 {
                println!("Cancelling open orders...");
                match client.order.cancel(order_ids).await {
                    Ok(summary) => {
                        println!("Order cancel result: {:#?}\n\n", summary);
                    }
                    Err(error) => {
                        println!("\n\nIN-MAIN ERROR: {}", error);
                    }
                }
            }
        }
        Err(error) => {
            println!("\n\nIN-MAIN ERROR: {}", error);
        }
    }

    // Get a singular order based on the ID.
    println!("Obtaining Order: {}", order_id);
    match client.order.get(order_id.clone()).await {
        Ok(order) => {
            println!("{:#?}\n\n", order);
        }
        Err(error) => {
            println!("\n\nIN-MAIN ERROR: {}", error);
        }
    }
}

#[tokio::main]
async fn main() {
    let product_name: String = "DOT".to_string();
    let product_pair: String = format!("{}-USD", product_name);
    // Load the configuration file.
    let config = config::Config::read("config.toml").unwrap();

    // Create a client to interact with the API.
    let client = Client::new(config.cb_api_key, config.cb_api_secret);

    // Check the product api.
    // check_product_api(&client, &product_pair).await;

    // Check the product api.
    let total_size: String = check_account_api(&client, &product_name).await;

    // Check the fee api.
    // check_fee_api(&client).await;

    // Check the fee api.
    check_order_api(&client, product_pair, total_size, true).await;
}
