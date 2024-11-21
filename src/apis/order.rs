//! # Coinbase Advanced Order API
//!
//! `order` gives access to the Order API and the various endpoints associated with it.
//! These allow you to obtain past created orders, create new orders, and cancel orders.

use crate::constants::orders::{
    BATCH_ENDPOINT, CANCEL_BATCH_ENDPOINT, CLOSE_POSITION_ENDPOINT, CREATE_PREVIEW_ENDPOINT,
    EDIT_ENDPOINT, EDIT_PREVIEW_ENDPOINT, FILLS_ENDPOINT, RESOURCE_ENDPOINT,
};
use crate::errors::CbAdvError;
use crate::http_agent::{HttpAgent, SecureHttpAgent};
use crate::order::{
    CancelOrders, CancelOrdersWrapper, ClosePositionQuery, CreateOrder, CreateOrderPreview,
    EditOrder, EditOrderPreview, EditOrderResponse, ListFillsQuery, ListOrdersQuery, Order,
    OrderResponse, OrderStatus, OrderWrapper, PaginatedFills, PaginatedOrders,
};
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Order API for the service.
pub struct OrderApi {
    /// Object used to sign requests made to the API.
    agent: SecureHttpAgent,
}

impl OrderApi {
    /// Creates a new instance of the Order API. This grants access to order information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: SecureHttpAgent) -> Self {
        Self { agent }
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

        let response = self
            .agent
            .post(CANCEL_BATCH_ENDPOINT, &NoQuery, body)
            .await?;
        let data: CancelOrdersWrapper = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.results)
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
            product_ids: Some(vec![product_id.to_string()]),
            order_status: Some(vec![OrderStatus::Open]),
            ..Default::default()
        };

        // Obtain all open orders for the given product.
        let open_orders = self.get_all(product_id, Some(query)).await?;

        // Collect the IDs of orders to cancel.
        let order_ids: Vec<String> = open_orders
            .iter()
            .map(|order| order.order_id.clone())
            .collect();

        // If no orders are found, return a "nothing to do" error.
        if order_ids.is_empty() {
            return Err(CbAdvError::NothingToDo(format!(
                "No open orders found to cancel for product '{}'.",
                product_id
            )));
        }

        // Cancel the orders and return the response.
        self.cancel(&order_ids).await
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

        let response = self.agent.post(EDIT_ENDPOINT, &NoQuery, body).await?;
        let data: EditOrderResponse = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data)
    }

    /// Preview creating an order.
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
    /// https://api.coinbase.com/api/v3/brokerage/orders/preview
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_previeworder>
    pub async fn preview_create(&mut self, order: &CreateOrder) -> CbResult<CreateOrderPreview> {
        let response = self
            .agent
            .post(CREATE_PREVIEW_ENDPOINT, &NoQuery, order)
            .await?;
        let data: CreateOrderPreview = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data)
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
    ) -> CbResult<EditOrderPreview> {
        let body = EditOrder {
            order_id: order_id.to_string(),
            size: size.to_string(),
            price: price.to_string(),
        };

        let response = self
            .agent
            .post(EDIT_PREVIEW_ENDPOINT, &NoQuery, body)
            .await?;
        let data: EditOrderPreview = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data)
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
    pub async fn create(&mut self, order: &CreateOrder) -> CbResult<OrderResponse> {
        let response = self.agent.post(RESOURCE_ENDPOINT, &NoQuery, order).await?;
        let data: OrderResponse = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data)
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
        let response = self.agent.get(&resource, &NoQuery).await?;
        let data: OrderWrapper = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.order)
    }

    /// Obtains various orders from the API.
    ///
    /// # Arguments
    ///
    /// * `query` - A Parameters to modify what is returned by the API.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/historical
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_gethistoricalorders>
    pub async fn get_bulk(&mut self, query: &ListOrdersQuery) -> CbResult<PaginatedOrders> {
        let response = self.agent.get(BATCH_ENDPOINT, query).await?;
        let data: PaginatedOrders = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data)
    }

    /// Obtains all orders for a product based on the product ID. (ex. "BTC-USD").
    /// This wraps `get_bulk` and makes several additional requests until there are no
    /// additional orders.
    ///
    /// NOTE: NOT A STANDARD API FUNCTION. QoL function that may require additional API requests than normal.
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
        let mut query = query.unwrap_or_default();

        // Set the product ID for the query.
        query.product_ids = Some(vec![product_id.to_string()]);
        let mut all_orders: Vec<Order> = vec![];

        // Fetch orders until no more pages are available.
        loop {
            let listed_orders = self.get_bulk(&query).await?;
            all_orders.extend(listed_orders.orders);

            if listed_orders.has_next {
                query.cursor = Some(listed_orders.cursor);
            } else {
                break;
            }
        }

        Ok(all_orders)
    }

    /// Obtains fills from the API.
    ///
    /// # Arguments
    ///
    /// * `query` - A Parameters to modify what is returned by the API.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/historical/fills
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getfills>
    pub async fn fills(&mut self, query: &ListFillsQuery) -> CbResult<PaginatedFills> {
        let response = self.agent.get(FILLS_ENDPOINT, query).await?;
        let data: PaginatedFills = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data)
    }

    /// Places an order to close any open positions for a specified product_id.
    ///
    /// # Arguments
    ///
    /// * `query` - A Parameters to modify what is returned by the API.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/close_position
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_closeposition>
    pub async fn close_position(&mut self, query: &ClosePositionQuery) -> CbResult<OrderResponse> {
        let response = self.agent.get(CLOSE_POSITION_ENDPOINT, query).await?;
        let data: OrderResponse = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data)
    }
}
