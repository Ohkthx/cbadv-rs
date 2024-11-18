//! # Public API Example, check out the Public API for all functionality.
//!
//! Shows how to:
//! - Obtain the API Unix time.
//! - Obtain the Product Book for a product.
//! - Obtain multiple products.
//! - Obtain candles for a product.
//! - Obtain the ticker for a product.

use std::process::exit;

use cbadv::config::{self, BaseConfig};
use cbadv::product::{ListProductsQuery, TickerQuery};
use cbadv::{time, PublicRestClient};

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
    let mut client = match PublicRestClient::from_config(&config) {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };

    // Get API Unix time.
    println!("Obtaining API Unix time");
    match client.public.server_time().await {
        Ok(time) => println!("{:#?}", time),
        Err(error) => println!("Unable to get the Unix time: {}", error),
    }

    // NOTE: Commented out due to large amounts of output.
    // Get the Product Book for BTC-USD.
    // println!("\n\nObtain the Product Book for {product_pair}.");
    // match client.public.product_book(product_pair, None).await {
    //     Ok(book) => println!("{:#?}", book),
    //     Err(error) => println!("Unable to get the Product Book: {}", error),
    // }

    println!("\n\nGetting multiple products.");
    let query = ListProductsQuery {
        // limit: Some(500),
        // product_ids: Some(vec!["BTC-USD".to_string(), "ETH-USD".to_string()]),
        // get_all_products: Some(true),
        ..Default::default()
    };

    // Pull multiple products from the Product API.
    match client.public.products(&query).await {
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
    match client.public.candles_ext(product_pair, &time_span).await {
        Ok(candles) => {
            println!("Obtained {} candles.", candles.len());
            match candles.first() {
                Some(candle) => println!("{:#?}", candle),
                None => println!("Out of bounds, no candles obtained."),
            }
        }
        Err(error) => println!("Unable to get candles: {}", error),
    }

    // Pull ticker.
    println!("\n\nGetting ticker for: {}.", product_pair);
    let query = TickerQuery { limit: 200 };
    match client.public.ticker(product_pair, &query).await {
        Ok(ticker) => {
            println!(
                "best bid: {:#?}\nbest ask: {:#?}\ntrades: {:#?}",
                ticker.best_bid,
                ticker.best_ask,
                ticker.trades.len()
            );
            match ticker.trades.first() {
                Some(trade) => println!("{:#?}", trade),
                None => println!("Out of bounds, no trades available."),
            }
        }
        Err(error) => println!("Unable to get ticker: {}", error),
    }
}
