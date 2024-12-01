//! # Coinbase Advanced Account API
//!
//! `account` gives access to the Account API and the various endpoints associated with it.
//! This allows you to obtain account information either by account UUID or in bulk (all accounts).

use crate::account::{Account, AccountListQuery, AccountWrapper, PaginatedAccounts};
use crate::constants::accounts::{LIST_ACCOUNT_MAXIMUM, RESOURCE_ENDPOINT};
use crate::errors::CbError;
use crate::http_agent::SecureHttpAgent;
use crate::traits::{HttpAgent, NoQuery};
use crate::types::CbResult;

/// Provides access to the Account API for the service.
pub struct AccountApi {
    /// Object used to sign requests made to the API.
    agent: Option<SecureHttpAgent>,
}

impl AccountApi {
    /// Creates a new instance of the Account API. This grants access to account information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: Option<SecureHttpAgent>) -> Self {
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
        let agent = get_auth!(self.agent, "get account");
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, account_uuid);
        let response = agent.get(&resource, &NoQuery).await?;
        let data: AccountWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
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
    /// * `query` - Parameters to control the query, such as limit.
    pub async fn get_by_id(&mut self, id: &str, query: &AccountListQuery) -> CbResult<Account> {
        is_auth!(self.agent, "get account by ID");

        let mut query = query.clone().limit(LIST_ACCOUNT_MAXIMUM);

        loop {
            // Fetch accounts with the current query, propagating any errors.
            let mut listed = self.get_bulk(&query).await?;

            // Check if the desired account is in the current batch.
            if let Some(index) = listed.accounts.iter().position(|r| r.currency == id) {
                return Ok(listed.accounts.swap_remove(index));
            }

            // If no more pages to fetch, return a "not found" error with context.
            if !listed.has_next {
                return Err(CbError::NotFound(format!(
                    "No account found with ID '{}'.",
                    id
                )));
            }

            // Update the cursor for the next API call.
            query.cursor = Some(listed.cursor);
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
    /// * `query` - Parameters to control the query, such as limit.
    pub async fn get_all(&mut self, query: &AccountListQuery) -> CbResult<Vec<Account>> {
        is_auth!(self.agent, "get all accounts");

        let mut query = query.clone().limit(LIST_ACCOUNT_MAXIMUM);
        let mut all_accounts = Vec::new();

        loop {
            // Fetch accounts with the current query, propagating any errors.
            let mut listed = self.get_bulk(&query).await?;

            // Append fetched accounts to the result list.
            all_accounts.append(&mut listed.accounts);

            // Check if there's more data to fetch.
            if listed.has_next {
                // Update the cursor for the next request.
                query.cursor = Some(listed.cursor);
            } else {
                // No more data to fetch.
                break;
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
    pub async fn get_bulk(&mut self, query: &AccountListQuery) -> CbResult<PaginatedAccounts> {
        let agent = get_auth!(self.agent, "get bulk accounts");
        let response = agent.get(RESOURCE_ENDPOINT, query).await?;
        let data: PaginatedAccounts = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
    }
}
