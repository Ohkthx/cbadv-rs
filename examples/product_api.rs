//! # Product API Example, check out the Product API for all functionality.
//!
//! Shows how to:
//! - Obtain multiple products.
//! - Obtain specific product by ID (Pair)
//! - Obtain best bids and asks for multiple products.
//! - Obtain candles for specific product.
//! - Obtain ticker (Market Trades) for specific product.

use std::process::exit;

use cbadv::config::{self, BaseConfig};
use cbadv::product::{
    ProductBidAskQuery, ProductCandleQuery, ProductListQuery, ProductTickerQuery,
};
use cbadv::time::Granularity;
use cbadv::{time, RestClientBuilder};

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
    let mut client = match RestClientBuilder::new().with_config(&config).build() {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };

    // Pull a singular product from the Product API.
    println!("Getting product: {}.", product_pair);
    let product = client.product.get(product_pair).await.unwrap();
    println!("{:#?}\n\n", product);

    println!("Getting best bids and asks.");
    let query = ProductBidAskQuery::new().product_ids(&["BTC-USD".to_string()]);
    match client.product.best_bid_ask(&query).await {
        Ok(bidasks) => println!("{:#?}", bidasks),
        Err(error) => println!("Unable to get best bids and asks: {}", error),
    }

    // NOTE: Commented out due to large amounts of output.
    // println!("\n\nGetting product book.");
    // match client.product.product_book(product_pair, None).await {
    //     Ok(book) => println!("{:#?}", book),
    //     Err(error) => println!("Unable to get product book: {}", error),
    // }

    println!("\n\nGetting multiple products.");
    let query = ProductListQuery::new();

    // Pull multiple products from the Product API.
    match client.product.get_bulk(&query).await {
        Ok(products) => println!("Obtained {:#?} products", products.len()),
        Err(error) => println!("Unable to get products: {}", error),
    }

    // Pull candles.
    let end = time::now();
    let interval = Granularity::to_secs(&Granularity::OneDay) as u64;
    println!("\n\nGetting candles for: {}.", product_pair);
    let query = ProductCandleQuery::new(
        time::before(end, interval * 365),
        end,
        time::Granularity::OneDay,
    );

    match client.product.candles_ext(product_pair, &query).await {
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
    let query = ProductTickerQuery::new(200);
    match client.product.ticker(product_pair, &query).await {
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
