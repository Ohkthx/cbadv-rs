//! # Coinbase Advanced Account API
//!
//! `account` gives access to the Account API and the various endpoints associated with it.
//! This allows you to obtain account information either by account UUID or in bulk (all accounts).

use serde::{Deserialize, Serialize};

use crate::traits::Query;
use crate::utils::{deserialize_numeric, QueryBuilder};

/// Represents a Balance for either Available or Held funds.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
    /// Value for the currency available or held.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub value: f64,
    /// Denomination of the currency.
    pub currency: String,
}

/// Represents an Account received from the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    /// Unique identifier for account.
    pub uuid: String,
    /// Name for the account.
    pub name: String,
    /// Currency symbol for the account.
    pub currency: String,
    /// Current available balance for the account.
    pub available_balance: Balance,
    /// Whether or not this account is the user's primary account.
    pub default: bool,
    /// Whether or not this account is active and okay to use.
    pub active: bool,
    /// Time at which this account was created.
    pub created_at: String,
    /// Time at which this account was updated.
    pub updated_at: String,
    /// Time at which this account was deleted.
    pub deleted_at: Option<String>,
    /// Possible values: [ACCOUNT_TYPE_UNSPECIFIED, ACCOUNT_TYPE_CRYPTO, ACCOUNT_TYPE_FIAT, ACCOUNT_TYPE_VAULT]
    pub r#type: String,
    /// Whether or not this account is ready to trade.
    pub ready: bool,
    /// Current balance on hold.
    pub hold: Balance,
}

/// Represents a list of accounts received from the API.
#[derive(Deserialize, Debug)]
pub struct ListedAccounts {
    /// Accounts returned from the API.
    pub accounts: Vec<Account>,
    /// Whether there are additional pages for this query.
    pub has_next: bool,
    /// Cursor for paginating. Users can use this string to pass in the next call to this endpoint, and repeat this process to fetch all accounts through pagination.
    pub cursor: String,
    /// Number of accounts returned.
    pub size: u32,
}

/// Represents an account response from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct AccountResponse {
    /// Account returned from the API.
    pub(crate) account: Account,
}

/// Represents parameters that are optional for List Account API request.
#[derive(Serialize, Default, Debug, Clone)]
pub struct ListAccountsQuery {
    /// Amount to obtain, default 49 maximum is 250.
    pub limit: Option<u32>,
    /// Returns accounts after the cursor provided.
    pub cursor: Option<String>,
}

impl Query for ListAccountsQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_u32_optional("limit", self.limit)
            .push_optional("cursor", &self.cursor)
            .build()
    }
}
