//! # Coinbase Advanced Order API
//!
//! `order` gives access to the Order API and the various endpoints associated with it.
//! These allow you to obtain past created orders, create new orders, and cancel orders.

use crate::signer::Signer;
use crate::utils::{CBAdvError, Result};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, DisplayFromStr};
use std::fmt;
use uuid::Uuid;

/// Various order types.
#[derive(Serialize, Deserialize, Debug)]
pub enum OrderType {
    /// A Market order.
    MARKET,
    /// A Limit order.
    LIMIT,
    /// A stop order is an order that becomes a market order when triggered.
    STOP,
    /// A stop order is a limit order that doesn't go on the book until it hits the stop price.
    STOPLIMIT,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderType::MARKET => write!(f, "MARKET"),
            OrderType::LIMIT => write!(f, "LIMIT"),
            OrderType::STOP => write!(f, "STOP"),
            OrderType::STOPLIMIT => write!(f, "STOPLIMIT"),
        }
    }
}

/// Order side, BUY or SELL.
#[derive(Serialize, Deserialize, Debug)]
pub enum OrderSide {
    /// Buying a product.
    BUY,
    /// Selling a product.
    SELL,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderSide::BUY => write!(f, "BUY"),
            OrderSide::SELL => write!(f, "SELL"),
        }
    }
}

/// Order status, OPEN, CANCELLED, and EXPIRED.
#[derive(Serialize, Deserialize, Debug)]
pub enum OrderStatus {
    /// Implies the order is still available and not closed.
    OPEN,
    /// Order was closed by cancellation.
    CANCELLED,
    /// Order was closed by expiration.
    EXPIRED,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderStatus::OPEN => write!(f, "OPEN"),
            OrderStatus::CANCELLED => write!(f, "CANCELLED"),
            OrderStatus::EXPIRED => write!(f, "EXPIRED"),
        }
    }
}

/// Order updates for a user from a websocket.
#[serde_as]
#[derive(Deserialize, Debug)]
pub struct OrderUpdate {
    /// Type of the update.
    pub r#type: String,
    /// Client Order ID (Normally a UUID)
    pub client_order_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub cumulative_quantity: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub leaves_quantity: f64,
    /// Average price for the order.
    #[serde_as(as = "DisplayFromStr")]
    pub avg_price: f64,
    /// Total fees for the order.
    #[serde_as(as = "DisplayFromStr")]
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
struct MarketIOC {
    /// Amount of quote currency to spend on order. Required for BUY orders.
    pub quote_size: Option<String>,
    /// Amount of base currency to spend on order. Required for SELL orders.
    pub base_size: Option<String>,
}

/// Limit Good til Cancelled.
#[derive(Serialize, Debug)]
struct LimitGTC {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// Post only limit order.
    pub post_only: bool,
}

/// Limit Good til Time (Date).
#[derive(Serialize, Debug)]
struct LimitGTD {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// Time at which the order should be cancelled if it's not filled.
    pub end_time: String,
    /// Post only limit order.
    pub post_only: bool,
}

/// Stop Limit Good til Cancelled.
#[derive(Serialize, Debug)]
struct StopLimitGTC {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// Price at which the order should trigger - if stop direction is Up, then the order will trigger when the last trade price goes above this, otherwise order will trigger when last trade price goes below this price.
    pub stop_price: String,
    /// Possible values: [UNKNOWN_STOP_DIRECTION, STOP_DIRECTION_STOP_UP, STOP_DIRECTION_STOP_DOWN]
    pub stop_direction: String,
}

/// Stop Limit Good til Time (Date).
#[derive(Serialize, Debug)]
struct StopLimitGTD {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// Price at which the order should trigger - if stop direction is Up, then the order will trigger when the last trade price goes above this, otherwise order will trigger when last trade price goes below this price.
    pub stop_price: String,
    /// Time at which the order should be cancelled if it's not filled.
    pub end_time: String,
    /// Possible values: [UNKNOWN_STOP_DIRECTION, STOP_DIRECTION_STOP_UP, STOP_DIRECTION_STOP_DOWN]
    pub stop_direction: String,
}

/// Create Order Configuration.
#[derive(Serialize, Default, Debug)]
struct OrderConfiguration {
    /// Market Order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_market_ioc: Option<MarketIOC>,
    /// Limit Order, Good til Cancelled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_limit_gtc: Option<LimitGTC>,
    /// Limit Order, Good til Date (time)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_limit_gtd: Option<LimitGTD>,
    /// Stop Limit Order, Good til Cancelled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_limit_stop_limit_gtc: Option<StopLimitGTC>,
    /// Stop Limit Order, Good til Date (time)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_limit_stop_limit_gtd: Option<StopLimitGTD>,
}

/// Represents an order created to BUY or SELL.
#[derive(Serialize, Debug)]
struct CreateOrder {
    /// Client Order ID (UUID)
    pub client_order_id: String,
    /// Product ID (pair)
    pub product_id: String,
    /// Order Side: BUY or SELL.
    pub side: String,
    /// Configuration for the order.
    pub order_configuration: OrderConfiguration,
}

/// Represents a vector of orders IDs to cancel.
#[derive(Serialize, Debug)]
struct CancelOrders {
    /// Vector of Order IDs to cancel.
    pub order_ids: Vec<String>,
}

/// Represents an Order received from the API.
#[serde_as]
#[derive(Deserialize, Debug)]
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
    #[serde_as(as = "DisplayFromStr")]
    pub completion_percentage: f64,
    /// The portion (in base currency) of total order amount that has been filled.
    #[serde_as(as = "DisplayFromStr")]
    pub filled_size: f64,
    /// The average of all prices of fills for this order.
    #[serde_as(as = "DisplayFromStr")]
    pub average_filled_price: f64,
    /// Commission amount.
    #[serde_as(as = "DefaultOnError")]
    pub fee: f64,
    /// Number of fills that have been posted for this order.
    #[serde_as(as = "DisplayFromStr")]
    pub number_of_fills: u32,
    /// The portion (in quote current) of total order amount that has been filled.
    #[serde_as(as = "DisplayFromStr")]
    pub filled_value: f64,
    /// Whether a cancel request has been initiated for the order, and not yet completed.
    pub pending_cancel: bool,
    /// Whether the order was placed with quote currency/
    pub size_in_quote: bool,
    /// The total fees for the order.
    #[serde_as(as = "DisplayFromStr")]
    pub total_fees: f64,
    /// Whether the order size includes fees.
    pub size_inclusive_of_fees: bool,
    /// Derived field: filled_value + total_fees for buy orders and filled_value - total_fees for sell orders.
    #[serde_as(as = "DisplayFromStr")]
    pub total_value_after_fees: f64,
    /// Possible values: [UNKNOWN_TRIGGER_STATUS, INVALID_ORDER_TYPE, STOP_PENDING, STOP_TRIGGERED]
    pub trigger_status: String,
    /// Possible values: [UNKNOWN_ORDER_TYPE, MARKET, LIMIT, STOP, STOP_LIMIT]
    pub order_type: String,
    /// Possible values: [REJECT_REASON_UNSPECIFIED]
    pub reject_reason: String,
    /// True if the order is fully filled, false otherwise.
    pub settled: bool,
    /// Possible values: [SPOT, FUTURE]
    pub product_type: String,
    /// Message stating why the order was rejected.
    pub reject_message: String,
    /// Message stating why the order was canceled.
    pub cancel_message: String,
}

/// Represents a fill received from the API.
#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Fill {
    pub entry_id: String,
    pub trade_id: String,
    pub order_id: String,
    pub trade_time: String,
    pub trade_type: String,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    #[serde_as(as = "DefaultOnError")]
    pub commission: f64,
    pub product_id: String,
    pub sequence_timestamp: String,
    pub liquidity_indicator: String,
    pub size_in_quote: bool,
    pub user_id: String,
    pub side: String,
}

/// Represents a list of orders received from the API.
#[derive(Deserialize, Debug)]
pub struct ListedOrders {
    pub orders: Vec<Order>,
    pub has_next: bool,
    pub cursor: String,
}

/// Represents a list of fills received from the API.
#[derive(Deserialize, Debug)]
pub struct ListedFills {
    pub orders: Vec<Fill>,
    pub cursor: String,
}

/// Represents a create order response from the API.
#[derive(Deserialize, Debug)]
pub struct OrderResponse {
    pub success: bool,
    pub failure_reason: String,
    pub order_id: String,
}

/// Represents a cancel order response from the API.
#[derive(Deserialize, Debug)]
pub struct CancelOrdersResponse {
    results: Vec<OrderResponse>,
}

/// Represents an order when obtaining a single order from the API.
#[derive(Deserialize, Debug)]
struct OrderStatusResponse {
    pub order: Order,
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

impl fmt::Display for ListOrdersQuery {
    /// Converts the object into HTTP request parameters.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut query: String = "".to_string();

        query = match &self.product_id {
            Some(v) => format!("{}&product_id={}", query, v),
            _ => query,
        };

        query = match &self.order_status {
            Some(v) => {
                let statuses: String = v.iter().map(|s| format!("&order_status={s}")).collect();
                format!("{}{}", query, statuses)
            }
            _ => query,
        };

        query = match &self.limit {
            Some(v) => format!("{}&limit={}", query, v),
            _ => query,
        };

        query = match &self.start_date {
            Some(v) => format!("{}&start_date={}", query, v),
            _ => query,
        };

        query = match &self.end_date {
            Some(v) => format!("{}&end_date={}", query, v),
            _ => query,
        };

        query = match &self.order_type {
            Some(v) => format!("{}&order_type={}", query, v),
            _ => query,
        };

        query = match &self.order_side {
            Some(v) => format!("{}&order_side={}", query, v),
            _ => query,
        };

        query = match &self.cursor {
            Some(v) => format!("{}&cursor={}", query, v),
            _ => query,
        };

        query = match &self.product_type {
            Some(v) => format!("{}&product_type={}", query, v),
            _ => query,
        };

        match query.is_empty() {
            true => write!(f, ""),
            false => write!(f, "{}", query[1..].to_string()),
        }
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

impl fmt::Display for ListFillsQuery {
    /// Converts the object into HTTP request parameters.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut query: String = "".to_string();

        query = match &self.order_id {
            Some(v) => format!("{}&order_id={}", query, v),
            _ => query,
        };

        query = match &self.product_id {
            Some(v) => format!("{}&product_id={}", query, v),
            _ => query,
        };

        query = match &self.start_sequence_timestamp {
            Some(v) => format!("{}&start_sequence_timestamp={}", query, v),
            _ => query,
        };

        query = match &self.end_sequence_timestamp {
            Some(v) => format!("{}&end_sequence_timestamp={}", query, v),
            _ => query,
        };

        query = match &self.limit {
            Some(v) => format!("{}&limit={}", query, v),
            _ => query,
        };

        query = match &self.cursor {
            Some(v) => format!("{}&cursor={}", query, v),
            _ => query,
        };

        match query.is_empty() {
            true => write!(f, ""),
            false => write!(f, "{}", query[1..].to_string()),
        }
    }
}

/// Provides access to the Order API for the service.
pub struct OrderAPI {
    /// Object used to sign requests made to the API.
    signer: Signer,
}

impl OrderAPI {
    /// Resource for the API.
    const RESOURCE: &str = "/api/v3/brokerage/orders";

    /// Creates a new instance of the Order API. This grants access to order information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub(crate) fn new(signer: Signer) -> Self {
        Self { signer }
    }

    /// Cancel orders.
    ///
    /// # Arguments
    ///
    /// * `order_ids` - A vector of strings that represents order IDs to cancel.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/batch_cancel
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_cancelorders>
    pub async fn cancel(&self, order_ids: &Vec<String>) -> Result<Vec<OrderResponse>> {
        let body = CancelOrders {
            order_ids: order_ids.clone(),
        };

        let resource = format!("{}/batch_cancel", Self::RESOURCE);
        match self.signer.post(&resource, "", body).await {
            Ok(value) => match value.json::<CancelOrdersResponse>().await {
                Ok(resp) => Ok(resp.results),
                Err(_) => Err(CBAdvError::BadParse("cancel order object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Cancel all OPEN orders for a specific product ID.
    ///
    /// NOTE: NOT A STANDARD API FUNCTION. QoL function that may require additional API requests
    /// than normal.
    ///
    /// # Arguments
    ///
    /// * `product_id` - Product to cancel all OPEN orders for.
    pub async fn cancel_all(&self, product_id: &str) -> Result<Vec<OrderResponse>> {
        let query = ListOrdersQuery {
            product_id: Some(product_id.to_string()),
            order_status: Some(vec![OrderStatus::OPEN]),
            ..Default::default()
        };

        // Obtain all open orders.
        match self.get_all(product_id, Some(query)).await {
            Ok(orders) => {
                // Build list of orders to cancel.
                let order_ids: Vec<String> = orders.iter().map(|o| o.order_id.clone()).collect();

                // Do nothing since no orders found.
                if order_ids.len() == 0 {
                    return Err(CBAdvError::NothingToDo(
                        "no orders found to cancel".to_string(),
                    ));
                }

                // Cancel the order list.
                self.cancel(&order_ids).await
            }
            Err(error) => Err(error),
        }
    }

    /// Create an order.
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string that represents the product's ID.
    /// * `side` - A string that represents the side: BUY or SELL
    /// * `configuration` - A OrderConfiguration containing details on type of order.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder>
    async fn create(
        &self,
        product_id: &str,
        side: &str,
        configuration: OrderConfiguration,
    ) -> Result<OrderResponse> {
        let body = CreateOrder {
            client_order_id: Uuid::new_v4().to_string(),
            product_id: product_id.to_string(),
            side: side.to_string(),
            order_configuration: configuration,
        };

        match self.signer.post(Self::RESOURCE, "", body).await {
            Ok(value) => match value.json::<OrderResponse>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse("created order object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Create a market order.
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string that represents the product's ID.
    /// * `side` - A string that represents the side: BUY or SELL
    /// * `size` - A 64-bit float that represents the size to buy or sell.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder>
    pub async fn create_market(
        &self,
        product_id: &str,
        side: &str,
        size: &f64,
    ) -> Result<OrderResponse> {
        let market = if side == "BUY" {
            MarketIOC {
                quote_size: Some(size.to_string()),
                base_size: None,
            }
        } else {
            MarketIOC {
                quote_size: None,
                base_size: Some(size.to_string()),
            }
        };

        let config = OrderConfiguration {
            market_market_ioc: Some(market),
            ..Default::default()
        };

        self.create(product_id, side, config).await
    }

    /// Create a Good til Cancelled Limit order.
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string that represents the product's ID.
    /// * `side` - A string that represents the side: BUY or SELL
    /// * `size` - A 64-bit float that represents the size to buy or sell.
    /// * `price` - A 64-bit float that represents the price to buy or sell.
    /// * `post_only` - A boolean that represents MAKER or TAKER.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder>
    pub async fn create_limit_gtc(
        &self,
        product_id: &str,
        side: &str,
        size: &f64,
        price: &f64,
        post_only: bool,
    ) -> Result<OrderResponse> {
        let limit = LimitGTC {
            base_size: size.to_string(),
            limit_price: price.to_string(),
            post_only: post_only.clone(),
        };

        let config = OrderConfiguration {
            limit_limit_gtc: Some(limit),
            ..Default::default()
        };

        self.create(product_id, side, config).await
    }

    /// Create a Good til Time (Date) Limit order.
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string that represents the product's ID.
    /// * `side` - A string that represents the side: BUY or SELL
    /// * `size` - A 64-bit float that represents the size to buy or sell.
    /// * `price` - A 64-bit float that represents the price to buy or sell.
    /// * `end_time` - A string that represents the time to kill the order.
    /// * `post_only` - A boolean that represents MAKER or TAKER.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder>
    pub async fn create_limit_gtd(
        &self,
        product_id: &str,
        side: &str,
        size: &f64,
        price: &f64,
        end_time: &str,
        post_only: bool,
    ) -> Result<OrderResponse> {
        let limit = LimitGTD {
            base_size: size.to_string(),
            limit_price: price.to_string(),
            end_time: end_time.to_string(),
            post_only: post_only.clone(),
        };

        let config = OrderConfiguration {
            limit_limit_gtd: Some(limit),
            ..Default::default()
        };

        self.create(product_id, side, config).await
    }

    /// Create a Good til Cancelled Stop Limit order.
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string that represents the product's ID.
    /// * `side` - A string that represents the side: BUY or SELL
    /// * `size` - A 64-bit float that represents the size to buy or sell.
    /// * `limit_price` - Ceiling price for which the order should get filled.
    /// * `stop_price` - Price at which the order should trigger - if stop direction is Up, then the order will trigger when the last trade price goes above this, otherwise order will trigger when last trade price goes below this price.
    /// * `stop_direction` - Possible values: [UNKNOWN_STOP_DIRECTION, STOP_DIRECTION_STOP_UP, STOP_DIRECTION_STOP_DOWN]
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder>
    pub async fn create_stop_limit_gtc(
        &self,
        product_id: &str,
        side: &str,
        size: &f64,
        limit_price: &f64,
        stop_price: &f64,
        stop_direction: &str,
    ) -> Result<OrderResponse> {
        let stoplimit = StopLimitGTC {
            base_size: size.to_string(),
            limit_price: limit_price.to_string(),
            stop_price: stop_price.to_string(),
            stop_direction: stop_direction.to_string(),
        };

        let config = OrderConfiguration {
            stop_limit_stop_limit_gtc: Some(stoplimit),
            ..Default::default()
        };

        self.create(product_id, side, config).await
    }

    /// Create a Good til Time (Date) Stop Limit order.
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string that represents the product's ID.
    /// * `side` - A string that represents the side: BUY or SELL
    /// * `size` - A 64-bit float that represents the size to buy or sell.
    /// * `limit_price` - Ceiling price for which the order should get filled.
    /// * `stop_price` - Price at which the order should trigger - if stop direction is Up, then the order will trigger when the last trade price goes above this, otherwise order will trigger when last trade price goes below this price.
    /// * `stop_direction` - Possible values: [UNKNOWN_STOP_DIRECTION, STOP_DIRECTION_STOP_UP, STOP_DIRECTION_STOP_DOWN]
    /// * `end_time` - Time at which the order should be cancelled if it's not filled.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder>
    pub async fn create_stop_limit_gtd(
        &self,
        product_id: &str,
        side: &str,
        size: &f64,
        limit_price: &f64,
        stop_price: &f64,
        stop_direction: &str,
        end_time: &str,
    ) -> Result<OrderResponse> {
        let stoplimit = StopLimitGTD {
            base_size: size.to_string(),
            limit_price: limit_price.to_string(),
            stop_price: stop_price.to_string(),
            end_time: end_time.to_string(),
            stop_direction: stop_direction.to_string(),
        };

        let config = OrderConfiguration {
            stop_limit_stop_limit_gtd: Some(stoplimit),
            ..Default::default()
        };

        self.create(product_id, side, config).await
    }

    /// Obtains a single order based on the Order ID (ex. "XXXX-YYYY-ZZZZ").
    ///
    /// # Arguments
    ///
    /// * `order_id` - A string that represents the order's ID.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/historical/{order_id}
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gethistoricalorder>
    pub async fn get(&self, order_id: &str) -> Result<Order> {
        let resource = format!("{}/historical/{}", Self::RESOURCE, order_id);
        match self.signer.get(&resource, "").await {
            Ok(value) => match value.json::<OrderStatusResponse>().await {
                Ok(resp) => Ok(resp.order),
                Err(_) => Err(CBAdvError::BadParse(
                    "could not parse order object".to_string(),
                )),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains various orders from the API.
    ///
    /// * `query` - A Parameters to modify what is returned by the API.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/historical
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gethistoricalorders>
    pub async fn get_bulk(&self, query: &ListOrdersQuery) -> Result<ListedOrders> {
        let resource = format!("{}/historical/batch", Self::RESOURCE);
        match self.signer.get(&resource, &query.to_string()).await {
            Ok(value) => match value.json::<ListedOrders>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse(
                    "could not parse orders vector".to_string(),
                )),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains all orders for a product based on the product ID. (ex. "BTC-USD").
    /// This wraps `get_bulk` and makes several additional requests until there are no
    /// additional orders.
    ///
    /// NOTE: NOT A STANDARD API FUNCTION. QoL function that may require additional API requests than
    /// normal.
    ///
    /// # Arguments
    ///
    /// * `product_id` - Identifier for the account, such as BTC-USD or ETH-USD.
    /// * `query` - Optional parameters, should default to None unless you want additional control.
    pub async fn get_all(
        &self,
        product_id: &str,
        query: Option<ListOrdersQuery>,
    ) -> Result<Vec<Order>> {
        let mut query = match query {
            Some(p) => p,
            None => ListOrdersQuery::default(),
        };

        // Override product ID.
        query.product_id = Some(product_id.to_string());
        let mut orders: Vec<Order> = vec![];
        let mut has_next: bool = true;

        // Get the orders until there is not a next.
        while has_next {
            match self.get_bulk(&query).await {
                Ok(listed) => {
                    has_next = listed.has_next;
                    query.cursor = Some(listed.cursor);
                    orders.extend(listed.orders);
                }
                Err(error) => return Err(error),
            }
        }

        Ok(orders)
    }

    /// Obtains fills from the API.
    ///
    /// * `query` - A Parameters to modify what is returned by the API.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/historical/fills
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getfills>
    pub async fn fills(&self, query: &ListFillsQuery) -> Result<ListedFills> {
        let resource = format!("{}/historical/fills", Self::RESOURCE);
        match self.signer.get(&resource, &query.to_string()).await {
            Ok(value) => match value.json::<ListedFills>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse(
                    "could not parse fills vector".to_string(),
                )),
            },
            Err(error) => Err(error),
        }
    }
}
