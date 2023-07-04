use cbadv::fee::TransactionSummaryQuery;
use cbadv::{config, rest};

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config = config::load("config.toml").unwrap();

    // Create a client to interact with the API.
    let client = rest::Client::new(&config.cb_api_key, &config.cb_api_secret);

    // Parameters to send to the API.
    let params = TransactionSummaryQuery::default();

    // Get fee transaction summary.
    println!("Obtaining Transaction Fee Summary");
    match client.fee.get(&params).await {
        Ok(summary) => println!("{:#?}", summary),
        Err(error) => println!("Unable to get the Transaction Summary: {}", error),
    }
}
