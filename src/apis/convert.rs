//! # Coinbase Advanced Convert API
//!
//! `convert` gives access to the Convert API and the various endpoints associated with it.
//! This allows for the conversion between two currencies.

use crate::constants::convert::{QUOTE_ENDPOINT, TRADE_ENDPOINT};
use crate::convert::{
    ConvertQuery, ConvertQuoteQuery, Trade, TradeIncentiveMetadata, TradeWrapper,
};
use crate::errors::CbAdvError;
use crate::http_agent::{HttpAgent, SecureHttpAgent};
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Convert API for the service.
pub struct ConvertApi {
    /// Object used to sign requests made to the API.
    agent: SecureHttpAgent,
}

impl ConvertApi {
    /// Creates a new instance of the Convert API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: SecureHttpAgent) -> Self {
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
    /// * `from_account` - The source currency to convert from.
    /// * `to_account` - The target currency to convert to.
    /// * `amount` - The amount to convert.
    /// * `metadata` - Optional metadata for the trade.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/convert/quote
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_createconvertquote>
    pub async fn create_quote(
        &mut self,
        from_account: &str,
        to_account: &str,
        amount: f64,
        metadata: Option<TradeIncentiveMetadata>,
    ) -> CbResult<Trade> {
        let query = ConvertQuoteQuery {
            from_account: from_account.to_string(),
            to_account: to_account.to_string(),
            amount: amount.to_string(),
            trade_incentive_metadata: metadata,
        };

        let response = self.agent.post(QUOTE_ENDPOINT, &NoQuery, &query).await?;
        let data = response
            .json::<TradeWrapper>()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.trade)
    }

    /// Gets a list of information about a convert trade with a specified trade ID, source currency, and target currency.
    ///
    /// # Arguments
    ///
    /// * `trade_id` - The trade ID to get information about.
    /// * `from_account` - The source currency to convert from.
    /// * `to_account` - The target currency to convert to.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/convert/trade
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getconverttrade>
    pub async fn get(
        &mut self,
        trade_id: &str,
        from_account: &str,
        to_account: &str,
    ) -> CbResult<Trade> {
        let query = ConvertQuery {
            from_account: from_account.to_string(),
            to_account: to_account.to_string(),
        };

        let resource = format!("{}/{}", TRADE_ENDPOINT, trade_id);
        let response = self.agent.get(&resource, &query).await?;
        let data: TradeWrapper = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.trade)
    }

    /// Commits a convert trade with a specified trade ID, source currency, and target currency.
    ///
    /// # Arguments
    ///
    /// * `trade_id` - The trade ID to get information about.
    /// * `from_account` - The source currency to convert from.
    /// * `to_account` - The target currency to convert to.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/convert/trade
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_commitconverttrade>
    pub async fn commit(
        &mut self,
        trade_id: &str,
        from_account: &str,
        to_account: &str,
    ) -> CbResult<Trade> {
        let query = ConvertQuery {
            from_account: from_account.to_string(),
            to_account: to_account.to_string(),
        };

        let resource = format!("{}/{}", TRADE_ENDPOINT, trade_id);
        let response = self.agent.post(&resource, &NoQuery, &query).await?;
        let data: TradeWrapper = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.trade)
    }
}
