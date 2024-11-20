//! # Coinbase Advanced Order API
//!
//! `order` gives access to the Order API and the various endpoints associated with it.
//! These allow you to obtain past created orders, create new orders, and cancel orders.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::errors::CbAdvError;
use crate::traits::Query;
use crate::types::CbResult;
use crate::utils::{deserialize_numeric, QueryBuilder};

/// Various order types.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    /// Unknown order type.
    UnknownOrderType,
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
            OrderType::UnknownOrderType => "UNKNOWN_ORDER_TYPE",
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
            OrderType::Stop => "STOP",
            OrderType::StopLimit => "STOP_LIMIT",
            OrderType::Bracket => "BRACKET",
        }
    }
}

/// Order side, BUY or SELL.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
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
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        }
    }
}

/// Order status, OPEN, CANCELLED, and EXPIRED.
#[derive(Serialize, Deserialize, Debug, Clone)]
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
    UnknownOrderStatus,
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
            OrderStatus::UnknownOrderStatus => "UNKNOWN_ORDER_STATUS",
            OrderStatus::Queued => "QUEUED",
            OrderStatus::CancelQueued => "CANCEL_QUEUED",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    /// Good 'til Cancelled
    Gtc,
    /// Good 'til Date
    Gtd,
    /// Immediate or Cancel
    Ioc,
    /// Fill or Kill
    Fok,
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for TimeInForce {
    fn as_ref(&self) -> &str {
        match self {
            TimeInForce::Gtc => "GTC",
            TimeInForce::Gtd => "GTD",
            TimeInForce::Ioc => "IOC",
            TimeInForce::Fok => "FOK",
        }
    }
}

/// Buy or sell a specified quantity of an Asset at the current best available market price.
#[derive(Serialize, Debug, Clone)]
pub struct MarketIoc {
    /// Amount of quote currency to spend on order. Required for BUY orders.
    pub quote_size: Option<String>,
    /// Amount of base currency to spend on order. Required for SELL orders.
    pub base_size: Option<String>,
}

/// Buy or sell a specified quantity of an Asset at a specified price. The Order will only post to the Order Book if it will immediately Fill; any remaining quantity is canceled.
#[derive(Serialize, Debug, Clone)]
pub struct SorLimitIoc {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
}

/// Limit Good til Cancelled.
#[derive(Serialize, Debug, Clone)]
pub struct LimitGtc {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// Post only limit order.
    pub post_only: bool,
}

/// Limit Good til Time (Date).
#[derive(Serialize, Debug, Clone)]
pub struct LimitGtd {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// Time at which the order should be cancelled if it's not filled.
    pub end_time: String,
    /// Post only limit order.
    pub post_only: bool,
}

/// Buy or sell a specified quantity of an Asset at a specified price. The Order will only post to the Order Book if it is to immediately and completely Fill.
#[derive(Serialize, Debug, Clone)]
pub struct LimitFok {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
}

/// Stop Limit Good til Cancelled.
#[derive(Serialize, Debug, Clone)]
pub struct StopLimitGtc {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// Price at which the order should trigger - if stop direction is Up, then the order will trigger when the last trade price goes above this, otherwise order will trigger when last trade price goes below this price.
    pub stop_price: String,
    /// Possible values: [UNKNOWN_STOP_DIRECTION, STOP_DIRECTION_STOP_UP, STOP_DIRECTION_STOP_DOWN]
    pub stop_direction: StopDirection,
}

/// Stop Limit Good til Time (Date).
#[derive(Serialize, Debug, Clone)]
pub struct StopLimitGtd {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// Price at which the order should trigger - if stop direction is Up, then the order will trigger when the last trade price goes above this, otherwise order will trigger when last trade price goes below this price.
    pub stop_price: String,
    /// Time at which the order should be cancelled if it's not filled.
    pub end_time: String,
    /// Possible values: [UNKNOWN_STOP_DIRECTION, STOP_DIRECTION_STOP_UP, STOP_DIRECTION_STOP_DOWN]
    pub stop_direction: StopDirection,
}

/// A Limit Order to buy or sell a specified quantity of an Asset at a specified price, with stop limit order parameters embedded in the order. If posted, the Order will remain on the Order Book until canceled.
#[derive(Serialize, Debug, Clone)]
pub struct TriggerBracketGtc {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// The price level (in quote currency) where the position will be exited. When triggered, a stop limit order is automatically placed with a limit price 5% higher for BUYS and 5% lower for SELLS.
    pub stop_trigger_price: String,
}

/// A Limit Order to buy or sell a specified quantity of an Asset at a specified price, with stop limit order parameters embedded in the order. If posted, the Order will remain on the Order Book until a certain time is reached or the Order is canceled.
#[derive(Serialize, Debug, Clone)]
pub struct TriggerBracketGtd {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// The price level (in quote currency) where the position will be exited. When triggered, a stop limit order is automatically placed with a limit price 5% higher for BUYS and 5% lower for SELLS.
    pub stop_trigger_price: String,
    /// Time at which the order should be cancelled if it's not filled.
    pub end_time: String,
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

/// A builder for creating `CreateOrder` instances.
///
/// This builder provides a fluent interface to construct an order by specifying the product,
/// side, order type, time-in-force, and other optional parameters. It ensures that all required
/// parameters are set before building the final order, and helps prevent invalid configurations.
pub struct CreateOrderBuilder {
    product_id: String,
    side: OrderSide,
    is_preview: bool,
    order_type: Option<OrderType>,
    time_in_force: Option<TimeInForce>,
    base_size: Option<String>,
    quote_size: Option<String>,
    limit_price: Option<String>,
    stop_price: Option<String>,
    stop_trigger_price: Option<String>,
    end_time: Option<String>,
    post_only: Option<bool>,
    stop_direction: Option<StopDirection>,
    client_order_id: Option<String>,
}

impl CreateOrderBuilder {
    /// Creates a new `CreateOrderBuilder` instance.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The trading pair (e.g., "BTC-USD") for which the order will be created.
    ///   This must be a valid product ID supported by the exchange.
    /// * `side` - The side of the order, either `BUY` or `SELL`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let builder = CreateOrderBuilder::new("BTC-USD", &OrderSide::Buy);
    /// ```
    pub fn new(product_id: &str, side: &OrderSide) -> Self {
        Self {
            product_id: product_id.to_string(),
            side: side.clone(),
            is_preview: false,
            order_type: None,
            time_in_force: None,
            base_size: None,
            quote_size: None,
            limit_price: None,
            stop_price: None,
            stop_trigger_price: None,
            end_time: None,
            post_only: None,
            stop_direction: None,
            client_order_id: None,
        }
    }

    /// Sets the order type for the order.
    ///
    /// The order type determines the kind of order to be placed, such as `Market`, `Limit`,
    /// `StopLimit`, or `Trigger`. This setting affects which additional parameters are required.
    ///
    /// # Arguments
    ///
    /// * `order_type` - An `OrderType` enum variant specifying the type of order.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.order_type(OrderType::Limit);
    /// ```
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    /// Sets the time-in-force policy for the order.
    ///
    /// Time-in-force specifies how long an order remains active before it is executed or expires.
    /// Common values include:
    ///
    /// - `GTC` (Good 'til Cancelled): The order remains active until it is filled or canceled.
    /// - `GTD` (Good 'til Date): The order remains active until a specified date and time.
    /// - `IOC` (Immediate or Cancel): The order must be executed immediately; otherwise, any unfilled portion is canceled.
    /// - `FOK` (Fill or Kill): The order must be filled entirely immediately; otherwise, it is canceled.
    ///
    /// # Arguments
    ///
    /// * `tif` - A `TimeInForce` enum variant specifying the time-in-force policy.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.time_in_force(TimeInForce::GTC);
    /// ```
    pub fn time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Sets the base size for the order.
    ///
    /// The base size is the amount of the base currency to buy or sell. For example, in the "BTC-USD"
    /// trading pair, BTC is the base currency.
    ///
    /// # Arguments
    ///
    /// * `base_size` - The quantity of the base currency to trade.
    ///
    /// # Note
    ///
    /// This parameter is required for most order types except when specifying `quote_size` for certain market orders.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.base_size(0.5); // Buying or selling 0.5 BTC
    /// ```
    pub fn base_size(mut self, base_size: f64) -> Self {
        self.base_size = Some(base_size.to_string());
        self
    }

    /// Sets the quote size for the order.
    ///
    /// The quote size is the amount of the quote currency to spend (for buys) or receive (for sells).
    /// For example, in the "BTC-USD" trading pair, USD is the quote currency.
    ///
    /// # Arguments
    ///
    /// * `quote_size` - The amount of the quote currency to use in the order.
    ///
    /// # Note
    ///
    /// - For market orders, you can specify either `base_size` or `quote_size`.
    /// - `quote_size` is not typically used with limit orders.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.quote_size(1000.0); // Spending $1000 USD to buy BTC
    /// ```
    pub fn quote_size(mut self, quote_size: f64) -> Self {
        self.quote_size = Some(quote_size.to_string());
        self
    }

    /// Sets the limit price for the order.
    ///
    /// The limit price is the worst price at which the order will be executed:
    ///
    /// - For **buy** orders, it's the maximum price you're willing to pay per unit of the base currency.
    /// - For **sell** orders, it's the minimum price you're willing to accept per unit of the base currency.
    ///
    /// # Arguments
    ///
    /// * `limit_price` - The limit price in terms of the quote currency.
    ///
    /// # Note
    ///
    /// This parameter is required for limit orders and stop limit orders.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.limit_price(50000.0); // Limit price of $50,000 per BTC
    /// ```
    pub fn limit_price(mut self, limit_price: f64) -> Self {
        self.limit_price = Some(limit_price.to_string());
        self
    }

    /// Sets the stop price for the order.
    ///
    /// The stop price is the price at which a stop order is triggered and becomes active.
    /// When the market reaches the stop price, the stop order is converted into a regular order (e.g., a limit order).
    ///
    /// # Arguments
    ///
    /// * `stop_price` - The price at which the stop order is triggered.
    ///
    /// # Note
    ///
    /// - Required for stop limit orders.
    /// - The `stop_direction` must also be specified.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.stop_price(48000.0); // Trigger the order when the price reaches $48,000
    /// ```
    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price.to_string());
        self
    }

    /// Sets the stop trigger price for a trigger bracket order.
    ///
    /// The stop trigger price is the price level at which the position will be exited.
    /// When the market reaches this price, a stop limit order is automatically placed.
    ///
    /// # Arguments
    ///
    /// * `stop_trigger_price` - The price level to trigger the exit order.
    ///
    /// # Note
    ///
    /// - Required for trigger bracket orders.
    /// - The exit order typically has a limit price adjusted based on the side of the original order.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.stop_trigger_price(47000.0); // Exit the position when the price reaches $47,000
    /// ```
    pub fn stop_trigger_price(mut self, stop_trigger_price: f64) -> Self {
        self.stop_trigger_price = Some(stop_trigger_price.to_string());
        self
    }

    /// Sets the end time for the order.
    ///
    /// The end time is the timestamp at which the order will be automatically canceled if it has not been filled.
    /// It is used with Good 'til Date (GTD) orders.
    ///
    /// # Arguments
    ///
    /// * `end_time` - The end time as an RFC3339 formatted timestamp (e.g., "2024-12-31T23:59:59Z").
    ///
    /// # Note
    ///
    /// This parameter is required for orders with a time-in-force of `GTD`.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.end_time("2024-12-31T23:59:59Z");
    /// ```
    pub fn end_time(mut self, end_time: &str) -> Self {
        self.end_time = Some(end_time.to_string());
        self
    }

    /// Sets the post-only flag for the order.
    ///
    /// When `post_only` is set to `true`, the order will only be posted to the order book if it does not
    /// immediately match with an existing order. This ensures that the order will be a maker order, not a taker.
    ///
    /// # Arguments
    ///
    /// * `post_only` - A boolean indicating whether to enable post-only mode.
    ///
    /// # Note
    ///
    /// - Applicable to limit orders.
    /// - If an order would be immediately matched (taking liquidity), it will be rejected if `post_only` is `true`.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.post_only(true);
    /// ```
    pub fn post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    /// Sets the stop direction for a stop order.
    ///
    /// The stop direction determines whether the stop order is triggered when the market price moves up or down:
    ///
    /// - `StopUp`: The order triggers when the last trade price **rises** to or above the stop price.
    /// - `StopDown`: The order triggers when the last trade price **falls** to or below the stop price.
    ///
    /// # Arguments
    ///
    /// * `stop_direction` - A `StopDirection` enum variant specifying the trigger direction.
    ///
    /// # Note
    ///
    /// Required for stop limit orders.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.stop_direction(StopDirection::StopUp);
    /// ```
    pub fn stop_direction(mut self, stop_direction: StopDirection) -> Self {
        self.stop_direction = Some(stop_direction);
        self
    }

    /// Sets the client-defined order ID.
    ///
    /// The `client_order_id` is a unique identifier supplied by the client to identify the order.
    /// If not provided, a random UUID will be generated. This can be useful for tracking orders in your system.
    ///
    /// # Arguments
    ///
    /// * `client_order_id` - A string representing the client-defined order ID.
    ///
    /// # Note
    ///
    /// - Must be unique to prevent conflicts.
    /// - Useful for idempotency and tracking purposes.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.client_order_id("my-custom-order-id-123");
    /// ```
    pub fn client_order_id(mut self, client_order_id: &str) -> Self {
        self.client_order_id = Some(client_order_id.to_string());
        self
    }

    /// Sets whether the order is a preview order. This will skip serializing the `client_order_id`.
    ///
    /// # Arguments
    ///
    /// * `is_preview` - A boolean indicating if it is a preview or not.
    ///
    /// # Note
    ///
    /// - By default, preview is false.
    ///
    /// # Example
    ///
    /// ```rust
    /// builder.preview(true);
    /// ```
    pub fn preview(mut self, is_preview: bool) -> Self {
        self.is_preview = is_preview;
        self
    }

    /// Builds the `CreateOrder` object based on the provided parameters.
    ///
    /// This method validates that all required parameters have been set according to the
    /// specified `order_type` and `time_in_force`. If any required parameters are missing or
    /// invalid, it returns an error.
    ///
    /// # Returns
    ///
    /// * `CbResult<CreateOrder>` - A result containing the `CreateOrder` object if successful,
    ///   or a `CbAdvError` if validation fails.
    ///
    /// # Errors
    ///
    /// Returns `CbAdvError::BadParse` if required parameters are missing or if the combination
    /// of `order_type` and `time_in_force` is unsupported.
    ///
    /// # Example
    ///
    /// ```rust
    /// let create_order = builder.build()?;
    /// ```
    pub fn build(self) -> CbResult<CreateOrder> {
        // Validate required fields based on order type and time-in-force.
        let order_configuration = match (self.order_type, self.time_in_force) {
            (Some(OrderType::Market), Some(TimeInForce::Ioc)) => {
                // Ensure required parameters are set
                if self.base_size.is_none() && self.quote_size.is_none() {
                    return Err(CbAdvError::BadParse(
                        "Either base_size or quote_size must be provided for Market IOC orders"
                            .to_string(),
                    ));
                }

                Ok(OrderConfiguration::MarketIoc(MarketIoc {
                    base_size: self.base_size,
                    quote_size: self.quote_size,
                }))
            }
            (Some(OrderType::Limit), Some(TimeInForce::Gtc)) => {
                let base_size = self.base_size.ok_or_else(|| {
                    CbAdvError::BadParse("base_size is required for Limit GTC orders".to_string())
                })?;
                let limit_price = self.limit_price.ok_or_else(|| {
                    CbAdvError::BadParse("limit_price is required for Limit GTC orders".to_string())
                })?;

                Ok(OrderConfiguration::LimitGtc(LimitGtc {
                    base_size,
                    limit_price,
                    post_only: self.post_only.unwrap_or(false),
                }))
            }
            (Some(OrderType::Limit), Some(TimeInForce::Gtd)) => {
                let base_size = self.base_size.ok_or_else(|| {
                    CbAdvError::BadParse("base_size is required for Limit GTD orders".to_string())
                })?;
                let limit_price = self.limit_price.ok_or_else(|| {
                    CbAdvError::BadParse("limit_price is required for Limit GTD orders".to_string())
                })?;
                let end_time = self.end_time.ok_or_else(|| {
                    CbAdvError::BadParse("end_time is required for Limit GTD orders".to_string())
                })?;

                Ok(OrderConfiguration::LimitGtd(LimitGtd {
                    base_size,
                    limit_price,
                    end_time,
                    post_only: self.post_only.unwrap_or(false),
                }))
            }
            (Some(OrderType::StopLimit), Some(TimeInForce::Gtc)) => {
                let base_size = self.base_size.ok_or_else(|| {
                    CbAdvError::BadParse(
                        "base_size is required for Stop Limit GTC orders".to_string(),
                    )
                })?;
                let limit_price = self.limit_price.ok_or_else(|| {
                    CbAdvError::BadParse(
                        "limit_price is required for Stop Limit GTC orders".to_string(),
                    )
                })?;
                let stop_price = self.stop_price.ok_or_else(|| {
                    CbAdvError::BadParse(
                        "stop_price is required for Stop Limit GTC orders".to_string(),
                    )
                })?;
                let stop_direction = self.stop_direction.ok_or_else(|| {
                    CbAdvError::BadParse(
                        "stop_direction is required for Stop Limit GTC orders".to_string(),
                    )
                })?;

                Ok(OrderConfiguration::StopLimitGtc(StopLimitGtc {
                    base_size,
                    limit_price,
                    stop_price,
                    stop_direction,
                }))
            }
            (Some(OrderType::StopLimit), Some(TimeInForce::Gtd)) => {
                let base_size = self.base_size.ok_or_else(|| {
                    CbAdvError::BadParse(
                        "base_size is required for Stop Limit GTD orders".to_string(),
                    )
                })?;
                let limit_price = self.limit_price.ok_or_else(|| {
                    CbAdvError::BadParse(
                        "limit_price is required for Stop Limit GTD orders".to_string(),
                    )
                })?;
                let stop_price = self.stop_price.ok_or_else(|| {
                    CbAdvError::BadParse(
                        "stop_price is required for Stop Limit GTD orders".to_string(),
                    )
                })?;
                let stop_direction = self.stop_direction.ok_or_else(|| {
                    CbAdvError::BadParse(
                        "stop_direction is required for Stop Limit GTD orders".to_string(),
                    )
                })?;
                let end_time = self.end_time.ok_or_else(|| {
                    CbAdvError::BadParse(
                        "end_time is required for Stop Limit GTD orders".to_string(),
                    )
                })?;

                Ok(OrderConfiguration::StopLimitGtd(StopLimitGtd {
                    base_size,
                    limit_price,
                    stop_price,
                    stop_direction,
                    end_time,
                }))
            }
            _ => Err(CbAdvError::BadParse(
                "Invalid or unsupported combination of order_type and time_in_force".to_string(),
            )),
        }?;

        let client_order_id = if self.is_preview {
            "".to_string()
        } else {
            self.client_order_id
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
        };

        Ok(CreateOrder {
            client_order_id,
            product_id: self.product_id,
            side: self.side.to_string(),
            order_configuration,
        })
    }
}

/// Represents an order created to BUY or SELL.
#[derive(Serialize, Debug)]
pub struct CreateOrder {
    /// Client Order ID (UUID). Skipped if creating a preview order.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub client_order_id: String,
    /// Product ID (pair)
    pub product_id: String,
    /// Order Side: BUY or SELL.
    pub side: String,
    /// Configuration for the order.
    pub order_configuration: OrderConfiguration,
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

/// Represents a create order response from the API.
#[derive(Deserialize, Debug)]
pub struct OrderResponse {
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
pub struct EditOrderPreview {
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

/// Represents the response for a preview of creating an order.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateOrderPreview {
    /// The total value of the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub order_total: f64,
    /// The total commission for the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub commission_total: f64,
    /// List of errors encountered during the preview.
    pub errs: Vec<String>,
    /// List of warnings related to the order preview.
    pub warning: Vec<String>,
    /// The best bid price at the time of the preview.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub best_bid: f64,
    /// The best ask price at the time of the preview.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub best_ask: f64,
    /// The size of the quote currency in the order.
    /// Note: Currently does not appear to work as of 20241119
    // pub quote_size: Option<f64>,
    /// The size of the base currency in the order.
    /// Note: Currently does not appear to work as of 20241119
    // pub base_size: Option<f64>,
    /// Indicates whether the maximum allowed amount was used.
    pub is_max: bool,
    /// The total margin required for the order.
    pub order_margin_total: String,
    /// The leverage applied to the order.
    pub leverage: String,
    /// The long leverage available for the order.
    pub long_leverage: String,
    /// The short leverage available for the order.
    pub short_leverage: String,
    /// The projected slippage for the order.
    pub slippage: String,
    /// The unique identifier for the order preview.
    pub preview_id: String,
    /// The current liquidation buffer for the account.
    pub current_liquidation_buffer: String,
    /// The projected liquidation buffer after the order.
    pub projected_liquidation_buffer: String,
    /// The maximum leverage available for the order.
    pub max_leverage: String,
}

pub struct _Temp {}

/// Represents parameters that are optional for List Orders API request.
#[derive(Serialize, Default, Debug)]
pub struct ListOrdersQuery {
    /// ID(s) of order(s).
    pub order_ids: Option<Vec<String>>,
    /// Optional string of the product ID(s). Defaults to null, or fetch for all products.
    pub product_ids: Option<Vec<String>>,
    /// Only orders matching this product type are returned. Default is to return all product types. Valid options are SPOT or FUTURE.
    pub product_type: Option<String>,
    /// Note: Cannot pair OPEN orders with other order types.
    pub order_status: Option<Vec<OrderStatus>>,
    /// Only orders matching this time in force(s) are returned. Default is to return all time in forces.
    pub time_in_forces: Option<Vec<TimeInForce>>,
    /// Type of orders to return. Default is to return all order types.
    pub order_types: Option<Vec<OrderType>>,
    /// Only orders matching this side are returned. Default is to return all sides.
    pub order_side: Option<OrderSide>,
    /// Start date to fetch orders from, inclusive.
    pub start_date: Option<String>,
    /// An optional end date for the query window, exclusive. If provided only orders with creation time before this date will be returned.
    pub end_date: Option<String>,
    /// Only returns the orders where the quote, base or underlying asset matches the provided asset filter(s) (e.g. 'BTC').
    pub asset_filters: Option<Vec<String>>,
    /// A pagination limit with no default set. If has_next is true, additional orders are available to be fetched with pagination; also the cursor value in the response can be passed as cursor parameter in the subsequent request.
    pub limit: Option<u32>,
    /// Cursor used for pagination. When provided, the response returns responses after this cursor.
    pub cursor: Option<String>,
}

impl Query for ListOrdersQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .with_optional_vec("order_ids", &self.order_ids)
            .with_optional_vec("product_ids", &self.product_ids)
            .push_optional("product_type", &self.product_type)
            .with_optional_vec("order_status", &self.order_status)
            .with_optional_vec("time_in_forces", &self.time_in_forces)
            .with_optional_vec("order_types", &self.order_types)
            .push_optional("order_side", &self.order_side)
            .push_optional("start_date", &self.start_date)
            .push_optional("end_date", &self.end_date)
            .with_optional_vec("asset_filters", &self.asset_filters)
            .push_u32_optional("limit", self.limit)
            .push_optional("cursor", &self.cursor)
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

/// Represents parameters that are needed to close positions.
#[derive(Serialize, Default, Debug)]
pub struct ClosePositionQuery {
    /// The unique ID provided for the order (used for identification purposes).
    pub client_order_id: Option<String>,
    /// The trading pair (e.g. 'BIT-28JUL23-CDE').
    pub product_id: Option<String>,
    /// The amount of contracts that should be closed.
    pub size: Option<u32>,
}

impl Query for ClosePositionQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional("client_order_id", &self.client_order_id)
            .push_optional("product_id", &self.product_id)
            .push_u32_optional("size", self.size)
            .build()
    }
}
