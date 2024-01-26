//! # Coinbase Advanced Order API
//!
//! `order` gives access to the Order API and the various endpoints associated with it.
//! These allow you to obtain past created orders, create new orders, and cancel orders.

use uuid::Uuid;

use crate::constants::orders::{
    BATCH_ENDPOINT, CANCEL_BATCH_ENDPOINT, EDIT_ENDPOINT, EDIT_PREVIEW_ENDPOINT, FILLS_ENDPOINT,
    RESOURCE_ENDPOINT,
};
use crate::errors::CbAdvError;
use crate::order::{
    CancelOrders, CancelOrdersResponse, CreateOrder, EditOrder, EditOrderResponse, LimitGtc,
    LimitGtd, ListFillsQuery, ListOrdersQuery, ListedFills, ListedOrders, MarketIoc, Order,
    OrderConfiguration, OrderResponse, OrderStatus, OrderStatusResponse, PreviewEditOrderResponse,
    StopLimitGtc, StopLimitGtd,
};
use crate::signer::Signer;
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Order API for the service.
pub struct OrderApi {
    /// Object used to sign requests made to the API.
    signer: Signer,
}

impl OrderApi {
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
    pub async fn cancel(&mut self, order_ids: &[String]) -> CbResult<Vec<OrderResponse>> {
        let body = CancelOrders {
            order_ids: order_ids.to_vec(),
        };

        match self
            .signer
            .post(CANCEL_BATCH_ENDPOINT, &NoQuery, body)
            .await
        {
            Ok(value) => match value.json::<CancelOrdersResponse>().await {
                Ok(resp) => Ok(resp.results),
                Err(_) => Err(CbAdvError::BadParse("cancel order object".to_string())),
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
    pub async fn cancel_all(&mut self, product_id: &str) -> CbResult<Vec<OrderResponse>> {
        let query = ListOrdersQuery {
            product_id: Some(product_id.to_string()),
            order_status: Some(vec![OrderStatus::Open]),
            ..Default::default()
        };

        // Obtain all open orders.
        match self.get_all(product_id, Some(query)).await {
            Ok(orders) => {
                // Build list of orders to cancel.
                let order_ids: Vec<String> = orders.iter().map(|o| o.order_id.clone()).collect();

                // Do nothing since no orders found.
                if order_ids.is_empty() {
                    return Err(CbAdvError::NothingToDo(
                        "no orders found to cancel".to_string(),
                    ));
                }

                // Cancel the order list.
                self.cancel(&order_ids).await
            }
            Err(error) => Err(error),
        }
    }

    /// Edit an order with a specified new size, or new price. Only limit order types, with time
    /// in force type of good-till-cancelled can be edited.
    ///
    /// CAUTION: You lose your place in line if you increase size or increase/decrease price.
    ///
    /// # Arguments
    ///
    /// * `order_id` - ID of the order to edit.
    /// * `size` - New size of the order.
    /// * `price` - New price of the order.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/edit
    ///
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_editorder
    pub async fn edit(
        &mut self,
        order_id: &str,
        size: f64,
        price: f64,
    ) -> CbResult<EditOrderResponse> {
        let body = EditOrder {
            order_id: order_id.to_string(),
            size: size.to_string(),
            price: price.to_string(),
        };

        match self.signer.post(EDIT_ENDPOINT, &NoQuery, body).await {
            Ok(value) => match value.json::<EditOrderResponse>().await {
                Ok(edits) => Ok(edits),
                Err(_) => Err(CbAdvError::BadParse(
                    "could not parse edit order object".to_string(),
                )),
            },
            Err(error) => Err(error),
        }
    }

    /// Simulate an edit order request with a specified new size, or new price, to preview the result of an edit. Only
    /// limit order types, with time in force type of good-till-cancelled can be edited.
    ///
    /// # Arguments
    ///
    /// * `order_id` - ID of the order to edit.
    /// * `size` - New size of the order.
    /// * `price` - New price of the order.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/edit_preivew
    ///
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_previeweditorder
    pub async fn preview_edit(
        &mut self,
        order_id: &str,
        size: f64,
        price: f64,
    ) -> CbResult<PreviewEditOrderResponse> {
        let body = EditOrder {
            order_id: order_id.to_string(),
            size: size.to_string(),
            price: price.to_string(),
        };

        match self
            .signer
            .post(EDIT_PREVIEW_ENDPOINT, &NoQuery, body)
            .await
        {
            Ok(value) => match value.json::<PreviewEditOrderResponse>().await {
                Ok(response) => Ok(response),
                Err(_) => Err(CbAdvError::BadParse(
                    "could not parse preview edit order response".to_string(),
                )),
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
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder>
    async fn create(
        &mut self,
        product_id: &str,
        side: &str,
        configuration: OrderConfiguration,
    ) -> CbResult<OrderResponse> {
        let body = CreateOrder {
            client_order_id: Uuid::new_v4().to_string(),
            product_id: product_id.to_string(),
            side: side.to_string(),
            order_configuration: configuration,
        };

        match self.signer.post(RESOURCE_ENDPOINT, &NoQuery, body).await {
            Ok(value) => match value.json::<OrderResponse>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CbAdvError::BadParse("created order object".to_string())),
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
        &mut self,
        product_id: &str,
        side: &str,
        size: &f64,
    ) -> CbResult<OrderResponse> {
        let market = if side == "BUY" {
            MarketIoc {
                quote_size: Some(size.to_string()),
                base_size: None,
            }
        } else {
            MarketIoc {
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
        &mut self,
        product_id: &str,
        side: &str,
        size: &f64,
        price: &f64,
        post_only: bool,
    ) -> CbResult<OrderResponse> {
        let limit = LimitGtc {
            base_size: size.to_string(),
            limit_price: price.to_string(),
            post_only,
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
        &mut self,
        product_id: &str,
        side: &str,
        size: &f64,
        price: &f64,
        end_time: &str,
        post_only: bool,
    ) -> CbResult<OrderResponse> {
        let limit = LimitGtd {
            base_size: size.to_string(),
            limit_price: price.to_string(),
            end_time: end_time.to_string(),
            post_only,
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
        &mut self,
        product_id: &str,
        side: &str,
        size: &f64,
        limit_price: &f64,
        stop_price: &f64,
        stop_direction: &str,
    ) -> CbResult<OrderResponse> {
        let stoplimit = StopLimitGtc {
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
    #[allow(clippy::too_many_arguments)]
    pub async fn create_stop_limit_gtd(
        &mut self,
        product_id: &str,
        side: &str,
        size: &f64,
        limit_price: &f64,
        stop_price: &f64,
        stop_direction: &str,
        end_time: &str,
    ) -> CbResult<OrderResponse> {
        let stoplimit = StopLimitGtd {
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
    pub async fn get(&mut self, order_id: &str) -> CbResult<Order> {
        let resource = format!("{}/historical/{}", RESOURCE_ENDPOINT, order_id);
        match self.signer.get(&resource, &NoQuery).await {
            Ok(value) => match value.json::<OrderStatusResponse>().await {
                Ok(resp) => Ok(resp.order),
                Err(_) => Err(CbAdvError::BadParse(
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
    pub async fn get_bulk(&mut self, query: &ListOrdersQuery) -> CbResult<ListedOrders> {
        match self.signer.get(BATCH_ENDPOINT, query).await {
            Ok(value) => match value.json::<ListedOrders>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CbAdvError::BadParse(
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
        &mut self,
        product_id: &str,
        query: Option<ListOrdersQuery>,
    ) -> CbResult<Vec<Order>> {
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
    pub async fn fills(&mut self, query: &ListFillsQuery) -> CbResult<ListedFills> {
        match self.signer.get(FILLS_ENDPOINT, query).await {
            Ok(value) => match value.json::<ListedFills>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CbAdvError::BadParse(
                    "could not parse fills vector".to_string(),
                )),
            },
            Err(error) => Err(error),
        }
    }
}
