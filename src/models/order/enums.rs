//! # Coinbase Advanced Order API
//!
//! `order/enums` is the module containing the enums for the different order types and configurations.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::{
    LimitFok, LimitGtc, LimitGtd, MarketIoc, SorLimitIoc, StopLimitGtc, StopLimitGtd,
    TriggerBracketGtc, TriggerBracketGtd,
};

/// Various order types.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum OrderType {
    /// Unknown order type.
    #[serde(rename = "UNKNOWN_ORDER_TYPE")]
    Unknown,
    /// Buy or sell a specified quantity of an Asset at the current best available market price.
    Market,
    /// Buy or sell a specified quantity of an Asset at a specified price. The Order will only post to the Order Book if it will immediately Fill; any remaining quantity is canceled.
    Limit,
    /// Buy or sell a specified quantity of an Asset at a specified price. The Order will only post to the Order Book if it is to immediately and completely Fill.
    Stop,
    /// Buy or sell a specified quantity of an Asset at a specified price. The Order will only post to the Order Book if it is to immediately and completely Fill.
    StopLimit,
    /// A Limit Order to buy or sell a specified quantity of an Asset at a specified price, with stop limit order parameters embedded in the order. If posted, the Order will remain on the Order Book until canceled.
    Bracket,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for OrderType {
    fn as_ref(&self) -> &str {
        match self {
            OrderType::Unknown => "UNKNOWN_ORDER_TYPE",
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
            OrderType::Stop => "STOP",
            OrderType::StopLimit => "STOP_LIMIT",
            OrderType::Bracket => "BRACKET",
        }
    }
}

/// Order side, BUY or SELL.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    /// Unknown order side. Only used by remote API.
    #[serde(rename = "UNKNOWN_ORDER_SIDE")]
    Unknown,
    /// Buy order.
    Buy,
    /// Sell order.
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for OrderSide {
    fn as_ref(&self) -> &str {
        match self {
            OrderSide::Unknown => "UNKNOWN_ORDER_SIDE",
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        }
    }
}

/// Used to sort results.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSortBy {
    /// Unknown sort by.
    #[serde(rename = "UNKNOWN_SORT_BY")]
    Unknown,
    /// Sort by price.
    Price,
    /// Sort by trade time.
    TradeTime,
    /// Sort by limit price.
    LimitPrice,
    /// Sort by last fill time.
    LastFillTime,
}

impl fmt::Display for OrderSortBy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for OrderSortBy {
    fn as_ref(&self) -> &str {
        match self {
            OrderSortBy::Unknown => "UNKNOWN_SORT_BY",
            OrderSortBy::Price => "PRICE",
            OrderSortBy::TradeTime => "TRADE_TIME",
            OrderSortBy::LimitPrice => "LIMIT_PRICE",
            OrderSortBy::LastFillTime => "LAST_FILL_TIME",
        }
    }
}

/// Order status, OPEN, CANCELLED, and EXPIRED.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// Order is pending.
    Pending,
    /// Order is open.
    Open,
    /// Order is filled.
    Filled,
    /// Order is cancelled.
    Cancelled,
    /// Order is expired.
    Expired,
    /// Order failed.
    Failed,
    /// Unknown order status.
    #[serde(rename = "UNKNOWN_ORDER_STATUS")]
    Unknown,
    /// Order is queued.
    Queued,
    /// Order is queued to be cancelled.
    CancelQueued,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AsRef<str> for OrderStatus {
    fn as_ref(&self) -> &str {
        match self {
            OrderStatus::Pending => "PENDING",
            OrderStatus::Open => "OPEN",
            OrderStatus::Filled => "FILLED",
            OrderStatus::Cancelled => "CANCELLED",
            OrderStatus::Expired => "EXPIRED",
            OrderStatus::Failed => "FAILED",
            OrderStatus::Unknown => "UNKNOWN_ORDER_STATUS",
            OrderStatus::Queued => "QUEUED",
            OrderStatus::CancelQueued => "CANCEL_QUEUED",
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StopDirection {
    /// Unknown stop direction.
    #[serde(rename = "UNKNOWN_STOP_DIRECTION")]
    Unknown,
    /// Stop up direction.
    #[serde(rename = "STOP_DIRECTION_STOP_UP")]
    StopUp,
    /// Stop down direction.
    #[serde(rename = "STOP_DIRECTION_STOP_DOWN")]
    StopDown,
}

impl fmt::Display for StopDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    /// Unknown time in force.
    #[serde(rename = "UNKNOWN_TIME_IN_FORCE")]
    Unknown,
    /// Good 'til Cancelled
    GoodUntilCancelled,
    /// Good 'til Date
    #[serde(rename = "GOOD_UNTIL_DATE_TIME")]
    GoodUntilDate,
    /// Immediate or Cancel
    ImmediateOrCancel,
    /// Fill or Kill
    FillOrKill,
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for TimeInForce {
    fn as_ref(&self) -> &str {
        match self {
            TimeInForce::Unknown => "UNKNOWN_TIME_IN_FORCE",
            TimeInForce::GoodUntilCancelled => "GOOD_TIL_CANCELLED",
            TimeInForce::GoodUntilDate => "GOOD_TIL_DATE_TIME",
            TimeInForce::ImmediateOrCancel => "IMMEDIATE_OR_CANCEL",
            TimeInForce::FillOrKill => "FILL_OR_KILL",
        }
    }
}

/// Enum representing the different possible trigger statuses.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TriggerStatus {
    /// Unknown time in force.
    #[serde(rename = "UNKNOWN_TRIGGER_STATUS")]
    Unknown,
    /// Invalid order type.
    InvalidOrderType,
    /// Stop pending.
    StopPending,
    /// Stop triggered.
    StopTriggered,
}

impl fmt::Display for TriggerStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AsRef<str> for TriggerStatus {
    fn as_ref(&self) -> &str {
        match self {
            TriggerStatus::Unknown => "UNKNOWN_TRIGGER_STATUS",
            TriggerStatus::InvalidOrderType => "INVALID_ORDER_TYPE",
            TriggerStatus::StopPending => "STOP_PENDING",
            TriggerStatus::StopTriggered => "STOP_TRIGGERED",
        }
    }
}

/// Enum representing reasons for rejecting an order.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RejectReason {
    /// Unspecified reject reason.
    #[serde(rename = "REJECT_REASON_UNSPECIFIED")]
    Unspecified,
    /// Hold failure reject reason.
    #[serde(rename = "HOLD_FAILURE")]
    HoldFailure,
    /// Too many open orders reject reason.
    TooManyOpenOrders,
    /// Insufficient funds reject reason.
    #[serde(rename = "REJECT_REASON_INSUFFICIENT_FUNDS")]
    InsufficientFunds,
    /// Rate limit exceeded reject reason.
    RateLimitExceeded,
}

impl fmt::Display for RejectReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AsRef<str> for RejectReason {
    fn as_ref(&self) -> &str {
        match self {
            RejectReason::Unspecified => "REJECT_REASON_UNSPECIFIED",
            RejectReason::HoldFailure => "HOLD_FAILURE",
            RejectReason::TooManyOpenOrders => "TOO_MANY_OPEN_ORDERS",
            RejectReason::InsufficientFunds => "REJECT_REASON_INSUFFICIENT_FUNDS",
            RejectReason::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
        }
    }
}

/// Enum representing the different possible order configurations.
#[derive(Serialize, Debug, Clone)]
pub enum OrderConfiguration {
    /// Market Immediate or Cancel Order.
    #[serde(rename = "market_market_ioc")]
    MarketIoc(MarketIoc),
    /// Only posts if it will immediately fill, remaining quantity is cancelled.
    #[serde(rename = "sor_limit_ioc")]
    SorLimitIoc(SorLimitIoc),
    /// Limit Good 'til Cancelled Order.
    #[serde(rename = "limit_limit_gtc")]
    LimitGtc(LimitGtc),
    /// Limit Good 'til Date (time) Order.
    #[serde(rename = "limit_limit_gtd")]
    LimitGtd(LimitGtd),
    /// Order only posts if it is fill entirely, otherwise cancelled.
    #[serde(rename = "limit_limit_fok")]
    LimitFok(LimitFok),
    /// Stop Limit Good 'til Cancelled Order.
    #[serde(rename = "stop_limit_stop_limit_gtc")]
    StopLimitGtc(StopLimitGtc),
    /// Stop Limit Good 'til Date (time) Order.
    #[serde(rename = "stop_limit_stop_limit_gtd")]
    StopLimitGtd(StopLimitGtd),
    /// Trigger Bracket 'til Cancelled Order.
    #[serde(rename = "trigger_bracket_gtc")]
    TriggerBracketGtc(TriggerBracketGtc),
    /// Trigger Bracket 'til Date (time) Order.
    #[serde(rename = "trigger_bracket_gtd")]
    TriggerBracketGtd(TriggerBracketGtd),
}
