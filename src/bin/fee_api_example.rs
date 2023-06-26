use cbadv::fee::TransactionSummaryParams;
use cbadv::{client, config};

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config = config::load("config.toml").unwrap();

    // Create a client to interact with the API.
    let client = client::new(config.cb_api_key, config.cb_api_secret);

    // Parameters to send to the API.
    let params = TransactionSummaryParams {
        ..Default::default()
    };

    // Get fee transaction summary.
    println!("Obtaining Transaction Fee Summary");
    match client.fee.get(params).await {
        Ok(summary) => println!("{:#?}", summary),
        Err(error) => println!("Unable to get the Transaction Summary: {}", error),
    }
}
