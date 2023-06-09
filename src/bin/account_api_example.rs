//! # Account API Example, check out the Account API for all functionality.
//!
//! Shows how to:
//! - Obtain multiple accounts.
//! - Obtain specific account by ID (Product Name)
//! - Obtain specific account by UUID.

use cbadv::account::ListAccountsQuery;
use cbadv::{config, rest};
use std::process::exit;

#[tokio::main]
async fn main() {
    let product_name: &str = "BTC";

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

    // Parameters to send to the API.
    let query = ListAccountsQuery {
        // limit: Some(50),
        ..Default::default()
    };

    // Pull accounts by ID.
    println!("Obtaining account by ID.");
    match client.account.get_by_id(&product_name, None).await {
        Ok(account) => println!("{:#?}", account),
        Err(error) => println!("Unable to get account: {}", error),
    }

    // Pull all accounts.
    println!("\n\nObtaining ALL Accounts.");
    let mut account_uuid = "".to_string();
    match client.account.get_bulk(&query).await {
        Ok(accounts) => {
            println!("Accounts obtained: {:#?}", accounts.accounts.len());
            for acct in accounts.accounts.iter() {
                println!("Account name: {}", acct.currency);
            }

            match accounts
                .accounts
                .iter()
                .position(|r| &r.currency == product_name)
            {
                Some(index) => {
                    println!("Account index: {}", index);
                    let account = accounts.accounts.get(index).unwrap();
                    account_uuid = account.uuid.clone();
                    println!("{:#?}", account);
                }
                None => println!("Out of bounds, could not find account."),
            }
        }
        Err(error) => println!("Unable to get all accounts: {}", error),
    }

    // Get a singular account based on the UUID.
    println!("\n\nObtaining Account by UUID: {}", account_uuid);
    match client.account.get(&account_uuid).await {
        Ok(account) => println!("{:#?}", account),
        Err(error) => println!("Unable to get account: {}", error),
    }
}
