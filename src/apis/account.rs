//! # Coinbase Advanced Account API
//!
//! `account` gives access to the Account API and the various endpoints associated with it.
//! This allows you to obtain account information either by account UUID or in bulk (all accounts).

use async_recursion::async_recursion;

use crate::account::{Account, AccountResponse, ListAccountsQuery, ListedAccounts};
use crate::constants::accounts::RESOURCE_ENDPOINT;
use crate::errors::CbAdvError;
use crate::signer::Signer;
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Account API for the service.
pub struct AccountApi {
    /// Object used to sign requests made to the API.
    signer: Signer,
}

impl AccountApi {
    /// Creates a new instance of the Account API. This grants access to account information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub(crate) fn new(signer: Signer) -> Self {
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
    pub async fn get(&mut self, account_uuid: &str) -> CbResult<Account> {
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, account_uuid);
        match self.signer.get(&resource, &NoQuery).await {
            Ok(value) => match value.json::<AccountResponse>().await {
                Ok(resp) => Ok(resp.account),
                Err(_) => Err(CbAdvError::BadParse("account object".to_string())),
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
    pub async fn get_by_id(
        &mut self,
        id: &str,
        query: Option<ListAccountsQuery>,
    ) -> CbResult<Account> {
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
                            return Err(CbAdvError::NotFound("no matching ids".to_string()));
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

    /// Obtains all accounts available to the API Key. Use a larger limit in the query to decrease
    /// the amount of API calls. Recursively makes calls to obtain all accounts.
    ///
    /// NOTE: NOT A STANDARD API FUNCTION. QoL function that may require additional API requests than
    /// normal.
    ///
    /// # Arguments
    ///
    /// * `query` - Optional parameters, should default to None unless you want additional control.
    #[async_recursion]
    pub async fn get_all(&mut self, query: Option<ListAccountsQuery>) -> CbResult<Vec<Account>> {
        let mut query = match query {
            Some(p) => p,
            None => ListAccountsQuery::default(),
        };

        // Obtain until there are not anymore accounts.
        match self.get_bulk(&query).await {
            Ok(mut listed) => {
                if listed.has_next {
                    query.cursor = Some(listed.cursor);
                    match self.get_all(Some(query)).await {
                        Ok(mut accounts) => listed.accounts.append(&mut accounts),
                        Err(error) => return Err(error),
                    }
                }
                Ok(listed.accounts)
            }
            Err(error) => return Err(error),
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
    pub async fn get_bulk(&mut self, query: &ListAccountsQuery) -> CbResult<ListedAccounts> {
        match self.signer.get(RESOURCE_ENDPOINT, query).await {
            Ok(value) => match value.json::<ListedAccounts>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CbAdvError::BadParse("accounts vector".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
