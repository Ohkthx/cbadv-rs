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

/// Represents a list of orders received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ListOrders {
    pub orders: Vec<Order>,
    pub has_next: bool,
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

/// Represents parameters that are optional for List Account API request.
#[allow(dead_code)]
#[derive(Serialize, Default, Debug)]
pub struct ListOrdersParams {
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

impl ListOrdersParams {
    /// Converts the object into HTTP request parameters.
    pub fn to_params(&self) -> String {
        let mut params: String = "".to_string();

        params = match &self.product_id {
            Some(v) => format!("{}&product_id={}", params, v),
            _ => params,
        };

        params = match &self.order_status {
            Some(v) => {
                let statuses: String = v.iter().map(|s| s.to_string() + ",").collect();
                format!("{}&order_status={}", params, statuses)
            }
            _ => params,
        };

        params = match &self.limit {
            Some(v) => format!("{}&limit={}", params, v),
            _ => params,
        };

        params = match &self.start_date {
            Some(v) => format!("{}&start_date={}", params, v),
            _ => params,
        };

        params = match &self.end_date {
            Some(v) => format!("{}&end_date={}", params, v),
            _ => params,
        };

        params = match &self.order_type {
            Some(v) => format!("{}&order_type={}", params, v),
            _ => params,
        };

        params = match &self.order_side {
            Some(v) => format!("{}&order_side={}", params, v),
            _ => params,
        };

        params = match &self.cursor {
            Some(v) => format!("{}&cursor={}", params, v),
            _ => params,
        };

        params = match &self.product_type {
            Some(v) => format!("{}&product_type={}", params, v),
            _ => params,
        };

        match params.is_empty() {
            true => params,
            false => params[1..].to_string(),
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
    /// https://api.coinbase.com/api/v3/brokerage/orders/batch_cancel
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_cancelorders
    pub async fn cancel(&self, order_ids: Vec<String>) -> Result<Vec<OrderResponse>> {
        let body = CancelOrders { order_ids };

        let resource = format!("{}/batch_cancel", Self::RESOURCE.to_string());
        match self.signer.post(resource, "".to_string(), body).await {
            Ok(value) => match value.json::<CancelOrdersResponse>().await {
                Ok(resp) => Ok(resp.results),
                Err(_) => Err(CBAdvError::BadParse("cancel order object".to_string())),
            },
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
    /// https://api.coinbase.com/api/v3/brokerage/orders
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder
    async fn create(
        &self,
        product_id: &String,
        side: &String,
        configuration: OrderConfiguration,
    ) -> Result<OrderResponse> {
        let body = CreateOrder {
            client_order_id: Uuid::new_v4().to_string(),
            product_id: product_id.clone(),
            side: side.clone(),
            order_configuration: configuration,
        };

        match self
            .signer
            .post(Self::RESOURCE.to_string(), "".to_string(), body)
            .await
        {
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
    /// https://api.coinbase.com/api/v3/brokerage/orders
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder
    pub async fn create_market(
        &self,
        product_id: &String,
        side: &String,
        size: &String,
    ) -> Result<OrderResponse> {
        let market = if side == "BUY" {
            MarketIOC {
                quote_size: Some(size.clone()),
                base_size: None,
            }
        } else {
            MarketIOC {
                quote_size: None,
                base_size: Some(size.clone()),
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
    /// https://api.coinbase.com/api/v3/brokerage/orders
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder
    pub async fn create_limit_gtc(
        &self,
        product_id: &String,
        side: &String,
        size: &String,
        price: &String,
        post_only: bool,
    ) -> Result<OrderResponse> {
        let limit = LimitGTC {
            base_size: size.clone(),
            limit_price: price.clone(),
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
    /// https://api.coinbase.com/api/v3/brokerage/orders
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder
    pub async fn create_limit_gtd(
        &self,
        product_id: &String,
        side: &String,
        size: &String,
        price: &String,
        end_time: &String,
        post_only: bool,
    ) -> Result<OrderResponse> {
        let limit = LimitGTD {
            base_size: size.clone(),
            limit_price: price.clone(),
            end_time: end_time.clone(),
            post_only: post_only.clone(),
        };

        let config = OrderConfiguration {
            limit_limit_gtd: Some(limit),
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
    /// https://api.coinbase.com/api/v3/brokerage/orders/historical/{order_id}
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gethistoricalorder
    pub async fn get(&self, order_id: String) -> Result<Order> {
        let resource = format!("{}/historical/{}", Self::RESOURCE.to_string(), order_id);
        match self.signer.get(resource, "".to_string()).await {
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
    /// * `params` - A Parameters to modify what is returned by the API.
    ///
    /// # Endpoint / Reference
    ///
    /// https://api.coinbase.com/api/v3/brokerage/orders/historical
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gethistoricalorders
    pub async fn get_all(&self, params: ListOrdersParams) -> Result<ListOrders> {
        let resource = format!("{}/historical/batch", Self::RESOURCE.to_string());
        match self.signer.get(resource, params.to_params()).await {
            Ok(value) => match value.json::<ListOrders>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse(
                    "could not parse orders vector".to_string(),
                )),
            },
            Err(error) => Err(error),
        }
    }
}
