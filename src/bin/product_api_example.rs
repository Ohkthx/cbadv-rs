//! # Product API Example, check out the Product API for all functionality.
//!
//! Shows how to:
//! - Obtain multiple products.
//! - Obtain specific product by ID (Pair)
//! - Obtain best bids and asks for multiple products.
//! - Obtain candles for specific product.
//! - Obtain ticker (Market Trades) for specific product.

use cbadv::product::{ListProductsQuery, TickerQuery};
use cbadv::time;
use cbadv::{config, rest};
use std::process::exit;

#[tokio::main]
async fn main() {
    let product_pair: &str = "BTC-USD";

    // Load the configuration file.
    let config: config::Config = match config::load("config.toml") {
        Ok(c) => c,
        Err(_) => {
            println!("Could not load configuration file.");
            if config::exists("config.toml") {
                println!("Make sure it is a valid configuration file.");
                exit(1);
            }

            // Create a new configuration file.
            config::new().save("config.toml").unwrap();
            println!("Empty configuration file created, please update it.");
            exit(1);
        }
    };

    // Create a client to interact with the API.
    let client = rest::Client::new(&config.cb_api_key, &config.cb_api_secret);

    // Pull a singular product from the Product API.
    println!("Getting product: {}.", product_pair);
    let product = client.product.get(product_pair.clone()).await.unwrap();
    println!("{:#?}\n\n", product);

    println!("Getting best bids and asks.");
    match client
        .product
        .best_bid_ask(vec!["BTC-USD".to_string()])
        .await
    {
        Ok(bidasks) => println!("{:#?}", bidasks),
        Err(error) => println!("Unable to get best bids and asks: {}", error),
    }

    // NOTE: Commented out due to large amounts of output.
    // println!("\n\nGetting product book.");
    // match client
    //     .product
    //     .product_book(product_pair.clone(), None)
    //     .await
    // {
    //     Ok(book) => println!("{:#?}", book),
    //     Err(error) => println!("Unable to get product book: {}", error),
    // }

    println!("\n\nGetting multiple products.");
    let query = ListProductsQuery {
        limit: Some(5),
        product_ids: Some(vec!["BTC-USD".to_string()]),
        ..Default::default()
    };

    // Pull multiple products from the Product API.
    match client.product.get_bulk(&query).await {
        Ok(products) => println!("{:#?}", products),
        Err(error) => println!("Unable to get products: {}", error),
    }

    // Pull candles.
    println!("\n\nGetting candles for: {}.", product_pair);
    let end = time::now();
    let start = time::before(end, 60 * 300);
    let time_span = time::Span::new(start, end, time::Granularity::OneMinute);
    match client
        .product
        .candles(product_pair.clone(), &time_span)
        .await
    {
        Ok(candles) => match candles.get(0) {
            Some(candle) => println!("{:#?}", candle),
            None => println!("Out of bounds, no candles obtained."),
        },
        Err(error) => println!("Unable to get candles: {}", error),
    }

    // Pull ticker.
    println!("\n\nGetting ticker for: {}.", product_pair);
    let query = TickerQuery { limit: 200 };
    match client.product.ticker(product_pair.clone(), &query).await {
        Ok(ticker) => {
            println!(
                "best bid: {:#?}\nbest ask: {:#?}\ntrades: {:#?}",
                ticker.best_bid,
                ticker.best_ask,
                ticker.trades.len()
            );
            match ticker.trades.get(0) {
                Some(trade) => println!("{:#?}", trade),
                None => println!("Out of bounds, no trades available."),
            }
        }
        Err(error) => println!("Unable to get ticker: {}", error),
    }
}
