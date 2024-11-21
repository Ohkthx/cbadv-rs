//! # Coinbase Advanced Fee API
//!
//! `fee` gives access to the Fee API and the various endpoints associated with it.
//! Currently the only endpoint available is the Transaction Summary endpoint.

use crate::constants::fees::RESOURCE_ENDPOINT;
use crate::errors::CbAdvError;
use crate::fee::{TransactionSummary, TransactionSummaryQuery};
use crate::http_agent::{HttpAgent, SecureHttpAgent};
use crate::types::CbResult;

/// Provides access to the Fee API for the service.
pub struct FeeApi {
    /// Object used to sign requests made to the API.
    agent: SecureHttpAgent,
}

impl FeeApi {
    /// Creates a new instance of the Fee API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: SecureHttpAgent) -> Self {
        Self { agent }
    }

    /// Obtains fee transaction summary from the API.
    ///
    /// # Arguments
    ///
    /// * `query` - Optional paramaters used to modify the resulting scope of the summary.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/transaction_summary
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gettransactionsummary>
    pub async fn get(&mut self, query: &TransactionSummaryQuery) -> CbResult<TransactionSummary> {
        let response = self.agent.get(RESOURCE_ENDPOINT, query).await?;
        let data: TransactionSummary = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data)
    }
}
