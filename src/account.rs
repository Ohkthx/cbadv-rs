//! # Coinbase Advanced Account API
//!
//! `account` gives access to the Account API and the various endpoints associated with it.
//! This allows you to obtain account information either by account UUID or in bulk (all accounts).

use crate::signer::Signer;
use crate::utils::{CBAdvError, Result};
use async_recursion::async_recursion;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::fmt;

/// Represents a Balance for either Available or Held funds.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    #[serde_as(as = "DisplayFromStr")]
    pub value: f64,
    pub currency: String,
}

/// Represents an Account received from the API.
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

/// Represents a list of accounts received from the API.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListedAccounts {
    pub accounts: Vec<Account>,
    pub has_next: bool,
    pub cursor: String,
    pub size: u32,
}

/// Represents an account response from the API.
#[derive(Serialize, Deserialize, Debug)]
struct AccountResponse {
    pub account: Account,
}

/// Represents parameters that are optional for List Account API request.
#[derive(Serialize, Default, Debug)]
pub struct ListAccountsQuery {
    /// Amount to obtain, default 49 maximum is 250.
    pub limit: Option<u32>,
    /// Returns accounts after the cursor provided.
    pub cursor: Option<String>,
}

impl fmt::Display for ListAccountsQuery {
    /// Converts the object into HTTP request parameters.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut query: String = "".to_string();

        query = match &self.limit {
            Some(v) => format!("{}&limit={}", query, v),
            _ => query,
        };

        query = match &self.cursor {
            Some(v) => format!("{}&cursor={}", query, v),
            _ => query,
        };

        match query.is_empty() {
            true => write!(f, ""),
            false => write!(f, "{}", query[1..].to_string()),
        }
    }
}

/// Provides access to the Account API for the service.
pub struct AccountAPI {
    signer: Signer,
}

impl AccountAPI {
    /// Resource for the API.
    const RESOURCE: &str = "/api/v3/brokerage/accounts";

    /// Creates a new instance of the Account API. This grants access to account information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub fn new(signer: Signer) -> Self {
        Self { signer }
    }

    /// Obtains a single account based on the Account UUID (ex. "XXXX-YYYY-ZZZZ"). This is the most
    /// efficient way to get a single account, however it requires the user to know the UUID.
    ///
    /// # Arguments
    ///
    /// * `account_uuid` - A string the represents the account's UUID.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/accounts/{account_uuid}
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getaccount>
    pub async fn get(&self, account_uuid: &str) -> Result<Account> {
        let resource = format!("{}/{}", Self::RESOURCE, account_uuid);
        match self.signer.get(&resource, "").await {
            Ok(value) => match value.json::<AccountResponse>().await {
                Ok(resp) => Ok(resp.account),
                Err(_) => Err(CBAdvError::BadParse("account object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains a single account based on the Account ID (ex. "BTC").
    /// This wraps `get_bulk` and recursively makes several additional requests until either the
    /// account is found or there are not more accounts. This is a more expensive call, but more
    /// convient than `get` which requires knowing the UUID already.
    ///
    /// NOTE: NOT A STANDARD API FUNCTION. QoL function that may require additional API requests than
    /// normal.
    ///
    /// # Arguments
    ///
    /// * `id` - Identifier for the account, such as BTC or ETH.
    /// * `query` - Optional parameters, should default to None unless you want additional control.
    #[async_recursion]
    pub async fn get_by_id(&self, id: &str, query: Option<ListAccountsQuery>) -> Result<Account> {
        let mut query = match query {
            Some(p) => p,
            None => ListAccountsQuery::default(),
        };

        match self.get_bulk(&query).await {
            Ok(mut listed) => {
                // Find the index.
                match listed.accounts.iter().position(|r| r.currency == id) {
                    Some(index) => Ok(listed.accounts.swap_remove(index)),
                    None => {
                        // Prevent further requests if no more can be made.
                        if !listed.has_next {
                            return Err(CBAdvError::NotFound("no matching ids".to_string()));
                        }

                        // Make another request to the API for the account.
                        query.cursor = Some(listed.cursor);
                        self.get_by_id(id, Some(query)).await
                    }
                }
            }
            Err(error) => Err(error),
        }
    }

    /// Obtains various accounts from the API.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/accounts
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getaccounts>
    pub async fn get_bulk(&self, query: &ListAccountsQuery) -> Result<ListedAccounts> {
        match self.signer.get(Self::RESOURCE, &query.to_string()).await {
            Ok(value) => match value.json::<ListedAccounts>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse("accounts vector".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
