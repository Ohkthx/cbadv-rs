//! # Account API Example, check out the Account API for all functionality.
//!
//! Shows how to:
//! - Obtain multiple accounts.
//! - Obtain specific account by ID (Product Name)
//! - Obtain specific account by UUID.

use std::process::exit;

use cbadv::account::AccountListQuery;
use cbadv::config::{self, BaseConfig};
use cbadv::RestClientBuilder;

#[tokio::main]
async fn main() {
    let product_name: &str = "BTC";

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

    // Pull accounts by ID.
    println!("Obtaining account by ID (non-standard).");
    match client
        .account
        .get_by_id(product_name, &AccountListQuery::new())
        .await
    {
        Ok(account) => println!("{:#?}", account),
        Err(error) => println!("Unable to get account: {}", error),
    }

    // Pull accounts by ID.
    let mut account_uuid = "".to_string();
    println!("\n\nObtaining ALL accounts (non-standard).");
    match client.account.get_all(&AccountListQuery::new()).await {
        Ok(accounts) => {
            println!("Obtained {:#?} accounts.", accounts.len());

            // Find the UUID of an account to pull at the end.
            match accounts.iter().position(|r| r.currency == product_name) {
                Some(index) => {
                    let account = accounts.get(index).unwrap();
                    account_uuid = account.uuid.clone();
                }
                None => println!("Out of bounds, could not find account."),
            }
        }
        Err(error) => println!("Unable to get accounts: {}", error),
    }

    // Parameters to send to the API.
    let query = AccountListQuery::new();

    // Pull all accounts.
    println!("\n\nObtaining Bulk Accounts.");
    match client.account.get_bulk(&query).await {
        Ok(accounts) => {
            println!("Accounts obtained: {:#?}", accounts.accounts.len());
            for acct in accounts.accounts.iter() {
                println!("Account name: {}", acct.currency);
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
