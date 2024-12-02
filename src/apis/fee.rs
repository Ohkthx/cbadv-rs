//! # Coinbase Advanced Fee API
//!
//! `fee` gives access to the Fee API and the various endpoints associated with it.
//! Currently the only endpoint available is the Transaction Summary endpoint.

use crate::constants::fees::RESOURCE_ENDPOINT;
use crate::errors::CbError;
use crate::http_agent::SecureHttpAgent;
use crate::models::fee::{FeeTransactionSummaryQuery, TransactionSummary};
use crate::traits::HttpAgent;
use crate::types::CbResult;

/// Provides access to the Fee API for the service.
pub struct FeeApi {
    /// Object used to sign requests made to the API.
    agent: Option<SecureHttpAgent>,
}

impl FeeApi {
    /// Creates a new instance of the Fee API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: Option<SecureHttpAgent>) -> Self {
        Self { agent }
    }

    /// Obtains fee transaction summary from the API.
    ///
    /// # Arguments
    ///
    /// * `query` - Paramaters used to modify the resulting scope of the summary.
    ///
    /// # Errors
    ///
    /// * `CbError::AuthenticationError` - If the agent is not authenticated.
    /// * `CbError::JsonError` - If there was an issue parsing the JSON response.
    /// * `CbError::RequestError` - If there was an issue making the request.
    /// * `CbError::UrlParseError` - If there was an issue parsing the URL.
    /// * `CbError::BadSerialization` - If there was an issue serializing the request.
    /// * `CbError::BadStatus` - If the status code was not 200.
    /// * `CbError::BadJwt` - If there was an issue creating the JWT.
    ///
    /// # Endpoint / Reference
    ///
    /// * <https://api.coinbase.com/api/v3/brokerage/transaction_summary>
    /// * <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gettransactionsummary>
    pub async fn get(
        &mut self,
        query: &FeeTransactionSummaryQuery,
    ) -> CbResult<TransactionSummary> {
        let agent = get_auth!(self.agent, "get fee transaction summary");
        let response = agent.get(RESOURCE_ENDPOINT, query).await?;
        let data: TransactionSummary = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
    }
}
