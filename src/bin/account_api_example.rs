use cbadv::account::ListAccountsParams;
use cbadv::{client, config};

#[tokio::main]
async fn main() {
    let product_name: &str = "BTC";

    // Load the configuration file.
    let config = config::load("config.toml").unwrap();

    // Create a client to interact with the API.
    let client = client::new(config.cb_api_key, config.cb_api_secret);

    // Parameters to send to the API.
    let params = ListAccountsParams {
        limit: Some(50),
        ..Default::default()
    };

    // Pull all accounts.
    println!("Obtaining ALL Accounts.");
    let mut account_uuid = "".to_string();
    match client.account.get_all(params).await {
        Ok(accounts) => {
            println!("Accounts obtained: {:#?}", accounts.accounts.len());
            let index = accounts
                .accounts
                .iter()
                .position(|r| r.currency == product_name)
                .unwrap();

            match accounts.accounts.get(index) {
                Some(account) => {
                    account_uuid = account.uuid.clone();
                    println!("{:#?}", account);
                }
                None => println!("Out of bounds, could not find account."),
            }
        }
        Err(error) => println!("Unable to get all accounts: {}", error),
    }

    // Get a singular account based on the UUID.
    println!("\n\nObtaining Account: {}", account_uuid);
    match client.account.get(&account_uuid).await {
        Ok(account) => {
            println!("{:#?}", account);
        }
        Err(error) => {
            println!("Unable to get account: {}", error);
        }
    }
}
