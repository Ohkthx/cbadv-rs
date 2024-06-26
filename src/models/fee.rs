//! # Coinbase Advanced Fee API
//!
//! `fee` gives access to the Fee API and the various endpoints associated with it.
//! Currently the only endpoint available is the Transaction Summary endpoint.

use serde::{Deserialize, Serialize};

use crate::traits::Query;
use crate::utils::{deserialize_numeric, QueryBuilder};

/// Pricing tier for user, determined by notional (USD) volume.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeeTier {
    /// Current fee teir for the user.
    pub pricing_tier: String,
    /// Lower bound (inclusive) of pricing tier in notional volume.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub usd_from: u32,
    /// Upper bound (exclusive) of pricing tier in notional volume.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub usd_to: u32,
    /// Taker fee rate, applied if the order takes liquidity.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub taker_fee_rate: f64,
    /// Maker fee rate, applied if the order creates liquidity.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub maker_fee_rate: f64,
}

/// Represents a decimal number with precision.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarginRate {
    /// Value of the margin rate.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub value: f64,
}

/// Represents a tax amount.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tax {
    /// Amount of tax.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub value: f64,
    /// Type of tax. Possible values: [INCLUSIVE, EXCLUSIVE]
    pub r#type: String,
}

/// Represents the transaction summary for fees received from the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionSummary {
    /// Total volume across assets, denoted in USD.
    pub total_volume: f64,
    /// Total fees across assets, denoted in USD.
    pub total_fees: f64,
    /// Fee tier for the summary.
    pub fee_tier: FeeTier,
    /// Margin rate for the summary.
    pub margin_rate: Option<MarginRate>,
    /// Goods and Services Tax rate.
    pub goods_and_services_tax: Option<Tax>,
    /// Advanced Trade volume (non-inclusive of Pro) across assets, denoted in USD.
    pub advanced_trade_only_volume: f64,
    /// Advanced Trade fees (non-inclusive of Pro) across assets, denoted in USD.
    pub advanced_trade_only_fees: f64,
    /// Coinbase Pro volume across assets, denoted in USD.
    pub coinbase_pro_volume: f64,
    /// Coinbase Pro fees across assets, denoted in USD.
    pub coinbase_pro_fees: f64,
}

/// Represents parameters that are optional for transaction summary API request.
#[derive(Serialize, Default, Debug)]
pub struct TransactionSummaryQuery {
    /// Start date for the summary.
    pub start_date: Option<String>,
    /// End date for the summary.
    pub end_date: Option<String>,
    /// String of the users native currency, default is USD.
    pub user_native_currency: Option<String>,
    /// Type of products to return. Valid options: SPOT or FUTURE
    pub product_type: Option<String>,
}

impl Query for TransactionSummaryQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional("start_date", &self.start_date)
            .push_optional("end_date", &self.end_date)
            .push_optional("user_native_currency", &self.user_native_currency)
            .push_optional("product_type", &self.product_type)
            .build()
    }
}
