//! # Coinbase Advanced Account API
//!
//! `account` gives access to the Account API and the various endpoints associated with it.
//! This allows you to obtain account information either by account UUID or in bulk (all accounts).

use crate::account::{Account, AccountResponse, ListAccountsQuery, ListedAccounts};
use crate::constants::accounts::RESOURCE_ENDPOINT;
use crate::errors::CbAdvError;
use crate::http_agent::{HttpAgent, SecureHttpAgent};
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Account API for the service.
pub struct AccountApi {
    /// Object used to sign requests made to the API.
    agent: SecureHttpAgent,
}

impl AccountApi {
    /// Creates a new instance of the Account API. This grants access to account information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: SecureHttpAgent) -> Self {
        Self { agent }
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
        match self.agent.get(&resource, &NoQuery).await {
            Ok(value) => match value.json::<AccountResponse>().await {
                Ok(resp) => Ok(resp.account),
                Err(_) => Err(CbAdvError::BadParse("account object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains a single account based on the Account ID (ex. "BTC").
    /// This wraps `get_bulk` and iteratively makes several additional requests until either the
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
    pub async fn get_by_id(
        &mut self,
        id: &str,
        query: Option<ListAccountsQuery>,
    ) -> CbResult<Account> {
        let mut query = query.unwrap_or_default();

        loop {
            match self.get_bulk(&query).await {
                Ok(mut listed) => {
                    // Check if the desired account is in the current batch
                    if let Some(index) = listed.accounts.iter().position(|r| r.currency == id) {
                        return Ok(listed.accounts.swap_remove(index));
                    }

                    // If no more pages to fetch, return a "not found" error
                    if !listed.has_next {
                        return Err(CbAdvError::NotFound("no matching ids".to_string()));
                    }

                    // Update the cursor for the next API call
                    query.cursor = Some(listed.cursor);
                }
                Err(error) => {
                    // Return an error if the API call fails
                    return Err(error);
                }
            }
        }
    }

    /// Obtains all accounts available to the API Key. Use a larger limit in the query to decrease
    /// the amount of API calls. Iteratively makes calls to obtain all accounts.
    ///
    /// NOTE: NOT A STANDARD API FUNCTION. QoL function that may require additional API requests than
    /// normal.
    ///
    /// # Arguments
    ///
    /// * `query` - Optional parameters, should default to None unless you want additional control.
    pub async fn get_all(&mut self, query: Option<ListAccountsQuery>) -> CbResult<Vec<Account>> {
        let mut query = query.unwrap_or_default();
        let mut all_accounts = Vec::new();

        loop {
            // Fetch accounts with the current query
            match self.get_bulk(&query).await {
                Ok(mut listed) => {
                    // Append fetched accounts to the result list
                    all_accounts.append(&mut listed.accounts);

                    // Check if there's more data to fetch
                    if listed.has_next {
                        // Update the cursor for the next request
                        query.cursor = Some(listed.cursor);
                    } else {
                        // No more data to fetch
                        break;
                    }
                }
                Err(error) => {
                    // Return an error if the fetch fails
                    return Err(error);
                }
            }
        }

        Ok(all_accounts)
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
        match self.agent.get(RESOURCE_ENDPOINT, query).await {
            Ok(value) => match value.json::<ListedAccounts>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CbAdvError::BadParse("accounts vector".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
