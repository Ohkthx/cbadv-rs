//! # Coinbase Advanced Fee API
//!
//! `fee` gives access to the Fee API and the various endpoints associated with it.
//! Currently the only endpoint available is the Transaction Summary endpoint.

use crate::signer::Signer;
use crate::utils::{CBAdvError, Result};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::fmt;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct FeeTier {
    pub pricing_tier: String,
    #[serde_as(as = "DisplayFromStr")]
    pub usd_from: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub usd_to: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub taker_fee_rate: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub maker_fee_rate: f64,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct MarginRate {
    #[serde_as(as = "DisplayFromStr")]
    pub value: f64,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Tax {
    #[serde_as(as = "DisplayFromStr")]
    pub value: f64,
    pub r#type: String,
}

/// Represents the transaction summary for fees received from the API.
#[derive(Deserialize, Debug)]
pub struct TransactionSummary {
    pub total_volume: f64,
    pub total_fees: f64,
    pub fee_tier: FeeTier,
    pub margin_rate: Option<MarginRate>,
    pub goods_and_services_tax: Option<Tax>,
    pub advanced_trade_only_volume: f64,
    pub advanced_trade_only_fees: f64,
    pub coinbase_pro_volume: f64,
    pub coinbase_pro_fees: f64,
}

/// Represents parameters that are optional for transaction summary API request.
#[derive(Serialize, Default, Debug)]
pub struct TransactionSummaryQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    /// String of the users native currency, default is USD.
    pub user_native_currency: Option<String>,
    /// Type of products to return. Valid options: SPOT or FUTURE
    pub product_type: Option<String>,
}

impl fmt::Display for TransactionSummaryQuery {
    /// Converts the object into HTTP request parameters.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut query: String = "".to_string();

        query = match &self.start_date {
            Some(v) => format!("{}&start_date={}", query, v),
            _ => query,
        };

        query = match &self.end_date {
            Some(v) => format!("{}&end_date={}", query, v),
            _ => query,
        };

        query = match &self.user_native_currency {
            Some(v) => format!("{}&user_native_currency={}", query, v),
            _ => query,
        };

        query = match &self.product_type {
            Some(v) => format!("{}&product_type={}", query, v),
            _ => query,
        };

        match query.is_empty() {
            true => write!(f, ""),
            false => write!(f, "{}", query[1..].to_string()),
        }
    }
}

/// Provides access to the Fee API for the service.
pub struct FeeAPI {
    signer: Signer,
}

impl FeeAPI {
    /// Resource for the API.
    const RESOURCE: &str = "/api/v3/brokerage/transaction_summary";

    /// Creates a new instance of the Fee API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub(crate) fn new(signer: Signer) -> Self {
        Self { signer }
    }

    /// Obtains fee transaction summary from the API.
    ///
    /// # Arguments
    ///
    /// * `query` - Optional paramaters used to modify the resulting scope of the
    /// summary.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/transaction_summary
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gettransactionsummary>
    pub async fn get(&self, query: &TransactionSummaryQuery) -> Result<TransactionSummary> {
        match self.signer.get(Self::RESOURCE, &query.to_string()).await {
            Ok(value) => match value.json::<TransactionSummary>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse("fee summary object".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
