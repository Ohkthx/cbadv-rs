//! # Coinbase Advanced Fee API
//!
//! `fee` gives access to the Fee API and the various endpoints associated with it.
//! Currently the only endpoint available is the Transaction Summary endpoint.

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::errors::CbError;
use crate::traits::Query;
use crate::types::CbResult;
use crate::utils::QueryBuilder;

use super::product::ProductType;

/// Pricing tier for user, determined by notional (USD) volume.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeeTier {
    /// Current fee teir for the user.
    pub pricing_tier: String,
    /// Lower bound (inclusive) of pricing tier in notional volume.
    #[serde_as(as = "DisplayFromStr")]
    pub usd_from: u32,
    /// Upper bound (exclusive) of pricing tier in notional volume.
    #[serde_as(as = "DisplayFromStr")]
    pub usd_to: u32,
    /// Taker fee rate, applied if the order takes liquidity.
    #[serde_as(as = "DisplayFromStr")]
    pub taker_fee_rate: f64,
    /// Maker fee rate, applied if the order creates liquidity.
    #[serde_as(as = "DisplayFromStr")]
    pub maker_fee_rate: f64,
}

/// Represents a decimal number with precision.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarginRate {
    /// Value of the margin rate.
    #[serde_as(as = "DisplayFromStr")]
    pub value: f64,
}

/// Represents a tax amount.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tax {
    /// Amount of tax.
    #[serde_as(as = "DisplayFromStr")]
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
pub struct FeeTransactionSummaryQuery {
    /// Type of products to return. Valid options: SPOT or FUTURE
    pub product_type: Option<ProductType>,
}

impl Query for FeeTransactionSummaryQuery {
    fn check(&self) -> CbResult<()> {
        if let Some(product_type) = &self.product_type {
            if *product_type == ProductType::Unknown {
                return Err(CbError::BadQuery(
                    "product_type cannot be unknown".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional("product_type", &self.product_type)
            .build()
    }
}

impl FeeTransactionSummaryQuery {
    /// Creates a new instance of the query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the product type for the query.
    pub fn product_type(mut self, product_type: ProductType) -> Self {
        self.product_type = Some(product_type);
        self
    }
}
