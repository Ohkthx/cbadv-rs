use crate::cbadv::client::Client;
pub mod cbadv;
pub mod config;

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config = config::Config::read("config.toml").unwrap();

    // Create a client to interact with the API.
    let client = Client::new(config.cb_api_key, config.cb_api_secret);

    // Pull a singular product from the Product API.
    let product = client.product.get("BTC-USD".to_string()).await.unwrap();
    println!("{:?}", product);
    println!("{}", product.product_id);

    // Pull multiple products from the Product API.
    let products = client.product.get_all().await.unwrap();
    match products.products.get(0) {
        Some(value) => println!("{:?}", value),
        None => println!("Out of bounds."),
    }
}
