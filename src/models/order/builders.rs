//! # Coinbase Advanced Order API
//!
//! `order/builders` provides a builder pattern for creating `CreateOrder` instances.

use crate::errors::CbError;
use crate::types::CbResult;

use super::{
    LimitGtc, LimitGtd, MarketIoc, OrderConfiguration, OrderCreateRequest, OrderSide, OrderType,
    StopDirection, StopLimitGtc, StopLimitGtd, TimeInForce,
};
use uuid::Uuid;

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
    /// * `side` - The side of the order, either `BUY` or `SELL`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy);
    /// ```
    pub fn new(product_id: &str, side: OrderSide) -> Self {
        Self {
            product_id: product_id.to_string(),
            side,
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
    /// # Arguments
    ///
    /// * `order_type` - An `OrderType` enum variant specifying the type of order.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderType, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .order_type(OrderType::Limit);
    /// ```
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    /// Sets the time-in-force policy for the order.
    ///
    /// # Arguments
    ///
    /// * `tif` - A `TimeInForce` enum variant specifying the time-in-force policy.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, TimeInForce, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .time_in_force(TimeInForce::GoodUntilCancelled);
    /// ```
    pub fn time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Sets the base size for the order.
    ///
    /// # Arguments
    ///
    /// * `base_size` - The quantity of the base currency to trade.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .base_size(0.5);
    /// ```
    pub fn base_size(mut self, base_size: f64) -> Self {
        self.base_size = Some(base_size);
        self
    }

    /// Sets the quote size for the order.
    ///
    /// # Arguments
    ///
    /// * `quote_size` - The amount of the quote currency to use in the order.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .quote_size(1000.0);
    /// ```
    pub fn quote_size(mut self, quote_size: f64) -> Self {
        self.quote_size = Some(quote_size);
        self
    }

    /// Sets the limit price for the order.
    ///
    /// # Arguments
    ///
    /// * `limit_price` - The limit price in terms of the quote currency.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .limit_price(50000.0);
    /// ```
    pub fn limit_price(mut self, limit_price: f64) -> Self {
        self.limit_price = Some(limit_price);
        self
    }

    /// Sets the stop price for the order.
    ///
    /// # Arguments
    ///
    /// * `stop_price` - The price at which the stop order is triggered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .stop_price(48000.0);
    /// ```
    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    /// Sets the stop trigger price for a trigger bracket order.
    ///
    /// # Arguments
    ///
    /// * `stop_trigger_price` - The price level to trigger the exit order.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .stop_trigger_price(47000.0);
    /// ```
    pub fn stop_trigger_price(mut self, stop_trigger_price: f64) -> Self {
        self.stop_trigger_price = Some(stop_trigger_price);
        self
    }

    /// Sets the end time for the order.
    ///
    /// # Arguments
    ///
    /// * `end_time` - The end time as an RFC3339 formatted timestamp.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .end_time("2024-12-31T23:59:59Z");
    /// ```
    pub fn end_time(mut self, end_time: &str) -> Self {
        self.end_time = Some(end_time.to_string());
        self
    }

    /// Sets the post-only flag for the order.
    ///
    /// # Arguments
    ///
    /// * `post_only` - A boolean indicating whether to enable post-only mode.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .post_only(true);
    /// ```
    pub fn post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    /// Sets the stop direction for a stop order.
    ///
    /// # Arguments
    ///
    /// * `stop_direction` - A `StopDirection` enum variant specifying the trigger direction.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide, StopDirection};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .stop_direction(StopDirection::StopUp);
    /// ```
    pub fn stop_direction(mut self, stop_direction: StopDirection) -> Self {
        self.stop_direction = Some(stop_direction);
        self
    }

    /// Sets the client-defined order ID.
    ///
    /// # Arguments
    ///
    /// * `client_order_id` - A string representing the client-defined order ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .client_order_id("my-custom-order-id-123");
    /// ```
    pub fn client_order_id(mut self, client_order_id: &str) -> Self {
        self.client_order_id = Some(client_order_id.to_string());
        self
    }

    /// Sets whether the order is a preview order.
    ///
    /// # Arguments
    ///
    /// * `is_preview` - A boolean indicating if it is a preview or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide};
    /// let builder = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .preview(true);
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
    /// use cbadv::models::order::{OrderCreateBuilder, OrderSide, OrderType, TimeInForce};
    /// let create_order = OrderCreateBuilder::new("BTC-USD", OrderSide::Buy)
    ///     .order_type(OrderType::Limit)
    ///     .time_in_force(TimeInForce::GoodUntilCancelled)
    ///     .base_size(0.5)
    ///     .limit_price(50000.0)
    ///     .build();
    /// ```
    pub fn build(self) -> CbResult<OrderCreateRequest> {
        self.validate_common_fields()?;

        let order_configuration = self.determine_order_configuration()?;

        let client_order_id = if self.is_preview {
            String::new()
        } else {
            self.client_order_id
                .unwrap_or_else(|| Uuid::new_v4().to_string())
        };

        Ok(OrderCreateRequest {
            client_order_id,
            product_id: self.product_id,
            side: self.side,
            is_preview: self.is_preview,
            order_configuration,
        })
    }

    /// Validates common fields applicable to all order types.
    fn validate_common_fields(&self) -> Result<(), CbError> {
        if self.side == OrderSide::Unknown {
            return Err(CbError::BadParse(
                "Order side cannot be unknown.".to_string(),
            ));
        }

        if self.product_id.trim().is_empty() {
            return Err(CbError::BadParse("Product ID cannot be empty.".to_string()));
        }

        if self.order_type.is_none() || self.order_type == Some(OrderType::Unknown) {
            return Err(CbError::BadParse(
                "Order type must be specified.".to_string(),
            ));
        }

        if self.time_in_force.is_none() {
            return Err(CbError::BadParse(
                "Time in force must be specified.".to_string(),
            ));
        }

        Ok(())
    }

    /// Determines and validates the order configuration based on `order_type` and `time_in_force`.
    fn determine_order_configuration(&self) -> Result<OrderConfiguration, CbError> {
        match (self.order_type.as_ref(), self.time_in_force) {
            (Some(order_type), Some(tif)) => order_type.validate_configuration(tif, self),
            _ => Err(CbError::BadParse(
                "Order type and time in force must be specified.".to_string(),
            )),
        }
    }
}

/// Extension trait for `OrderType` to validate and construct `OrderConfiguration`.
trait OrderTypeValidator {
    fn validate_configuration(
        &self,
        tif: TimeInForce,
        builder: &OrderCreateBuilder,
    ) -> Result<OrderConfiguration, CbError>;
}

impl OrderTypeValidator for OrderType {
    fn validate_configuration(
        &self,
        tif: TimeInForce,
        builder: &OrderCreateBuilder,
    ) -> Result<OrderConfiguration, CbError> {
        match (self, tif) {
            (OrderType::Market, TimeInForce::ImmediateOrCancel) => builder.build_market_ioc(),
            (OrderType::Limit, TimeInForce::GoodUntilCancelled) => builder.build_limit_gtc(),
            (OrderType::Limit, TimeInForce::GoodUntilDate) => builder.build_limit_gtd(),
            (OrderType::StopLimit, TimeInForce::GoodUntilCancelled) => {
                builder.build_stop_limit_gtc()
            }
            (OrderType::StopLimit, TimeInForce::GoodUntilDate) => builder.build_stop_limit_gtd(),
            _ => Err(CbError::BadParse(
                "Invalid or unsupported combination of order_type and time_in_force".to_string(),
            )),
        }
    }
}

impl OrderCreateBuilder {
    /// Validates and constructs `MarketIoc` configuration.
    fn build_market_ioc(&self) -> Result<OrderConfiguration, CbError> {
        if self.base_size.is_none() && self.quote_size.is_none() {
            return Err(CbError::BadParse(
                "Either base_size or quote_size must be provided for Market IOC orders".to_string(),
            ));
        }

        Ok(OrderConfiguration::MarketIoc(MarketIoc {
            base_size: self.base_size,
            quote_size: self.quote_size,
        }))
    }

    /// Validates and constructs `LimitGtc` configuration.
    fn build_limit_gtc(&self) -> Result<OrderConfiguration, CbError> {
        let base_size = require_field(self.base_size, "base_size")?;
        let limit_price = require_field(self.limit_price, "limit_price")?;

        Ok(OrderConfiguration::LimitGtc(LimitGtc {
            base_size,
            limit_price,
            post_only: self.post_only.unwrap_or(false),
        }))
    }

    /// Validates and constructs `LimitGtd` configuration.
    fn build_limit_gtd(&self) -> Result<OrderConfiguration, CbError> {
        let base_size = require_field(self.base_size, "base_size")?;
        let limit_price = require_field(self.limit_price, "limit_price")?;
        let end_time = require_field_ref(&self.end_time, "end_time")?;

        Ok(OrderConfiguration::LimitGtd(LimitGtd {
            base_size,
            limit_price,
            end_time: end_time.clone(),
            post_only: self.post_only.unwrap_or(false),
        }))
    }

    /// Validates and constructs `StopLimitGtc` configuration.
    fn build_stop_limit_gtc(&self) -> Result<OrderConfiguration, CbError> {
        let base_size = require_field(self.base_size, "base_size")?;
        let limit_price = require_field(self.limit_price, "limit_price")?;
        let stop_price = require_field(self.stop_price, "stop_price")?;
        let stop_direction = require_field(self.stop_direction, "stop_direction")?;

        Ok(OrderConfiguration::StopLimitGtc(StopLimitGtc {
            base_size,
            limit_price,
            stop_price,
            stop_direction,
        }))
    }

    /// Validates and constructs `StopLimitGtd` configuration.
    fn build_stop_limit_gtd(&self) -> Result<OrderConfiguration, CbError> {
        let base_size = require_field(self.base_size, "base_size")?;
        let limit_price = require_field(self.limit_price, "limit_price")?;
        let stop_price = require_field(self.stop_price, "stop_price")?;
        let stop_direction = require_field(self.stop_direction, "stop_direction")?;
        let end_time = require_field_ref(&self.end_time, "end_time")?;

        Ok(OrderConfiguration::StopLimitGtd(StopLimitGtd {
            base_size,
            limit_price,
            stop_price,
            end_time: end_time.clone(),
            stop_direction,
        }))
    }
}

/// Validates that a required field is present and returns it, or an error if it is missing.
fn require_field<T>(field: Option<T>, field_name: &str) -> Result<T, CbError> {
    field.ok_or_else(|| CbError::BadParse(format!("{field_name} is required.")))
}

/// Validates that a required field reference is present and returns it, or an error if it is missing.
fn require_field_ref<'a, T>(field: &'a Option<T>, field_name: &str) -> Result<&'a T, CbError> {
    field
        .as_ref()
        .ok_or_else(move || CbError::BadParse(format!("{field_name} is required.")))
}
