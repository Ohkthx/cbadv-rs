use crate::cbadv::utils::Signer;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Represents a Balance for either Available or Held funds.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub value: String,
    pub currency: String,
}

/// Represents an Account received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub uuid: String,
    pub name: String,
    pub currency: String,
    pub available_balance: Balance,
    pub default: bool,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub r#type: String,
    pub ready: bool,
    pub hold: Balance,
}

/// Represents an account response from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct AccountResponse {
    pub account: Account,
}

/// Represents a list of accounts received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct ListAccounts {
    pub accounts: Vec<Account>,
    pub has_next: bool,
    pub cursor: String,
    pub size: i32,
}

/// Represents parameters that are optional for List Account API request.
#[allow(dead_code)]
#[derive(Serialize, Default, Debug)]
pub struct ListAccountsParams {
    pub limit: i32,
    pub cursor: String,
}

impl ListAccountsParams {
    pub fn to_params(&self) -> String {
        // format!("limit={}&cursor={}", self.limit, self.cursor)
        format!("limit={}&cursor={}", self.limit, self.cursor)
    }
}

/// Provides access to the Account API for the service.
pub struct AccountAPI {
    signer: Signer,
}

impl AccountAPI {
    /// Resource for the API.
    const RESOURCE: &str = "/api/v3/brokerage/accounts";

    /// Creates a new instance of the Account API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub fn new(signer: Signer) -> Self {
        Self { signer }
    }

    /// Obtains a single account based on the Account UUID (ex. "XXXX-YYYY-ZZZZ").
    ///
    /// # Arguments
    ///
    /// * `account_uuid` - A string the represents the account's UUID.
    ///
    /// # Endpoint / Reference
    ///
    /// https://api.coinbase.com/api/v3/brokerage/accounts/{account_uuid}
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getaccount
    pub async fn get(&self, account_uuid: String) -> Result<Account> {
        let resource = format!("{}/{}", Self::RESOURCE.to_string(), account_uuid);
        match self.signer.get(resource, "".to_string()).await {
            Ok(value) => match value.json::<AccountResponse>().await {
                Ok(resp) => Ok(resp.account),
                Err(error) => Err(Box::new(error)),
            },
            Err(error) => {
                println!("Failed to get account: {}", error);
                Err(error)
            }
        }
    }

    /// Obtains various accounts from the API.
    ///
    /// # Endpoint / Reference
    ///
    /// https://api.coinbase.com/api/v3/brokerage/accounts
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getaccounts
    pub async fn get_all(&self, params: ListAccountsParams) -> Result<Vec<Account>> {
        let resource = Self::RESOURCE.to_string();
        match self.signer.get(resource, params.to_params()).await {
            Ok(value) => match value.json::<ListAccounts>().await {
                Ok(resp) => Ok(resp.accounts),
                Err(error) => Err(Box::new(error)),
            },
            Err(error) => {
                println!("Failed to get all accounts: {}", error);
                Err(error)
            }
        }
    }
}
