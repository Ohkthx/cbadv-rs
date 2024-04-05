//! # Util API Example, check out the Util API for all functionality.
//!
//! Shows how to:
//! - Obtain the API Unix time.

use std::process::exit;

use cbadv::config::{self, BaseConfig};
use cbadv::RestClient;

#[tokio::main]
async fn main() {
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
    let mut client = match RestClient::from_config(&config) {
        Ok(c) => c,
        Err(why) => {
            eprintln!("!ERROR! {}", why);
            exit(1)
        }
    };

    // Get API Unix time.
    println!("Obtaining API Unix time");
    match client.util.unixtime().await {
        Ok(time) => println!("{:#?}", time),
        Err(error) => println!("Unable to get the Unix time: {}", error),
    }
}
