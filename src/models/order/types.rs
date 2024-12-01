//! # Coinbase Advanced Order API
//!
//! `order/types` is the module containing the structs for the different order types and configurations.

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, DisplayFromStr};

use crate::product::ProductType;

use super::{
    OrderSide, OrderStatus, OrderType, RejectReason, StopDirection, TimeInForce, TriggerStatus,
};

/// Buy or sell a specified quantity of an Asset at the current best available market price.
#[serde_as]
#[derive(Serialize, Debug, Clone)]
pub struct MarketIoc {
    /// Amount of quote currency to spend on order. Required for BUY orders.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub quote_size: Option<f64>,
    /// Amount of base currency to spend on order. Required for SELL orders.
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub base_size: Option<f64>,
}

/// Buy or sell a specified quantity of an Asset at a specified price. The Order will only post to the Order Book if it will immediately Fill; any remaining quantity is canceled.
#[serde_as]
#[derive(Serialize, Debug, Clone)]
pub struct SorLimitIoc {
    /// Amount of base currency to spend on order.
    #[serde_as(as = "DisplayFromStr")]
    pub base_size: f64,
    /// Ceiling price for which the order should get filled.
    #[serde_as(as = "DisplayFromStr")]
    pub limit_price: f64,
}

/// Limit Good til Cancelled.
#[serde_as]
#[derive(Serialize, Debug, Clone)]
pub struct LimitGtc {
    /// Amount of base currency to spend on order.
    #[serde_as(as = "DisplayFromStr")]
    pub base_size: f64,
    /// Ceiling price for which the order should get filled.
    #[serde_as(as = "DisplayFromStr")]
    pub limit_price: f64,
    /// Post only limit order.
    pub post_only: bool,
}

/// Limit Good til Time (Date).
#[serde_as]
#[derive(Serialize, Debug, Clone)]
pub struct LimitGtd {
    /// Amount of base currency to spend on order.
    #[serde_as(as = "DisplayFromStr")]
    pub base_size: f64,
    /// Ceiling price for which the order should get filled.
    #[serde_as(as = "DisplayFromStr")]
    pub limit_price: f64,
    /// Time at which the order should be cancelled if it's not filled.
    pub end_time: String,
    /// Post only limit order.
    pub post_only: bool,
}

/// Buy or sell a specified quantity of an Asset at a specified price. The Order will only post to the Order Book if it is to immediately and completely Fill.
#[serde_as]
#[derive(Serialize, Debug, Clone)]
pub struct LimitFok {
    /// Amount of base currency to spend on order.
    #[serde_as(as = "DisplayFromStr")]
    pub base_size: f64,
    /// Ceiling price for which the order should get filled.
    #[serde_as(as = "DisplayFromStr")]
    pub limit_price: f64,
}

/// Stop Limit Good til Cancelled.
#[serde_as]
#[derive(Serialize, Debug, Clone)]
pub struct StopLimitGtc {
    /// Amount of base currency to spend on order.
    #[serde_as(as = "DisplayFromStr")]
    pub base_size: f64,
    /// Ceiling price for which the order should get filled.
    #[serde_as(as = "DisplayFromStr")]
    pub limit_price: f64,
    /// Price at which the order should trigger - if stop direction is Up, then the order will trigger when the last trade price goes above this, otherwise order will trigger when last trade price goes below this price.
    #[serde_as(as = "DisplayFromStr")]
    pub stop_price: f64,
    /// Possible values: [UNKNOWN_STOP_DIRECTION, STOP_DIRECTION_STOP_UP, STOP_DIRECTION_STOP_DOWN]
    pub stop_direction: StopDirection,
}

/// Stop Limit Good til Time (Date).
#[serde_as]
#[derive(Serialize, Debug, Clone)]
pub struct StopLimitGtd {
    /// Amount of base currency to spend on order.
    #[serde_as(as = "DisplayFromStr")]
    pub base_size: f64,
    /// Ceiling price for which the order should get filled.
    #[serde_as(as = "DisplayFromStr")]
    pub limit_price: f64,
    /// Price at which the order should trigger - if stop direction is Up, then the order will trigger when the last trade price goes above this, otherwise order will trigger when last trade price goes below this price.
    #[serde_as(as = "DisplayFromStr")]
    pub stop_price: f64,
    /// Time at which the order should be cancelled if it's not filled.
    pub end_time: String,
    /// Possible values: [UNKNOWN_STOP_DIRECTION, STOP_DIRECTION_STOP_UP, STOP_DIRECTION_STOP_DOWN]
    pub stop_direction: StopDirection,
}

/// A Limit Order to buy or sell a specified quantity of an Asset at a specified price, with stop limit order parameters embedded in the order. If posted, the Order will remain on the Order Book until canceled.
#[serde_as]
#[derive(Serialize, Debug, Clone)]
pub struct TriggerBracketGtc {
    /// Amount of base currency to spend on order.
    #[serde_as(as = "DisplayFromStr")]
    pub base_size: f64,
    /// Ceiling price for which the order should get filled.
    #[serde_as(as = "DisplayFromStr")]
    pub limit_price: f64,
    /// The price level (in quote currency) where the position will be exited. When triggered, a stop limit order is automatically placed with a limit price 5% higher for BUYS and 5% lower for SELLS.
    #[serde_as(as = "DisplayFromStr")]
    pub stop_trigger_price: f64,
}

/// A Limit Order to buy or sell a specified quantity of an Asset at a specified price, with stop limit order parameters embedded in the order. If posted, the Order will remain on the Order Book until a certain time is reached or the Order is canceled.
#[serde_as]
#[derive(Serialize, Debug, Clone)]
pub struct TriggerBracketGtd {
    /// Amount of base currency to spend on order.
    #[serde_as(as = "DisplayFromStr")]
    pub base_size: f64,
    /// Ceiling price for which the order should get filled.
    #[serde_as(as = "DisplayFromStr")]
    pub limit_price: f64,
    /// The price level (in quote currency) where the position will be exited. When triggered, a stop limit order is automatically placed with a limit price 5% higher for BUYS and 5% lower for SELLS.
    #[serde_as(as = "DisplayFromStr")]
    pub stop_trigger_price: f64,
    /// Time at which the order should be cancelled if it's not filled.
    pub end_time: String,
}

/// Represents a single edit entry in the edit history of an order.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditHistory {
    /// The price associated with the edit.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub price: f64,
    /// The size associated with the edit.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub size: f64,
    /// The timestamp when the edit was accepted.
    pub replace_accept_timestamp: String,
}

/// Represents an Order received from the API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    /// The unique id for this order.
    pub order_id: String,
    /// Client specified ID of order.
    pub client_order_id: String,
    /// The product this order was created for e.g. 'BTC-USD'
    pub product_id: String,
    /// The id of the User owning this Order.
    pub user_id: String,
    /// Possible values: [UNKNOWN_ORDER_SIDE, BUY, SELL]
    pub side: OrderSide,
    /// Possible values: [OPEN, FILLED, CANCELLED, EXPIRED, FAILED, UNKNOWN_ORDER_STATUS]
    pub status: OrderStatus,
    /// Possible values: [UNKNOWN_TIME_IN_FORCE, GOOD_UNTIL_DATE_TIME, GOOD_UNTIL_CANCELLED, IMMEDIATE_OR_CANCEL, FILL_OR_KILL]
    pub time_in_force: TimeInForce,
    /// Timestamp for when the order was created.
    pub created_time: String,
    /// The percent of total order amount that has been filled.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub completion_percentage: f64,
    /// The portion (in base currency) of total order amount that has been filled.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub filled_size: f64,
    /// The average of all prices of fills for this order.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub average_filled_price: f64,
    /// Commission amount.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub fee: f64,
    /// Number of fills that have been posted for this order.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub number_of_fills: u32,
    /// The portion (in quote current) of total order amount that has been filled.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub filled_value: f64,
    /// Whether a cancel request has been initiated for the order, and not yet completed.
    pub pending_cancel: bool,
    /// Whether the order was placed with quote currency/
    pub size_in_quote: bool,
    /// The total fees for the order.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub total_fees: f64,
    /// Whether the order size includes fees.
    pub size_inclusive_of_fees: bool,
    /// Derived field: filled_value + total_fees for buy orders and filled_value - total_fees for sell orders.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub total_value_after_fees: f64,
    /// Possible values: \[UNKNOWN_TRIGGER_STATUS, INVALID_ORDER_TYPE, STOP_PENDING, STOP_TRIGGERED\]
    pub trigger_status: TriggerStatus,
    /// Possible values: \[UNKNOWN_ORDER_TYPE, MARKET, LIMIT, STOP, STOP_LIMIT\]
    pub order_type: OrderType,
    /// Possible values: \[REJECT_REASON_UNSPECIFIED\]
    pub reject_reason: RejectReason,
    /// True if the order is fully filled, false otherwise.
    pub settled: bool,
    /// Possible values: [SPOT, FUTURE]
    pub product_type: ProductType,
    /// Message stating why the order was rejected.
    pub reject_message: String,
    /// Message stating why the order was canceled.
    pub cancel_message: String,
    /// An array of the latest 5 edits per order.
    pub edit_history: Vec<EditHistory>,
}

/// Represents a fill received from the API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fill {
    /// Unique identifier for the fill.
    pub entry_id: String,
    /// Id of the fill -- unique for all `FILL` trade_types but not unique for adjusted fills.
    pub trade_id: String,
    /// Id of the order the fill belongs to.
    pub order_id: String,
    /// Time at which this fill was completed.
    pub trade_time: String,
    /// String denoting what type of fill this is. Regular fills have the value `FILL`.
    /// Adjusted fills have possible values `REVERSAL`, `CORRECTION`, `SYNTHETIC`.
    pub trade_type: String,
    /// Price the fill was posted at.
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    /// Amount of order that was transacted at this fill.
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    /// Fee amount for fill.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub commission: f64,
    /// The product this order was created for.
    pub product_id: String,
    /// Time at which this fill was posted.
    pub sequence_timestamp: String,
    /// Possible values: [UNKNOWN_LIQUIDITY_INDICATOR, MAKER, TAKER]
    pub liquidity_indicator: String,
    /// Whether the order was placed with quote currency.
    pub size_in_quote: bool,
    /// User that placed the order the fill belongs to.
    pub user_id: String,
    /// Possible values: [UNKNOWN_ORDER_SIDE, BUY, SELL]
    pub side: OrderSide,
}

/// Represents a list of orders received from the API.
#[derive(Deserialize, Debug)]
pub struct PaginatedOrders {
    /// Vector of orders obtained.
    pub orders: Vec<Order>,
    /// If there are additional orders.
    pub has_next: bool,
    /// Cursor used to pull more orders.
    pub cursor: String,
}

/// Represents a list of fills received from the API.
#[derive(Deserialize, Debug)]
pub struct PaginatedFills {
    /// Vector of filled orders.
    pub orders: Vec<Fill>,
    /// Cursor used to pull more fills.
    pub cursor: String,
}

/// Contains information when an order is successfully created.
#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessResponse {
    /// The ID of the order.
    pub order_id: String,
    /// The trading pair (e.g., 'BTC-USD').
    pub product_id: String,
    /// The side of the market that the order is on ('BUY' or 'SELL').
    pub side: OrderSide,
    /// The unique ID provided for the order (used for identification purposes).
    pub client_order_id: String,
}

/// Contains error information when an order fails to be created.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    /// **(Deprecated)** The reason the order failed to be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Generic error message explaining why the order was not created.
    pub message: Option<String>,
    /// Descriptive error message explaining why the order was not created.
    pub error_details: Option<String>,
    /// **(Deprecated)** The reason the order failed during preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_failure_reason: Option<String>,
    /// The reason the order failed to be created.
    pub new_order_failure_reason: String,
}

/// Represents a create, edit, or cancel order response from the API.
#[derive(Deserialize, Debug)]
pub struct OrderCreateResponse {
    /// Whether the order was successfully created.
    pub success: bool,
    /// Contains information if the order was successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_response: Option<SuccessResponse>,
    /// Contains error information if the order failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_response: Option<ErrorResponse>,
}

/// Represents a cancel order response from the API.
#[derive(Deserialize, Debug)]
pub struct OrderCancelResponse {
    /// Whether the order was successfully cancelled.
    pub success: bool,
    /// Failure reason.
    pub failure_reason: String,
    /// Order ID.
    pub order_id: String,
}

/// Represents an order when obtaining a single order from the API.
#[derive(Deserialize, Debug)]
pub struct OrderEditResponse {
    /// Whether or not the order edit succeeded.
    pub success: bool,
    /// Errors associated with the changes.
    pub errors: Vec<OrderEditError>,
}

/// Errors associated with the changes.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrderEditError {
    /// Reason the edit failed.
    pub edit_failure_reason: Option<String>,
    /// Reason the preview failed.
    pub preview_failure_reason: Option<String>,
}

/// Response from a preview edit order.
#[serde_as]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrderEditPreview {
    /// Contains reasons for failure in the edit or preview edit operation.
    pub errors: Vec<OrderEditError>,
    /// The amount of slippage in the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub slippage: f64,
    /// The total value of the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub order_total: f64,
    /// The total commission for the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub commission_total: f64,
    /// The size of the quote currency in the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub quote_size: f64,
    /// The size of the base currency in the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub base_size: f64,
    /// The best bid price at the time of the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub best_bid: f64,
    /// The best ask price at the time of the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub best_ask: f64,
    /// The average price at which the order was filled.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub average_filled_price: f64,
}

/// Represents the response for a preview of creating an order.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderCreatePreview {
    /// The total value of the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub order_total: f64,
    /// The total commission for the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub commission_total: f64,
    /// List of errors encountered during the preview.
    pub errs: Vec<String>,
    /// List of warnings related to the order preview.
    pub warning: Vec<String>,
    /// The best bid price at the time of the preview.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub best_bid: f64,
    /// The best ask price at the time of the preview.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub best_ask: f64,
    /// The size of the quote currency in the order.
    /// NOTE: There were issues deserializing this in the past.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub quote_size: f64,
    /// The size of the base currency in the order.
    /// NOTE: There were issues deserializing this in the past.
    #[serde_as(as = "DisplayFromStr")]
    pub base_size: f64,
    /// Indicates whether the maximum allowed amount was used.
    pub is_max: bool,
    /// The total margin required for the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub order_margin_total: f64,
    /// The leverage applied to the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub leverage: f64,
    /// The long leverage available for the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub long_leverage: f64,
    /// The short leverage available for the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub short_leverage: f64,
    /// The projected slippage for the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub slippage: f64,
    /// The unique identifier for the order preview.
    pub preview_id: String,
    /// The current liquidation buffer for the account.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub current_liquidation_buffer: f64,
    /// The projected liquidation buffer after the order.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(default)]
    pub projected_liquidation_buffer: f64,
    /// The maximum leverage available for the order.
    #[serde_as(as = "DefaultOnError<Option<DisplayFromStr>>")]
    #[serde(default)]
    pub max_leverage: Option<f64>,
}

/// Represents a cancel order response from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct OrderCancelWrapper {
    /// Vector of orders cancelled.
    pub(crate) results: Vec<OrderCancelResponse>,
}

impl From<OrderCancelWrapper> for Vec<OrderCancelResponse> {
    fn from(wrapper: OrderCancelWrapper) -> Self {
        wrapper.results
    }
}

/// Represents an order when obtaining a single order from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct OrderWrapper {
    /// Order received.
    pub(crate) order: Order,
}

impl From<OrderWrapper> for Order {
    fn from(wrapper: OrderWrapper) -> Self {
        wrapper.order
    }
}
