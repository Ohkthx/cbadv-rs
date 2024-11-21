//! # Coinbase Advanced Portfolio API
//!
//! `portfolio` gives access to the Potfolio API and the various endpoints associated with it.
//! This allows for the management of individual portfolios.

use core::fmt;

use serde::{Deserialize, Serialize};

use super::shared::Balance;
use crate::{traits::Query, utils::QueryBuilder};

/// Portfolio type for a user's portfolio.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PortfolioType {
    /// Portfolio type for a user's default portfolio.
    Default,
    /// Portfolios created by the user.
    Consumer,
    Intx,
    /// Fallback for undefined or unrecognized values.
    Undefined,
}

impl AsRef<str> for PortfolioType {
    fn as_ref(&self) -> &str {
        match self {
            PortfolioType::Default => "DEFAULT",
            PortfolioType::Consumer => "CONSUMER",
            PortfolioType::Intx => "INTX",
            PortfolioType::Undefined => "UNDEFINED",
        }
    }
}

impl fmt::Display for PortfolioType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// Portfolio information.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Portfolio {
    /// Name of the portfolio.
    pub name: String,
    /// UUID of the portfolio.
    pub uuid: String,
    /// Type of the portfolio.
    pub r#type: PortfolioType,
    /// Indicates if the portfolio is deleted.
    pub deleted: bool,
}

/// Portfolio information returned from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct PortfoliosWrapper {
    pub portfolios: Vec<Portfolio>,
}

/// Response for creating or editing a  portfolio.
#[derive(Deserialize, Debug)]
pub(crate) struct PortfolioWrapper {
    /// Updated portfolio from the API.
    pub(crate) portfolio: Portfolio,
}

/// Create or Edit an existing portfolio.
#[derive(Serialize, Default, Debug)]
pub(crate) struct PortfolioQuery {
    /// New name of the portfolio.
    pub(crate) name: String,
}

/// Query parameters for listing portfolios.
#[derive(Serialize, Default, Debug)]
pub struct ListPortfoliosQuery {
    /// Type of portfolios to list.
    pub portfolio_type: Option<PortfolioType>,
}

impl Query for ListPortfoliosQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional("portfolio_type", &self.portfolio_type)
            .build()
    }
}

/// Parameters for moving funds between portfolios.
#[derive(Serialize, Debug)]
pub(crate) struct MoveFunds {
    /// Funds to move between portfolios.
    pub(crate) funds: Balance,
    /// Portfolio funds to be removed from.
    pub(crate) source_portfolio_uuid: String,
    /// Portfolio funds to be added to.
    pub(crate) target_portfolio_uuid: String,
}

/// Query parameters for a portfolio breakdown.
#[derive(Serialize, Default, Debug)]
pub(crate) struct PortfolioBreakdownQuery {
    /// Currency to use for the breakdown.
    pub(crate) currency: Option<String>,
}

impl Query for PortfolioBreakdownQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional("currency", &self.currency)
            .build()
    }
}

/// Enum for `PositionSide` values.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    FuturesPositionSideUnspecified,
    FuturesPositionSideLong,
    FuturesPositionSideShort,
}

/// Enum for `MarginType` values.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginType {
    MarginTypeUnspecified,
    MarginTypeCross,
    MarginTypeIsolated,
}

/// Portfolio balances for different categories.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PortfolioBalances {
    /// Total balance across all portfolio types.
    pub total_balance: Balance,
    /// Total balance in futures.
    pub total_futures_balance: Balance,
    /// Total balance in cash or cash-equivalent assets.
    pub total_cash_equivalent_balance: Balance,
    /// Total balance in cryptocurrencies.
    pub total_crypto_balance: Balance,
    /// Unrealized profit and loss in futures trading.
    pub futures_unrealized_pnl: Balance,
    /// Unrealized profit and loss in perpetual trading.
    pub perp_unrealized_pnl: Balance,
}

/// Spot position details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotPosition {
    /// The asset symbol (e.g., BTC, ETH).
    pub asset: String,
    /// The account UUID associated with the asset.
    pub account_uuid: String,
    /// Total balance of the asset in fiat currency.
    pub total_balance_fiat: f64,
    /// Total balance of the asset in cryptocurrency.
    pub total_balance_crypto: f64,
    /// Amount available for trading in fiat currency.
    pub available_to_trade_fiat: f64,
    /// Percentage of the portfolio allocated to this asset in decimal form.
    pub allocation: f64,
    /// Change in value of the asset over one day.
    /// NOTE: This field currently is not returned by the API.
    pub one_day_change: Option<f64>,
    /// Cost basis of the asset.
    pub cost_basis: Balance,
    /// URL of the asset's image.
    pub asset_img_url: String,
    /// Indicates if this position is cash or equivalent.
    pub is_cash: bool,
}

/// Represents monetary data with user and raw currency values.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MonetaryDetails {
    /// The monetary value in the user's native currency.
    #[serde(rename = "userNativeCurrency")]
    pub user_native_currency: Balance,
    /// The raw monetary value in the specified currency.
    #[serde(rename = "rawCurrency")]
    pub raw_currency: Balance,
}

/// Perpetual position details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerpPosition {
    /// The product ID associated with the perpetual position.
    pub product_id: String,
    /// The UUID of the product.
    pub product_uuid: String,
    /// The symbol representing the perpetual position (e.g., BTC-PERP).
    pub symbol: String,
    /// URL of the asset's image.
    pub asset_image_url: String,
    /// The volume-weighted average price (VWAP).
    pub vwap: MonetaryDetails,
    /// The side of the position (e.g., long, short).
    pub position_side: PositionSide,
    /// The net size of the position.
    pub net_size: String,
    /// Size of buy orders in the position.
    pub buy_order_size: String,
    /// Size of sell orders in the position.
    pub sell_order_size: String,
    /// Initial margin contribution for the position.
    pub im_contribution: String,
    /// Unrealized profit and loss for the position.
    pub unrealized_pnl: MonetaryDetails,
    /// The mark price of the position.
    pub mark_price: MonetaryDetails,
    /// The liquidation price of the position.
    pub liquidation_price: MonetaryDetails,
    /// Leverage used in the position.
    pub leverage: String,
    /// Initial margin notional value.
    pub im_notional: MonetaryDetails,
    /// Maintenance margin notional value.
    pub mm_notional: MonetaryDetails,
    /// Total notional value of the position.
    pub position_notional: MonetaryDetails,
    /// The margin type for the position (e.g., cross, isolated).
    pub margin_type: MarginType,
    /// The liquidation buffer for the position.
    pub liquidation_buffer: String,
    /// The liquidation percentage for the position.
    pub liquidation_percentage: String,
}

/// Futures position details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FuturesPosition {
    /// The product ID associated with the futures position.
    pub product_id: String,
    /// The contract size of the futures position.
    pub contract_size: String,
    /// The side of the futures position (e.g., long, short).
    pub side: PositionSide,
    /// The amount of the futures position.
    pub amount: String,
    /// The average entry price for the position.
    pub avg_entry_price: String,
    /// The current price of the futures position.
    pub current_price: String,
    /// Unrealized profit and loss for the futures position.
    pub unrealized_pnl: String,
    /// Expiry date of the futures contract.
    pub expiry: String,
    /// The underlying asset for the futures contract.
    pub underlying_asset: String,
    /// URL of the underlying asset's image.
    pub asset_img_url: String,
    /// The product name of the futures contract.
    pub product_name: String,
    /// The trading venue for the futures position.
    pub venue: String,
    /// The notional value of the futures position.
    pub notional_value: String,
}

/// Represents the breakdown of the portfolio returned by the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PortfolioBreakdown {
    /// The portfolio associated with the breakdown.
    pub portfolio: Portfolio,
    /// Balances across different portfolio categories.
    pub portfolio_balances: PortfolioBalances,
    /// Spot positions held in the portfolio.
    pub spot_positions: Vec<SpotPosition>,
    /// Perpetual positions held in the portfolio.
    pub perp_positions: Vec<PerpPosition>,
    /// Futures positions held in the portfolio.
    pub futures_positions: Vec<FuturesPosition>,
}

/// Represents a response for a portfolio breakdown.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct PortfolioBreakdownWrapper {
    /// The portfolio breakdown details.
    pub(crate) breakdown: PortfolioBreakdown,
}
