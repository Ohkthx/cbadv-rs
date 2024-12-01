//! # Coinbase Advanced Order API
//!
//! `order` gives access to the Order API and the various endpoints associated with it.
//! These allow you to obtain past created orders, create new orders, and cancel orders.

use crate::constants::orders::{
    BATCH_ENDPOINT, CANCEL_BATCH_ENDPOINT, CLOSE_POSITION_ENDPOINT, CREATE_PREVIEW_ENDPOINT,
    EDIT_ENDPOINT, EDIT_PREVIEW_ENDPOINT, FILLS_ENDPOINT, RESOURCE_ENDPOINT,
};
use crate::errors::CbError;
use crate::http_agent::SecureHttpAgent;
use crate::order::{
    Order, OrderCancelRequest, OrderCancelResponse, OrderCancelWrapper, OrderClosePositionRequest,
    OrderCreatePreview, OrderCreateRequest, OrderCreateResponse, OrderEditPreview,
    OrderEditRequest, OrderEditResponse, OrderListFillsQuery, OrderListQuery, OrderStatus,
    OrderWrapper, PaginatedFills, PaginatedOrders,
};
use crate::traits::{HttpAgent, NoQuery};
use crate::types::CbResult;

/// Provides access to the Order API for the service.
pub struct OrderApi {
    /// Object used to sign requests made to the API.
    agent: Option<SecureHttpAgent>,
}

impl OrderApi {
    /// Creates a new instance of the Order API. This grants access to order information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: Option<SecureHttpAgent>) -> Self {
        Self { agent }
    }

    /// Cancel orders.
    ///
    /// # Arguments
    ///
    /// * `request` - A struct containing what orders to cancel.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/batch_cancel
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_cancelorders>
    pub async fn cancel(
        &mut self,
        request: &OrderCancelRequest,
    ) -> CbResult<Vec<OrderCancelResponse>> {
        let agent = get_auth!(self.agent, "cancel orders");
        let response = agent.post(CANCEL_BATCH_ENDPOINT, &NoQuery, request).await?;
        let data: OrderCancelWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
    }

    /// Cancel all OPEN orders for a specific product ID.
    ///
    /// NOTE: NOT A STANDARD API FUNCTION. QoL function that may require additional API requests
    /// than normal.
    ///
    /// # Arguments
    ///
    /// * `product_id` - Product to cancel all OPEN orders for.
    pub async fn cancel_all(&mut self, product_id: &str) -> CbResult<Vec<OrderCancelResponse>> {
        is_auth!(self.agent, "cancel all orders");

        let query = OrderListQuery {
            product_ids: Some(vec![product_id.to_string()]),
            order_status: Some(vec![OrderStatus::Open]),
            ..Default::default()
        };

        // Obtain all open orders for the given product.
        let open_orders = self.get_all(product_id, &query).await?;

        // Collect the IDs of orders to cancel.
        let request = OrderCancelRequest::new(
            &open_orders
                .iter()
                .map(|order| order.order_id.clone())
                .collect::<Vec<String>>(),
        );

        // No orders to cancel.
        if request.order_ids.is_empty() {
            return Ok(vec![]);
        }

        // Cancel the orders and return the response.
        self.cancel(&request).await
    }

    /// Edit an order with a specified new size, or new price. Only limit order types, with time
    /// in force type of good-till-cancelled can be edited.
    ///
    /// CAUTION: You lose your place in line if you increase size or increase/decrease price.
    ///
    /// # Arguments
    ///
    /// * `request` - A struct containing the order ID, new size, and new price.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/edit
    ///
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_editorder
    pub async fn edit(&mut self, request: &OrderEditRequest) -> CbResult<OrderEditResponse> {
        let agent = get_auth!(self.agent, "edit order");
        let response = agent.post(EDIT_ENDPOINT, &NoQuery, request).await?;
        let data: OrderEditResponse = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
    }

    /// Preview creating an order.
    ///
    /// # Arguments
    ///
    /// * `request` - A struct containing the order details to preview.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/preview
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_previeworder>
    pub async fn preview_create(
        &mut self,
        request: &OrderCreateRequest,
    ) -> CbResult<OrderCreatePreview> {
        let agent = get_auth!(self.agent, "preview create order");
        let response = agent
            .post(CREATE_PREVIEW_ENDPOINT, &NoQuery, request)
            .await?;
        let data: OrderCreatePreview = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
    }

    /// Simulate an edit order request with a specified new size, or new price, to preview the result of an edit. Only
    /// limit order types, with time in force type of good-till-cancelled can be edited.
    ///
    /// # Arguments
    ///
    /// * `request` - A struct containing the order ID, new size, and new price.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/edit_preivew
    ///
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_previeweditorder
    pub async fn preview_edit(&mut self, request: &OrderEditRequest) -> CbResult<OrderEditPreview> {
        let agent = get_auth!(self.agent, "preview edit order");
        let response = agent.post(EDIT_PREVIEW_ENDPOINT, &NoQuery, request).await?;
        let data: OrderEditPreview = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
    }

    /// Create an order.
    ///
    /// # Arguments
    ///
    /// * `request` - A struct containing the order details to create.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_postorder>
    pub async fn create(&mut self, request: &OrderCreateRequest) -> CbResult<OrderCreateResponse> {
        let agent = get_auth!(self.agent, "create order");
        let response = agent.post(RESOURCE_ENDPOINT, &NoQuery, request).await?;
        let data: OrderCreateResponse = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
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
        let agent = get_auth!(self.agent, "get order");
        let resource = format!("{}/historical/{}", RESOURCE_ENDPOINT, order_id);
        let response = agent.get(&resource, &NoQuery).await?;
        let data: OrderWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
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
    pub async fn get_bulk(&mut self, query: &OrderListQuery) -> CbResult<PaginatedOrders> {
        let agent = get_auth!(self.agent, "get bulk orders");
        let response = agent.get(BATCH_ENDPOINT, query).await?;
        let data: PaginatedOrders = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
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
    /// * `query` - A Parameters to modify what is returned by the API.
    pub async fn get_all(
        &mut self,
        product_id: &str,
        query: &OrderListQuery,
    ) -> CbResult<Vec<Order>> {
        is_auth!(self.agent, "get all orders");

        // Set the product ID for the query.
        let mut query = query.clone().product_ids(&[product_id.to_string()]);
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
    pub async fn fills(&mut self, query: &OrderListFillsQuery) -> CbResult<PaginatedFills> {
        let agent = get_auth!(self.agent, "get fills");
        let response = agent.get(FILLS_ENDPOINT, query).await?;
        let data: PaginatedFills = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
    }

    /// Places an order to close any open positions for a specified product_id.
    ///
    /// # Arguments
    ///
    /// * `request` - A request as to what position to close.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/orders/close_position
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_closeposition>
    pub async fn close_position(
        &mut self,
        request: &OrderClosePositionRequest,
    ) -> CbResult<OrderCreateResponse> {
        let agent = get_auth!(self.agent, "close position");
        let response = agent
            .post(CLOSE_POSITION_ENDPOINT, &NoQuery, request)
            .await?;
        let data: OrderCreateResponse = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
    }
}
