//! # Portfolio API Example, check out the Portfolio API for all functionality.
//!
//! Shows how to:
//! - Create a new portfolio.
//! - Edit an existing portfolio.
//! - Delete an existing portfolio.
//! - Obtain a list of portfolios.
//! - Obtain the breakdown of a portfolio.

use std::process::exit;

use cbadv::config::{self, BaseConfig};
use cbadv::models::portfolio::{
    PortfolioBreakdownQuery, PortfolioListQuery, PortfolioModifyRequest,
};
use cbadv::RestClientBuilder;

#[tokio::main]
async fn main() {
    // Set to None to not create.
    let create_portfolio_name = None;
    // let create_portfolio_name = Some("New Portfolio");

    // Set to None to not edit.
    let edit_portfolio_uuid = None;
    // let edit_portfolio_uuid = Some("AAAAAAAA-BBBB-CCCC-DDDDDDDDDDDD");
    let edit_portfolio_name = "DeleteMe";

    // Set to None to not delete.
    let delete_portfolio_uuid = None;
    // let delete_portfolio_uuid = Some("AAAAAAAA-BBBB-CCCC-DDDDDDDDDDDD");

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

    // Create a new portfolio.
    if let Some(name) = create_portfolio_name {
        println!("Creating Portfolio.");
        match client.portfolio.create(name).await {
            Ok(portfolio) => println!("{:#?}", portfolio),
            Err(error) => println!("Unable to create the portfolio: {}", error),
        }
    }

    // Edit an existing portfolio.
    if let Some(uuid) = edit_portfolio_uuid {
        println!("Editing Portfolio.");
        let request = PortfolioModifyRequest::new(edit_portfolio_name);
        match client.portfolio.edit(uuid, &request).await {
            Ok(portfolio) => println!("{:#?}", portfolio),
            Err(error) => println!("Unable to edit the portfolio: {}", error),
        }
    }

    // Delete an existing portfolio.
    if let Some(uuid) = delete_portfolio_uuid {
        println!("Deleting Portfolio.");
        match client.portfolio.delete(uuid).await {
            Ok(_) => println!("Portfolio deleted!"),
            Err(error) => println!("Unable to delete the portfolio: {}", error),
        }
    }

    // Parameters to send to the API.
    let query = PortfolioListQuery::new();

    // Get listed portfolios..
    println!("Obtaining Portfolios");
    let breakdown_uuid = match client.portfolio.get_all(&query).await {
        Ok(portfolios) => {
            println!("{:#?}", portfolios);
            Some(portfolios.first().unwrap().uuid.clone())
        }
        Err(error) => {
            println!("Unable to get the portfolios: {}", error);
            None
        }
    };

    // Get the breakdown for the first portfolio.
    if let Some(uuid) = breakdown_uuid {
        println!("Obtaining Portfolio Breakdown for {}.", uuid);
        match client
            .portfolio
            .get(&uuid, &PortfolioBreakdownQuery::new())
            .await
        {
            Ok(breakdown) => println!("{:#?}", breakdown),
            Err(error) => println!("Unable to get the breakdown: {}", error),
        }
    }
}
