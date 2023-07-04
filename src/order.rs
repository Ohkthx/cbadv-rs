//! # Coinbase Advanced Order API
//!
//! `order` gives access to the Order API and the various endpoints associated with it.
//! These allow you to obtain past created orders, create new orders, and cancel orders.

use crate::utils::{CBAdvError, Result, Signer};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Various order types.
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum OrderStatus {
    OPEN,
    CANCELLED,
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
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderUpdate {
    pub r#type: String,
    pub client_order_id: String,
    pub cumulative_quantity: String,
    pub leaves_quantity: String,
    pub avg_price: String,
    pub total_fees: String,
    pub status: String,
    pub product_id: String,
    pub creation_time: String,
    pub order_side: String,
    pub order_type: String,
}

/// Market Immediate or Cancel.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct MarketIOC {
    /// Amount of quote currency to spend on order. Required for BUY orders.
    pub quote_size: Option<String>,
    /// Amount of base currency to spend on order. Required for SELL orders.
    pub base_size: Option<String>,
}

/// Limit Good til Cancelled.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct LimitGTC {
    /// Amount of base currency to spend on order.
    pub base_size: String,
    /// Ceiling price for which the order should get filled.
    pub limit_price: String,
    /// Post only limit order.
    pub post_only: bool,
}

/// Limit Good til Time (Date).
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
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
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
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
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
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
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Debug)]
struct OrderConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_market_ioc: Option<MarketIOC>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_limit_gtc: Option<LimitGTC>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_limit_gtd: Option<LimitGTD>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_limit_stop_limit_gtc: Option<StopLimitGTC>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_limit_stop_limit_gtd: Option<StopLimitGTD>,
}

/// Represents an order created to BUY or SELL.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct CreateOrder {
    pub client_order_id: String,
    pub product_id: String,
    pub side: String,
    pub order_configuration: OrderConfiguration,
}

/// Represents a vector of orders IDs to cancel.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct CancelOrders {
    pub order_ids: Vec<String>,
}

/// Represents an Order received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub order_id: String,
    pub client_order_id: String,
    pub product_id: String,
    pub user_id: String,

    pub side: String,
    pub status: String,
    pub time_in_force: String,
    pub created_time: String,

    pub completion_percentage: String,
    pub filled_size: String,
    pub average_filled_price: String,
    pub fee: String,
    pub number_of_fills: String,
    pub filled_value: String,

    pub pending_cancel: bool,
    pub size_in_quote: bool,

    pub total_fees: String,
    pub size_inclusive_of_fees: bool,
    pub total_value_after_fees: String,

    pub trigger_status: String,
    pub order_type: String,
    pub reject_reason: String,
    pub settled: bool,
    pub product_type: String,
    pub reject_message: String,
    pub cancel_message: String,
}

/// Represents a fill received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Fill {
    pub entry_id: String,
    pub trade_id: String,
    pub order_id: String,
    pub trade_time: String,
    pub trade_type: String,
    pub price: String,
    pub size: String,
    pub commission: String,
    pub product_id: String,
    pub sequence_timestamp: String,
    pub liquidity_indicator: String,
    pub size_in_quote: bool,
    pub user_id: String,
    pub side: String,
}

/// Represents a list of orders received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ListedOrders {
    pub orders: Vec<Order>,
    pub has_next: bool,
    pub cursor: String,
}

/// Represents a list of fills received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ListedFills {
    pub orders: Vec<Fill>,
    pub cursor: String,
}

/// Represents a create order response from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderResponse {
    pub success: bool,
    pub failure_reason: String,
    pub order_id: String,
}

/// Represents a cancel order response from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CancelOrdersResponse {
    results: Vec<OrderResponse>,
}

/// Represents an order when obtaining a single order from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct OrderStatusResponse {
    pub order: Order,
}

/// Represents parameters that are optional for List Orders API request.
#[allow(dead_code)]
#[derive(Serialize, Default, Debug)]
pub struct ListOrdersQuery {
    /// Optional string of the product ID. Defaults to null, or fetch for all products.
    pub product_id: Option<String>,
    /// Note: Cannot pair OPEN orders with other order types.
    pub order_status: Option<Vec<OrderStatus>>,
    /// A pagination limit with no default set. If has_next is true, additional orders are available to be fetched with pagination; also the cursor value in the response can be passed as cursor parameter in the subsequent request.
    pub limit: Option<i32>,
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
#[allow(dead_code)]
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
    pub limit: Option<i32>,
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
    pub fn new(signer: Signer) -> Self {
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
    /// * `size` - A string that represents the size to buy or sell.
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
        size: &str,
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
    /// * `size` - A string that represents the size to buy or sell.
    /// * `price` - A string that represents the price to buy or sell.
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
        size: &str,
        price: &str,
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
    /// * `size` - A string that represents the size to buy or sell.
    /// * `price` - A string that represents the price to buy or sell.
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
        size: &str,
        price: &str,
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
    /// * `size` - A string that represents the size to buy or sell.
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
        size: &str,
        limit_price: &str,
        stop_price: &str,
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
    /// * `size` - A string that represents the size to buy or sell.
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
        size: &str,
        limit_price: &str,
        stop_price: &str,
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
