//! # Coinbase Advanced Order API
//!
//! `order/requests` contains requests that are sent to the Order API.

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::{errors::CbError, traits::Request, types::CbResult};

use super::{OrderConfiguration, OrderSide};

/// A request send to the Order API to cancel orders.
#[derive(Serialize, Debug)]
pub struct OrderCancelRequest {
    /// Vector of Order IDs to cancel.
    pub order_ids: Vec<String>,
}

impl Request for OrderCancelRequest {
    fn check(&self) -> CbResult<()> {
        if self.order_ids.is_empty() {
            return Err(CbError::BadRequest("no order IDs provided".to_string()));
        }
        Ok(())
    }
}

impl OrderCancelRequest {
    pub fn new(order_ids: &[String]) -> Self {
        Self {
            order_ids: order_ids.to_vec(),
        }
    }
}

/// A request send to the Order API to create an order.
#[derive(Serialize, Debug)]
pub struct OrderCreateRequest {
    /// Client Order ID (UUID). Skipped if creating a preview order.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub client_order_id: String,
    /// Product ID (pair)
    pub product_id: String,
    /// Order Side: BUY or SELL.
    pub side: OrderSide,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub(crate) is_preview: bool,
    /// Configuration for the order.
    pub order_configuration: OrderConfiguration,
}

impl Request for OrderCreateRequest {
    fn check(&self) -> CbResult<()> {
        if self.client_order_id.is_empty() && !self.is_preview {
            return Err(CbError::BadRequest(
                "no client order ID provided".to_string(),
            ));
        } else if self.product_id.is_empty() {
            return Err(CbError::BadRequest("no product ID provided".to_string()));
        }
        Ok(())
    }
}

/// A request send to the Order API to edit an order.
#[serde_as]
#[derive(Serialize, Debug)]
pub struct OrderEditRequest {
    /// ID of the order to edit.
    pub order_id: String,
    /// New price for order.
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    /// New size for order.
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
}

impl Request for OrderEditRequest {
    fn check(&self) -> CbResult<()> {
        if self.order_id.is_empty() {
            return Err(CbError::BadRequest("no order ID provided".to_string()));
        } else if self.price < 0.0 {
            return Err(CbError::BadRequest(
                "price cannot be less than 0".to_string(),
            ));
        } else if self.size <= 0.0 {
            return Err(CbError::BadRequest(
                "size must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}

impl OrderEditRequest {
    /// Creates a new `OrderEditRequest`.
    pub fn new(order_id: &str, price: f64, size: f64) -> Self {
        Self {
            order_id: order_id.to_string(),
            price,
            size,
        }
    }
}

/// Represents parameters that are needed to close positions.
///
/// # Required Fields
///
/// * `client_order_id` - The unique ID provided for the order (used for identification purposes).
/// * `product_id` - The trading pair (e.g. 'BIT-28JUL23-CDE').
#[derive(Serialize, Debug)]
pub struct OrderClosePositionRequest {
    /// The unique ID provided for the order (used for identification purposes).
    pub client_order_id: String,
    /// The trading pair (e.g. 'BIT-28JUL23-CDE').
    pub product_id: String,
    /// The amount of contracts that should be closed.
    pub size: Option<u32>,
}

impl Request for OrderClosePositionRequest {
    fn check(&self) -> CbResult<()> {
        if self.client_order_id.is_empty() {
            return Err(CbError::BadRequest(
                "client_order_id is required".to_string(),
            ));
        } else if self.product_id.is_empty() {
            return Err(CbError::BadRequest("product_id is required".to_string()));
        } else if let Some(size) = self.size {
            if size == 0 {
                return Err(CbError::BadRequest(
                    "size must be greater than 0".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl OrderClosePositionRequest {
    /// Creates a new instance of a Query to close a position.
    ///
    /// # Arguments
    ///
    /// * `client_order_id` - The unique ID provided for the order (used for identification purposes).
    /// * `product_id` - The trading pair (e.g. 'BIT-28JUL23-CDE
    pub fn new(client_order_id: &str, product_id: &str) -> Self {
        Self {
            client_order_id: client_order_id.to_string(),
            product_id: product_id.to_string(),
            size: None,
        }
    }

    /// Sets the client order ID.
    /// Note: This is a required field.
    pub fn client_order_id(mut self, client_order_id: String) -> Self {
        self.client_order_id = client_order_id;
        self
    }

    /// Sets the product ID.
    /// Note: This is a required field.
    pub fn product_id(mut self, product_id: String) -> Self {
        self.product_id = product_id;
        self
    }

    /// Sets the size.
    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }
}
