//! # Coinbase Advanced Order API
//!
//! `order` gives access to the Order API and the various endpoints associated with it.
//! These allow you to obtain past created orders, create new orders, and cancel orders.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::traits::Query;
use crate::utils::{deserialize_numeric, QueryBuilder};

/// Various order types.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderType {
    /// A Market order.
    Market,
    /// A Limit order.
    Limit,
    /// A stop order is an order that becomes a market order when triggered.
    Stop,
    /// A stop order is a limit order that doesn't go on the book until it hits the stop price.
    StopLimit,
}

impl AsRef<str> for OrderType {
    fn as_ref(&self) -> &str {
        match self {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
            OrderType::Stop => "STOP",
            OrderType::StopLimit => "STOPLIMIT",
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// Order side, BUY or SELL.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderSide {
    /// Buying a product.
    Buy,
    /// Selling a product.
    Sell,
}

impl AsRef<str> for OrderSide {
    fn as_ref(&self) -> &str {
        match self {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        }
    }
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// Order status, OPEN, CANCELLED, and EXPIRED.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderStatus {
    /// Implies the order is still available and not closed.
    Open,
    /// Order was closed by cancellation.
    Cancelled,
    /// Order was closed by expiration.
    Expired,
}

impl AsRef<str> for OrderStatus {
    fn as_ref(&self) -> &str {
        match self {
            OrderStatus::Open => "OPEN",
            OrderStatus::Cancelled => "CANCELLED",
            OrderStatus::Expired => "EXPIRED",
        }
    }
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// Order updates for a user from a websocket.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdate {
    /// Type of the update.
    pub r#type: String,
    /// Client Order ID (Normally a UUID)
    pub client_order_id: String,
    #[serde(deserialize_with = "deserialize_numeric")]
    pub cumulative_quantity: f64,
    #[serde(deserialize_with = "deserialize_numeric")]
    pub leaves_quantity: f64,
    /// Average price for the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub avg_price: f64,
    /// Total fees for the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub total_fees: f64,
    /// Status of the order.
    pub status: String,
    /// Product ID.
    pub product_id: String,
    /// Date-time when the order was created.
    pub creation_time: String,
    /// BUY or SELL.
    pub order_side: String,
    /// Type of the order.
    pub order_type: String,
}

/// Market Immediate or Cancel.
#[derive(Serialize, Debug)]
pub(crate) struct MarketIoc {
    /// Amount of quote currency to spend on order. Required for BUY orders.
    pub(crate) quote_size: Option<String>,
    /// Amount of base currency to spend on order. Required for SELL orders.
    pub(crate) base_size: Option<String>,
}

/// Limit Good til Cancelled.
#[derive(Serialize, Debug)]
pub(crate) struct LimitGtc {
    /// Amount of base currency to spend on order.
    pub(crate) base_size: String,
    /// Ceiling price for which the order should get filled.
    pub(crate) limit_price: String,
    /// Post only limit order.
    pub(crate) post_only: bool,
}

/// Limit Good til Time (Date).
#[derive(Serialize, Debug)]
pub(crate) struct LimitGtd {
    /// Amount of base currency to spend on order.
    pub(crate) base_size: String,
    /// Ceiling price for which the order should get filled.
    pub(crate) limit_price: String,
    /// Time at which the order should be cancelled if it's not filled.
    pub(crate) end_time: String,
    /// Post only limit order.
    pub(crate) post_only: bool,
}

/// Stop Limit Good til Cancelled.
#[derive(Serialize, Debug)]
pub(crate) struct StopLimitGtc {
    /// Amount of base currency to spend on order.
    pub(crate) base_size: String,
    /// Ceiling price for which the order should get filled.
    pub(crate) limit_price: String,
    /// Price at which the order should trigger - if stop direction is Up, then the order will trigger when the last trade price goes above this, otherwise order will trigger when last trade price goes below this price.
    pub(crate) stop_price: String,
    /// Possible values: [UNKNOWN_STOP_DIRECTION, STOP_DIRECTION_STOP_UP, STOP_DIRECTION_STOP_DOWN]
    pub(crate) stop_direction: String,
}

/// Stop Limit Good til Time (Date).
#[derive(Serialize, Debug)]
pub(crate) struct StopLimitGtd {
    /// Amount of base currency to spend on order.
    pub(crate) base_size: String,
    /// Ceiling price for which the order should get filled.
    pub(crate) limit_price: String,
    /// Price at which the order should trigger - if stop direction is Up, then the order will trigger when the last trade price goes above this, otherwise order will trigger when last trade price goes below this price.
    pub(crate) stop_price: String,
    /// Time at which the order should be cancelled if it's not filled.
    pub(crate) end_time: String,
    /// Possible values: [UNKNOWN_STOP_DIRECTION, STOP_DIRECTION_STOP_UP, STOP_DIRECTION_STOP_DOWN]
    pub(crate) stop_direction: String,
}

/// Create Order Configuration.
#[derive(Serialize, Default, Debug)]
pub(crate) struct OrderConfiguration {
    /// Market Order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) market_market_ioc: Option<MarketIoc>,
    /// Limit Order, Good til Cancelled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limit_limit_gtc: Option<LimitGtc>,
    /// Limit Order, Good til Date (time)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) limit_limit_gtd: Option<LimitGtd>,
    /// Stop Limit Order, Good til Cancelled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stop_limit_stop_limit_gtc: Option<StopLimitGtc>,
    /// Stop Limit Order, Good til Date (time)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stop_limit_stop_limit_gtd: Option<StopLimitGtd>,
}

/// Represents an order created to BUY or SELL.
#[derive(Serialize, Debug)]
pub(crate) struct CreateOrder {
    /// Client Order ID (UUID)
    pub(crate) client_order_id: String,
    /// Product ID (pair)
    pub(crate) product_id: String,
    /// Order Side: BUY or SELL.
    pub(crate) side: String,
    /// Configuration for the order.
    pub(crate) order_configuration: OrderConfiguration,
}

/// Represents an order to be edited.
#[derive(Serialize, Debug)]
pub(crate) struct EditOrder {
    /// ID of the order to edit.
    pub(crate) order_id: String,
    /// New price for order.
    pub(crate) price: String,
    /// New size for order.
    pub(crate) size: String,
}

/// Represents a vector of orders IDs to cancel.
#[derive(Serialize, Debug)]
pub(crate) struct CancelOrders {
    /// Vector of Order IDs to cancel.
    pub(crate) order_ids: Vec<String>,
}

/// Represents a single edit entry in the edit history of an order.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditHistory {
    /// The price associated with the edit.
    pub price: String,
    /// The size associated with the edit.
    pub size: String,
    /// The timestamp when the edit was accepted.
    pub replace_accept_timestamp: String,
}

/// Represents an Order received from the API.
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
    pub side: String,
    /// Possible values: [OPEN, FILLED, CANCELLED, EXPIRED, FAILED, UNKNOWN_ORDER_STATUS]
    pub status: String,
    /// Possible values: [UNKNOWN_TIME_IN_FORCE, GOOD_UNTIL_DATE_TIME, GOOD_UNTIL_CANCELLED, IMMEDIATE_OR_CANCEL, FILL_OR_KILL]
    pub time_in_force: String,
    /// Timestamp for when the order was created.
    pub created_time: String,
    /// The percent of total order amount that has been filled.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub completion_percentage: f64,
    /// The portion (in base currency) of total order amount that has been filled.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub filled_size: f64,
    /// The average of all prices of fills for this order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub average_filled_price: f64,
    /// Commission amount.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub fee: f64,
    /// Number of fills that have been posted for this order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub number_of_fills: u32,
    /// The portion (in quote current) of total order amount that has been filled.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub filled_value: f64,
    /// Whether a cancel request has been initiated for the order, and not yet completed.
    pub pending_cancel: bool,
    /// Whether the order was placed with quote currency/
    pub size_in_quote: bool,
    /// The total fees for the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub total_fees: f64,
    /// Whether the order size includes fees.
    pub size_inclusive_of_fees: bool,
    /// Derived field: filled_value + total_fees for buy orders and filled_value - total_fees for sell orders.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub total_value_after_fees: f64,
    /// Possible values: \[UNKNOWN_TRIGGER_STATUS, INVALID_ORDER_TYPE, STOP_PENDING, STOP_TRIGGERED\]
    pub trigger_status: String,
    /// Possible values: \[UNKNOWN_ORDER_TYPE, MARKET, LIMIT, STOP, STOP_LIMIT\]
    pub order_type: String,
    /// Possible values: \[REJECT_REASON_UNSPECIFIED\]
    pub reject_reason: String,
    /// True if the order is fully filled, false otherwise.
    pub settled: bool,
    /// Possible values: [SPOT, FUTURE]
    pub product_type: String,
    /// Message stating why the order was rejected.
    pub reject_message: String,
    /// Message stating why the order was canceled.
    pub cancel_message: String,
    /// An array of the latest 5 edits per order.
    pub edit_history: Vec<EditHistory>,
}

/// Represents a fill received from the API.
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
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price: f64,
    /// Amount of order that was transacted at this fill.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub size: f64,
    /// Fee amount for fill.
    #[serde(deserialize_with = "deserialize_numeric")]
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
    pub side: String,
}

/// Represents a list of orders received from the API.
#[derive(Deserialize, Debug)]
pub struct ListedOrders {
    /// Vector of orders obtained.
    pub orders: Vec<Order>,
    /// If there are additional orders.
    pub has_next: bool,
    /// Cursor used to pull more orders.
    pub cursor: String,
}

/// Represents a list of fills received from the API.
#[derive(Deserialize, Debug)]
pub struct ListedFills {
    /// Vector of filled orders.
    pub orders: Vec<Fill>,
    /// Cursor used to pull more fills.
    pub cursor: String,
}

/// Represents a create order response from the API.
#[derive(Deserialize, Debug)]
pub struct OrderResponse {
    /// Whether or not the order completed correctly.
    pub success: bool,
    /// Reason the order failed, if it did.
    pub failure_reason: String,
    /// Order Id of the order created.
    pub order_id: String,
}

/// Represents a cancel order response from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct CancelOrdersResponse {
    /// Vector of orders cancelled.
    pub(crate) results: Vec<OrderResponse>,
}

/// Represents an order when obtaining a single order from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct OrderStatusResponse {
    /// Order received.
    pub(crate) order: Order,
}

/// Represents an order when obtaining a single order from the API.
#[derive(Deserialize, Debug)]
pub struct EditOrderResponse {
    /// Whether or not the order edit succeeded.
    pub success: bool,
    /// Errors associated with the changes.
    pub errors: Vec<EditOrderErrors>,
}

/// Errors associated with the changes.
#[derive(Deserialize, Debug)]
pub struct EditOrderErrors {
    /// Reason the edit failed.
    pub edit_failure_reason: Option<String>,
    /// Reason the preview failed.
    pub preview_failure_reason: Option<String>,
}

/// Response from a preview edit order.
#[derive(Deserialize, Debug)]
pub struct PreviewEditOrderResponse {
    /// Contains reasons for failure in the edit or preview edit operation.
    pub errors: Vec<EditOrderErrors>,
    /// The amount of slippage in the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub slippage: f64,
    /// The total value of the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub order_total: f64,
    /// The total commission for the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub commission_total: f64,
    /// The size of the quote currency in the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub quote_size: f64,
    /// The size of the base currency in the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub base_size: f64,
    /// The best bid price at the time of the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub best_bid: f64,
    /// The best ask price at the time of the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub best_ask: f64,
    /// The average price at which the order was filled.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub average_filled_price: f64,
}

/// Represents parameters that are optional for List Orders API request.
#[derive(Serialize, Default, Debug)]
pub struct ListOrdersQuery {
    /// Optional string of the product ID. Defaults to null, or fetch for all products.
    pub product_id: Option<String>,
    /// Note: Cannot pair OPEN orders with other order types.
    pub order_status: Option<Vec<OrderStatus>>,
    /// A pagination limit with no default set. If has_next is true, additional orders are available to be fetched with pagination; also the cursor value in the response can be passed as cursor parameter in the subsequent request.
    pub limit: Option<u32>,
    /// Start date to fetch orders from, inclusive.
    pub start_date: Option<String>,
    /// An optional end date for the query window, exclusive. If provided only orders with creation time before this date will be returned.
    pub end_date: Option<String>,
    /// Type of orders to return. Default is to return all order types.
    pub order_type: Option<OrderType>,
    /// Only orders matching this side are returned. Default is to return all sides.
    pub order_side: Option<OrderSide>,
    /// Cursor used for pagination. When provided, the response returns responses after this cursor.
    pub cursor: Option<String>,
    /// Only orders matching this product type are returned. Default is to return all product types. Valid options are SPOT or FUTURE.
    pub product_type: Option<String>,
}

impl Query for ListOrdersQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional("product_id", &self.product_id)
            .with_optional_vec("order_status", &self.order_status)
            .push_u32_optional("limit", self.limit)
            .push_optional("start_date", &self.start_date)
            .push_optional("end_date", &self.end_date)
            .push_optional("order_type", &self.order_type)
            .push_optional("order_side", &self.order_side)
            .push_optional("cursor", &self.cursor)
            .push_optional("product_type", &self.product_type)
            .build()
    }
}

/// Represents parameters that are optional for List Fills API request.
#[derive(Serialize, Default, Debug)]
pub struct ListFillsQuery {
    /// ID of the order.
    pub order_id: Option<String>,
    /// The ID of the product this order was created for.
    pub product_id: Option<String>,
    /// Start date. Only fills with a trade time at or after this start date are returned.
    pub start_sequence_timestamp: Option<String>,
    /// End date. Only fills with a trade time before this start date are returned.
    pub end_sequence_timestamp: Option<String>,
    /// Maximum number of fills to return in response. Defaults to 100.
    pub limit: Option<u32>,
    /// Cursor used for pagination. When provided, the response returns responses after this cursor.
    pub cursor: Option<String>,
}

impl Query for ListFillsQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional("order_id", &self.order_id)
            .push_optional("product_id", &self.product_id)
            .push_optional("start_sequence_timestamp", &self.start_sequence_timestamp)
            .push_optional("end_sequence_timestamp", &self.end_sequence_timestamp)
            .push_u32_optional("limit", self.limit)
            .push_optional("cursor", &self.cursor)
            .build()
    }
}
