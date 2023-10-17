//! # Product API Example, check out the Product API for all functionality.
//!
//! Shows how to:
//! - Obtain multiple products.
//! - Obtain specific product by ID (Pair)
//! - Obtain best bids and asks for multiple products.
//! - Obtain candles for specific product.
//! - Obtain ticker (Market Trades) for specific product.

use cbadv::config::{self, BaseConfig};
use cbadv::product::{ListProductsQuery, TickerQuery};
use cbadv::rest::RestClient;
use cbadv::time;
use std::process::exit;

#[tokio::main]
async fn main() {
    let product_pair: &str = "BTC-USD";

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
        // limit: Some(500),
        // product_ids: Some(vec!["BTC-USD".to_string(), "ETH-USD".to_string()]),
        ..Default::default()
    };

    // Pull multiple products from the Product API.
    match client.product.get_bulk(&query).await {
        Ok(products) => println!("Obtained {:#?} products", products.len()),
        Err(error) => println!("Unable to get products: {}", error),
    }

    // Pull candles.
    println!("\n\nGetting candles for: {}.", product_pair);
    let granularity = time::Granularity::OneDay;
    let interval = time::Granularity::to_secs(&granularity) as u64;
    let end = time::now();
    let start = time::before(end, interval * 730);
    let time_span = time::Span::new(start, end, &granularity);
    println!("Intervals collecting: {}", time_span.count());
    match client
        .product
        .candles_ext(product_pair.clone(), &time_span)
        .await
    {
        Ok(candles) => {
            println!("Obtained {} candles.", candles.len());
            match candles.get(0) {
                Some(candle) => println!("{:#?}", candle),
                None => println!("Out of bounds, no candles obtained."),
            }
        }
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
