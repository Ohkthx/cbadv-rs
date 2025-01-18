//! # Coinbase Advanced Portfolio API
//!
//! `portfolio` gives access to the Potfolio API and the various endpoints associated with it.
//! This allows for the management of individual portfolios.

use core::fmt;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, DisplayFromStr};

use super::shared::Balance;
use crate::errors::CbError;
use crate::traits::{Query, Request};
use crate::types::CbResult;
use crate::utils::QueryBuilder;

/// Portfolio type for a user's portfolio.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PortfolioType {
    /// Portfolio type for a user's default portfolio.
    Default,
    /// Portfolios created by the user.
    Consumer,
    /// /// International Exchange portfolios.
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

/// Enum for `PositionSide` values.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PositionSide {
    #[serde(rename = "FUTURES_POSITION_SIDE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "FUTURES_POSITION_SIDE_LONG")]
    Long,
    #[serde(rename = "FUTURES_POSITION_SIDE_SHORT")]
    Short,
}

/// Enum for `MarginType` values.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginType {
    #[serde(rename = "MARGIN_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "MARGIN_TYPE_CROSS")]
    Cross,
    #[serde(rename = "MARGIN_TYPE_ISOLATED")]
    Isolated,
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
#[serde_as]
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
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub one_day_change: f64,
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
    pub net_size: f64,
    /// Size of buy orders in the position.
    pub buy_order_size: f64,
    /// Size of sell orders in the position.
    pub sell_order_size: f64,
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
    pub liquidation_percentage: f64,
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
    pub amount: f64,
    /// The average entry price for the position.
    pub avg_entry_price: f64,
    /// The current price of the futures position.
    pub current_price: f64,
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

/// Create or Edit an existing portfolio.
#[derive(Serialize, Default, Debug)]
pub struct PortfolioModifyRequest {
    /// New name of the portfolio.
    pub name: String,
}

impl Request for PortfolioModifyRequest {
    fn check(&self) -> CbResult<()> {
        if self.name.is_empty() {
            return Err(CbError::BadRequest(
                "portfolio name cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

impl PortfolioModifyRequest {
    /// Creates a new instance with the default values.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

/// Parameters for moving funds between portfolios.
#[derive(Serialize, Debug)]
pub struct PortfolioMoveFundsRequest {
    /// Funds to move between portfolios.
    pub funds: Balance,
    /// Portfolio funds to be removed from.
    pub source_portfolio_uuid: String,
    /// Portfolio funds to be added to.
    pub target_portfolio_uuid: String,
}

impl Request for PortfolioMoveFundsRequest {
    fn check(&self) -> CbResult<()> {
        if self.funds.value <= 0.0 {
            return Err(CbError::BadRequest(
                "funds to move must be greater than zero".to_string(),
            ));
        } else if self.funds.currency.is_empty() {
            return Err(CbError::BadRequest(
                "funds currency cannot be empty".to_string(),
            ));
        } else if self.source_portfolio_uuid.is_empty() {
            return Err(CbError::BadRequest(
                "source portfolio UUID cannot be empty".to_string(),
            ));
        } else if self.target_portfolio_uuid.is_empty() {
            return Err(CbError::BadRequest(
                "target portfolio UUID cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

impl PortfolioMoveFundsRequest {
    /// Creates a new instance of a request to move funds.
    ///
    /// # Arguements
    ///
    /// * `funds` - The amount of funds to move.
    /// * `source_portfolio_uuid` - The UUID of the source portfolio.
    //// * `target_portfolio_uuid` - The UUID of the target portfolio.
    pub fn new(funds: &Balance, source_portfolio_uuid: &str, target_portfolio_uuid: &str) -> Self {
        Self {
            funds: funds.clone(),
            source_portfolio_uuid: source_portfolio_uuid.to_string(),
            target_portfolio_uuid: target_portfolio_uuid.to_string(),
        }
    }
}

/// Query parameters for listing portfolios.
#[derive(Serialize, Default, Debug)]
pub struct PortfolioListQuery {
    /// Type of portfolios to list.
    pub portfolio_type: Option<PortfolioType>,
}

impl Query for PortfolioListQuery {
    fn check(&self) -> CbResult<()> {
        if let Some(portfolio_type) = &self.portfolio_type {
            if *portfolio_type == PortfolioType::Undefined {
                return Err(CbError::BadQuery(
                    "portfolio type cannot be undefined".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional("portfolio_type", self.portfolio_type.as_ref())
            .build()
    }
}

impl PortfolioListQuery {
    /// Creates a new instance with the default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the type of portfolios to list.
    pub fn portfolio_type(mut self, portfolio_type: PortfolioType) -> Self {
        self.portfolio_type = Some(portfolio_type);
        self
    }
}

/// Query parameters for a portfolio breakdown.
#[derive(Serialize, Default, Debug)]
pub struct PortfolioBreakdownQuery {
    /// Currency to use for the breakdown.
    pub currency: Option<String>,
}

impl Query for PortfolioBreakdownQuery {
    fn check(&self) -> CbResult<()> {
        Ok(())
    }

    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional("currency", self.currency.as_ref())
            .build()
    }
}

impl PortfolioBreakdownQuery {
    /// Creates a new instance with the default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the currency to use for the breakdown.
    pub fn currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.to_string());
        self
    }
}

/// Response for creating or editing a  portfolio.
#[derive(Deserialize, Debug)]
pub(crate) struct PortfolioWrapper {
    /// Updated portfolio from the API.
    pub(crate) portfolio: Portfolio,
}

impl From<PortfolioWrapper> for Portfolio {
    fn from(wrapper: PortfolioWrapper) -> Self {
        wrapper.portfolio
    }
}

/// Portfolio information returned from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct PortfoliosWrapper {
    pub(crate) portfolios: Vec<Portfolio>,
}

impl From<PortfoliosWrapper> for Vec<Portfolio> {
    fn from(wrapper: PortfoliosWrapper) -> Self {
        wrapper.portfolios
    }
}

/// Represents a response for a portfolio breakdown.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct PortfolioBreakdownWrapper {
    /// The portfolio breakdown details.
    pub(crate) breakdown: PortfolioBreakdown,
}

impl From<PortfolioBreakdownWrapper> for PortfolioBreakdown {
    fn from(wrapper: PortfolioBreakdownWrapper) -> Self {
        wrapper.breakdown
    }
}
