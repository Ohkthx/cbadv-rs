use crate::cbadv::client::Client;
use crate::cbadv::product::{ListProductParams, TickerParams};
use crate::cbadv::time;
pub mod cbadv;
pub mod config;

#[tokio::main]
async fn main() {
    let test_product: String = "BTC-USD".to_string();
    // Load the configuration file.
    let config = config::Config::read("config.toml").unwrap();

    // Create a client to interact with the API.
    let client = Client::new(config.cb_api_key, config.cb_api_secret);

    // Pull a singular product from the Product API.
    let product = client.product.get(test_product.clone()).await.unwrap();
    println!("{:#?}\n\n", product);

    // Pull multiple products from the Product API.
    let params = ListProductParams {
        limit: 5,
        offset: 0,
        product_type: "SPOT".to_string(),
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
        .candles(test_product.clone(), time_span)
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
    // Pull multiple products from the Product API.
    let params = TickerParams { limit: 200 };
    match client.product.ticker(test_product.clone(), params).await {
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
