//! # Coinbase Advanced Account API
//!
//! `account` gives access to the Account API and the various endpoints associated with it.
//! This allows you to obtain account information either by account UUID or in bulk (all accounts).

use serde::{Deserialize, Serialize};

use crate::constants::accounts::LIST_ACCOUNT_MAXIMUM;
use crate::errors::CbError;
use crate::traits::Query;
use crate::types::CbResult;
use crate::utils::QueryBuilder;

use super::shared::Balance;

/// Platform that the account is associated with.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Platform {
    /// Spot account.
    #[serde(rename = "ACCOUNT_PLATFORM_CONSUMER")]
    Consumer,
    /// US Derivatives account.
    #[serde(rename = "ACCOUNT_PLATFORM_CFM_CONSUMER")]
    CfmConsumer,
    /// International Exchange account.
    #[serde(rename = "ACCOUNT_PLATFORM_INTX")]
    Intx,
}

/// Possible values for the account type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AccountType {
    #[serde(rename = "ACCOUNT_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "ACCOUNT_TYPE_CRYPTO")]
    Crypto,
    #[serde(rename = "ACCOUNT_TYPE_FIAT")]
    Fiat,
    #[serde(rename = "ACCOUNT_TYPE_VAULT")]
    Vault,
    #[serde(rename = "ACCOUNT_TYPE_PERP_FUTURES")]
    PerpFutures,
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
    /// Possible values: [`ACCOUNT_TYPE_UNSPECIFIED`, `ACCOUNT_TYPE_CRYPTO`, `ACCOUNT_TYPE_FIAT`, `ACCOUNT_TYPE_VAULT`]
    pub r#type: AccountType,
    /// Whether or not this account is ready to trade.
    pub ready: bool,
    /// Current balance on hold.
    pub hold: Balance,
    /// Platform that the account is associated with.
    pub platform: Platform,
}

/// Response from the API that wraps a list of accounts.
#[derive(Deserialize, Debug)]
pub struct PaginatedAccounts {
    /// Accounts returned from the API.
    pub accounts: Vec<Account>,
    /// Whether there are additional pages for this query.
    pub has_next: bool,
    /// Cursor for paginating. Users can use this string to pass in the next call to this endpoint, and repeat this process to fetch all accounts through pagination.
    pub cursor: String,
    /// Number of accounts returned.
    pub size: u32,
}

/// Represents parameters that are optional for List Account API request.
#[derive(Serialize, Debug, Clone)]
pub struct AccountListQuery {
    /// Amount to obtain, default 49 maximum is 250.
    pub limit: u32,
    /// Returns accounts after the cursor provided.
    pub cursor: Option<String>,
}

impl Query for AccountListQuery {
    fn check(&self) -> CbResult<()> {
        if self.limit == 0 || self.limit > LIST_ACCOUNT_MAXIMUM {
            return Err(CbError::BadQuery(format!(
                "Limit must be greater than 0 with a maximum of {LIST_ACCOUNT_MAXIMUM}"
            )));
        }
        Ok(())
    }

    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push("limit", self.limit)
            .push_optional("cursor", &self.cursor)
            .build()
    }
}

impl Default for AccountListQuery {
    fn default() -> Self {
        Self {
            limit: 49,
            cursor: None,
        }
    }
}

impl AccountListQuery {
    /// Creates a new `AccountListQuery` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the limit for the query. Default is 49 and maximum is 250.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// Sets the cursor for the query.
    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

/// Response from the API that wraps a single account.
#[derive(Deserialize, Debug)]
pub(crate) struct AccountWrapper {
    /// Account returned from the API.
    pub(crate) account: Account,
}

impl From<AccountWrapper> for Account {
    fn from(wrapper: AccountWrapper) -> Self {
        wrapper.account
    }
}
