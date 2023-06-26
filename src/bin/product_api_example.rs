use cbadv::product::{ListProductsParams, TickerParams};
use cbadv::time;
use cbadv::{client, config};

#[tokio::main]
async fn main() {
    let product_pair: &str = "BTC-USD";

    // Load the configuration file.
    let config = config::load("config.toml").unwrap();

    // Create a client to interact with the API.
    let client = client::new(config.cb_api_key, config.cb_api_secret);

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

    println!("\n\nGetting product book.");
    match client
        .product
        .product_book(product_pair.clone(), None)
        .await
    {
        Ok(book) => println!("{:#?}", book),
        Err(error) => println!("Unable to get product book: {}", error),
    }

    println!("\n\nGetting multiple products.");
    let params = ListProductsParams {
        limit: Some(5),
        product_ids: Some(vec!["BTC-USD".to_string()]),
        ..Default::default()
    };

    // Pull multiple products from the Product API.
    match client.product.get_all(params).await {
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
        .candles(product_pair.clone(), time_span)
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
                Some(trade) => println!("{:#?}", trade),
                None => println!("Out of bounds, no trades available."),
            }
        }
        Err(error) => println!("Unable to get ticker: {}", error),
    }
}
