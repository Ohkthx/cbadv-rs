//! # Coinbase Advanced Convert API
//!
//! `convert` gives access to the Convert API and the various endpoints associated with it.
//! This allows for the conversion between two currencies.

use crate::constants::convert::{QUOTE_ENDPOINT, RESOURCE_ENDPOINT};
use crate::convert::{
    ConvertQuery, ConvertQuoteQuery, ConvertResponse, Trade, TradeIncentiveMetadata,
};
use crate::errors::CbAdvError;
use crate::signer::Signer;
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Convert API for the service.
pub struct ConvertApi {
    /// Object used to sign requests made to the API.
    signer: Signer,
}

impl ConvertApi {
    /// Creates a new instance of the Convert API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub(crate) fn new(signer: Signer) -> Self {
        Self { signer }
    }

    /// Create a convert quote with a specified source currency, target currency, and amount.
    ///
    /// Supported conversions are USD to USDC and USDC to USD - both with 0 fees.
    /// Use the trade_id produced from this request to commit the trade.
    ///
    /// Trades are valid for 10 minutes after the quote is created.
    ///
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_createconvertquote
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

        match self.signer.post(QUOTE_ENDPOINT, &NoQuery, &query).await {
            Ok(value) => match value.json::<ConvertResponse>().await {
                Ok(resp) => Ok(resp.trade),
                Err(_) => Err(CbAdvError::BadParse(
                    "convert quote response object".to_string(),
                )),
            },
            Err(error) => Err(error),
        }
    }

    /// Gets a list of information about a convert trade with a specified trade ID, source currency, and target currency.
    ///
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getconverttrade
    pub async fn get(
        &mut self,
        trade_id: &str,
        from_account: &str,
        to_account: &str,
    ) -> CbResult<Trade> {
        let resource = format!("{}/trade/{}", RESOURCE_ENDPOINT, trade_id);
        let query = ConvertQuery {
            from_account: from_account.to_string(),
            to_account: to_account.to_string(),
        };

        match self.signer.get(&resource, &query).await {
            Ok(value) => match value.json::<ConvertResponse>().await {
                Ok(resp) => Ok(resp.trade),
                Err(_) => Err(CbAdvError::BadParse(
                    "get convert response object".to_string(),
                )),
            },
            Err(error) => Err(error),
        }
    }

    /// Commits a convert trade with a specified trade ID, source currency, and target currency.
    ///
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_commitconverttrade
    pub async fn commit(
        &mut self,
        trade_id: &str,
        from_account: &str,
        to_account: &str,
    ) -> CbResult<Trade> {
        let resource = format!("{}/trade/{}", RESOURCE_ENDPOINT, trade_id);
        let query = ConvertQuery {
            from_account: from_account.to_string(),
            to_account: to_account.to_string(),
        };

        match self.signer.post(&resource, &NoQuery, &query).await {
            Ok(value) => match value.json::<ConvertResponse>().await {
                Ok(resp) => Ok(resp.trade),
                Err(_) => Err(CbAdvError::BadParse(
                    "convert commit response object".to_string(),
                )),
            },
            Err(error) => Err(error),
        }
    }
}
