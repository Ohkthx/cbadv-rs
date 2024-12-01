//! # Coinbase Advanced Order API
//!
//! `order/builders` provides a builder pattern for creating `CreateOrder` instances.

use crate::errors::CbError;
use crate::types::CbResult;

use super::{
    LimitGtc, LimitGtd, MarketIoc, OrderConfiguration, OrderCreateRequest, OrderSide, OrderType,
    StopDirection, StopLimitGtc, StopLimitGtd, TimeInForce,
};

/// A builder for creating `OrderCreateRequest` instances.
///
/// This builder provides a fluent interface to construct an order by specifying the product,
/// side, order type, time-in-force, and other optional parameters. It ensures that all required
/// parameters are set before building the final order, and helps prevent invalid configurations.
pub struct OrderCreateBuilder {
    product_id: String,
    side: OrderSide,
    is_preview: bool,
    order_type: Option<OrderType>,
    time_in_force: Option<TimeInForce>,
    base_size: Option<f64>,
    quote_size: Option<f64>,
    limit_price: Option<f64>,
    stop_price: Option<f64>,
    stop_trigger_price: Option<f64>,
    end_time: Option<String>,
    post_only: Option<bool>,
    stop_direction: Option<StopDirection>,
    client_order_id: Option<String>,
}

impl OrderCreateBuilder {
    /// Creates a new `OrderCreateBuilder` instance.
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
    /// let builder = OrderCreateBuilder::new("BTC-USD", &OrderSide::Buy);
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
        self.base_size = Some(base_size);
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
        self.quote_size = Some(quote_size);
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
        self.limit_price = Some(limit_price);
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
        self.stop_price = Some(stop_price);
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
        self.stop_trigger_price = Some(stop_trigger_price);
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

    /// Builds the `OrderCreateRequest` object based on the provided parameters.
    ///
    /// This method validates that all required parameters have been set according to the
    /// specified `order_type` and `time_in_force`. If any required parameters are missing or
    /// invalid, it returns an error.
    ///
    /// # Returns
    ///
    /// * `CbResult<OrderCreateRequest>` - A result containing the `CreateOrder` object if successful,
    ///   or a `CbError` if validation fails.
    ///
    /// # Errors
    ///
    /// Returns `CbError::BadParse` if required parameters are missing or if the combination
    /// of `order_type` and `time_in_force` is unsupported.
    ///
    /// # Example
    ///
    /// ```rust
    /// let create_order = builder.build()?;
    /// ```
    pub fn build(self) -> CbResult<OrderCreateRequest> {
        if self.side == OrderSide::Unknown {
            return Err(CbError::BadParse(
                "Order side cannot be unknown.".to_string(),
            ));
        }

        // Validate required fields based on order type and time-in-force.
        let order_configuration = match (self.order_type, self.time_in_force) {
            (Some(OrderType::Market), Some(TimeInForce::ImmediateOrCancel)) => {
                // Ensure required parameters are set
                if self.base_size.is_none() && self.quote_size.is_none() {
                    return Err(CbError::BadParse(
                        "Either base_size or quote_size must be provided for Market IOC orders"
                            .to_string(),
                    ));
                }

                Ok(OrderConfiguration::MarketIoc(MarketIoc {
                    base_size: self.base_size,
                    quote_size: self.quote_size,
                }))
            }
            (Some(OrderType::Limit), Some(TimeInForce::GoodUntilCancelled)) => {
                let base_size = self.base_size.ok_or_else(|| {
                    CbError::BadParse("base_size is required for Limit GTC orders".to_string())
                })?;
                let limit_price = self.limit_price.ok_or_else(|| {
                    CbError::BadParse("limit_price is required for Limit GTC orders".to_string())
                })?;

                Ok(OrderConfiguration::LimitGtc(LimitGtc {
                    base_size,
                    limit_price,
                    post_only: self.post_only.unwrap_or(false),
                }))
            }
            (Some(OrderType::Limit), Some(TimeInForce::GoodUntilDate)) => {
                let base_size = self.base_size.ok_or_else(|| {
                    CbError::BadParse("base_size is required for Limit GTD orders".to_string())
                })?;
                let limit_price = self.limit_price.ok_or_else(|| {
                    CbError::BadParse("limit_price is required for Limit GTD orders".to_string())
                })?;
                let end_time = self.end_time.ok_or_else(|| {
                    CbError::BadParse("end_time is required for Limit GTD orders".to_string())
                })?;

                Ok(OrderConfiguration::LimitGtd(LimitGtd {
                    base_size,
                    limit_price,
                    end_time,
                    post_only: self.post_only.unwrap_or(false),
                }))
            }
            (Some(OrderType::StopLimit), Some(TimeInForce::GoodUntilCancelled)) => {
                let base_size = self.base_size.ok_or_else(|| {
                    CbError::BadParse("base_size is required for Stop Limit GTC orders".to_string())
                })?;
                let limit_price = self.limit_price.ok_or_else(|| {
                    CbError::BadParse(
                        "limit_price is required for Stop Limit GTC orders".to_string(),
                    )
                })?;
                let stop_price = self.stop_price.ok_or_else(|| {
                    CbError::BadParse(
                        "stop_price is required for Stop Limit GTC orders".to_string(),
                    )
                })?;
                let stop_direction = self.stop_direction.ok_or_else(|| {
                    CbError::BadParse(
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
            (Some(OrderType::StopLimit), Some(TimeInForce::GoodUntilDate)) => {
                let base_size = self.base_size.ok_or_else(|| {
                    CbError::BadParse("base_size is required for Stop Limit GTD orders".to_string())
                })?;
                let limit_price = self.limit_price.ok_or_else(|| {
                    CbError::BadParse(
                        "limit_price is required for Stop Limit GTD orders".to_string(),
                    )
                })?;
                let stop_price = self.stop_price.ok_or_else(|| {
                    CbError::BadParse(
                        "stop_price is required for Stop Limit GTD orders".to_string(),
                    )
                })?;
                let stop_direction = self.stop_direction.ok_or_else(|| {
                    CbError::BadParse(
                        "stop_direction is required for Stop Limit GTD orders".to_string(),
                    )
                })?;
                let end_time = self.end_time.ok_or_else(|| {
                    CbError::BadParse("end_time is required for Stop Limit GTD orders".to_string())
                })?;

                Ok(OrderConfiguration::StopLimitGtd(StopLimitGtd {
                    base_size,
                    limit_price,
                    stop_price,
                    stop_direction,
                    end_time,
                }))
            }
            _ => Err(CbError::BadParse(
                "Invalid or unsupported combination of order_type and time_in_force".to_string(),
            )),
        }?;

        let client_order_id = if self.is_preview {
            "".to_string()
        } else {
            self.client_order_id
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
        };

        Ok(OrderCreateRequest {
            client_order_id,
            product_id: self.product_id,
            side: self.side,
            is_preview: self.is_preview,
            order_configuration,
        })
    }
}
