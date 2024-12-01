//! # Coinbase Advanced Data API
//!
//! `data` gives access to the Data API and the various endpoints associated with it.

use crate::constants::data::KEY_PERMISSIONS_ENDPOINT;
use crate::errors::CbError;
use crate::http_agent::SecureHttpAgent;
use crate::models::data::KeyPermissions;
use crate::traits::{HttpAgent, NoQuery};
use crate::types::CbResult;

/// Provides access to the Data API for the service.
pub struct DataApi {
    /// Object used to sign requests made to the API.
    agent: Option<SecureHttpAgent>,
}

impl DataApi {
    /// Creates a new instance of the Data API. This grants access to various data information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: Option<SecureHttpAgent>) -> Self {
        Self { agent }
    }

    /// Get information about your CDP API key permissions.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/key_permissions
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_getapikeypermissions>
    pub async fn key_permissions(&mut self) -> CbResult<KeyPermissions> {
        let agent = get_auth!(self.agent, "get key permissions");
        let response = agent.get(KEY_PERMISSIONS_ENDPOINT, &NoQuery).await?;
        let data: KeyPermissions = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
    }
}