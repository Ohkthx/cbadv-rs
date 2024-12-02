//! # Coinbase Advanced Convert API
//!
//! `convert` gives access to the Convert API and the various endpoints associated with it.
//! This allows for the conversion between two currencies.

use crate::constants::convert::{QUOTE_ENDPOINT, TRADE_ENDPOINT};
use crate::errors::CbError;
use crate::http_agent::SecureHttpAgent;
use crate::models::convert::{ConvertQuery, ConvertQuoteRequest, Trade, TradeWrapper};
use crate::traits::{HttpAgent, NoQuery};
use crate::types::CbResult;

/// Provides access to the Convert API for the service.
pub struct ConvertApi {
    /// Object used to sign requests made to the API.
    agent: Option<SecureHttpAgent>,
}

impl ConvertApi {
    /// Creates a new instance of the Convert API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: Option<SecureHttpAgent>) -> Self {
        Self { agent }
    }

    /// Create a convert quote with a specified source currency, target currency, and amount.
    ///
    /// Supported conversions are USD to USDC and USDC to USD - both with 0 fees.
    /// Use the trade_id produced from this request to commit the trade.
    ///
    /// Trades are valid for 10 minutes after the quote is created.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to create a quote.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/convert/quote
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_createconvertquote>
    pub async fn create_quote(&mut self, request: &ConvertQuoteRequest) -> CbResult<Trade> {
        let agent = get_auth!(self.agent, "create convert quote");
        let response = agent.post(QUOTE_ENDPOINT, &NoQuery, request).await?;
        let data = response
            .json::<TradeWrapper>()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
    }

    /// Gets a list of information about a convert trade with a specified trade ID, source currency, and target currency.
    ///
    /// # Arguments
    ///
    /// * `trade_id` - The trade ID to get information about.
    /// * `query` - The query to obtain the trade.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/convert/trade
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getconverttrade>
    pub async fn get(&mut self, trade_id: &str, query: &ConvertQuery) -> CbResult<Trade> {
        let agent = get_auth!(self.agent, "get convert trade");
        let resource = format!("{}/{}", TRADE_ENDPOINT, trade_id);
        let response = agent.get(&resource, query).await?;
        let data: TradeWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
    }

    /// Commits a convert trade with a specified trade ID, source currency, and target currency.
    ///
    /// # Arguments
    ///
    /// * `trade_id` - The trade ID to get information about.
    /// * `query` - The query to commit the trade.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/convert/trade
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_commitconverttrade>
    pub async fn commit(&mut self, trade_id: &str, query: &ConvertQuery) -> CbResult<Trade> {
        let agent = get_auth!(self.agent, "commit convert quote");
        let resource = format!("{}/{}", TRADE_ENDPOINT, trade_id);
        let response = agent.post(&resource, &NoQuery, query).await?;
        let data: TradeWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
    }
}
