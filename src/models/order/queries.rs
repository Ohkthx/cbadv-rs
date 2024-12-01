//! # Coinbase Advanced Order API
//!
//! `order/queries` contains the query parameters for the various endpoints associated with the Order API.

use serde::Serialize;

use crate::errors::CbError;
use crate::product::ProductType;
use crate::utils::QueryBuilder;
use crate::{traits::Query, types::CbResult};

use super::{OrderSide, OrderSortBy, OrderStatus, OrderType, TimeInForce};

/// Represents parameters that are optional for List Orders API request.
#[derive(Serialize, Default, Debug, Clone)]
pub struct OrderListQuery {
    /// ID(s) of order(s).
    pub order_ids: Option<Vec<String>>,
    /// Optional string of the product ID(s). Defaults to null, or fetch for all products.
    pub product_ids: Option<Vec<String>>,
    /// Only orders matching this product type are returned. Default is to return all product types. Valid options are SPOT or FUTURE.
    pub product_type: Option<ProductType>,
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
    // Sort results by a field, results use unstable pagination. Default is sort by creation time.
    pub sort_by: Option<OrderSortBy>,
}

impl Query for OrderListQuery {
    fn check(&self) -> CbResult<()> {
        if let Some(product_type) = &self.product_type {
            if *product_type == ProductType::Unknown {
                return Err(CbError::BadQuery(
                    "product_type must not be unknown".to_string(),
                ));
            }
        } else if let Some(limit) = self.limit {
            if limit == 0 {
                return Err(CbError::BadQuery(
                    "limit must be greater than 0".to_string(),
                ));
            }
        } else if let (Some(start), Some(end)) = (&self.start_date, &self.end_date) {
            if start > end {
                return Err(CbError::BadQuery(
                    "start_date must be before end_date".to_string(),
                ));
            }
        } else if let Some(sort_by) = &self.sort_by {
            if *sort_by == OrderSortBy::Unknown {
                return Err(CbError::BadQuery("sort_by must not be unknown".to_string()));
            }
        }
        Ok(())
    }

    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional_vec("order_ids", &self.order_ids)
            .push_optional_vec("product_ids", &self.product_ids)
            .push_optional("product_type", &self.product_type)
            .push_optional_vec("order_status", &self.order_status)
            .push_optional_vec("time_in_forces", &self.time_in_forces)
            .push_optional_vec("order_types", &self.order_types)
            .push_optional("order_side", &self.order_side)
            .push_optional("start_date", &self.start_date)
            .push_optional("end_date", &self.end_date)
            .push_optional_vec("asset_filters", &self.asset_filters)
            .push_optional("limit", &self.limit)
            .push_optional("cursor", &self.cursor)
            .push_optional("sort_by", &self.sort_by)
            .build()
    }
}

impl OrderListQuery {
    /// Creates a new instance of a Query to list orders.
    pub fn new() -> Self {
        Self::default()
    }

    /// The ID(s) of order(s).
    pub fn order_ids(mut self, order_ids: &[String]) -> Self {
        self.order_ids = Some(order_ids.to_vec());
        self
    }

    /// The ID(s) of the product(s) to filter orders by.
    pub fn product_ids(mut self, product_ids: &[String]) -> Self {
        self.product_ids = Some(product_ids.to_vec());
        self
    }

    /// Only orders matching this product type are returned. Default is to return all product types. Valid options are SPOT or FUTURE.
    pub fn product_type(mut self, product_type: ProductType) -> Self {
        self.product_type = Some(product_type);
        self
    }

    /// Only orders matching this order status are returned. Default is to return all order statuses.
    pub fn order_status(mut self, order_status: &[OrderStatus]) -> Self {
        self.order_status = Some(order_status.to_vec());
        self
    }

    /// Only orders matching this time in force(s) are returned. Default is to return all time in forces.
    pub fn time_in_forces(mut self, time_in_forces: &[TimeInForce]) -> Self {
        self.time_in_forces = Some(time_in_forces.to_vec());
        self
    }

    /// Type of orders to return. Default is to return all order types.
    pub fn order_types(mut self, order_types: &[OrderType]) -> Self {
        self.order_types = Some(order_types.to_vec());
        self
    }

    /// Only orders matching this side are returned. Default is to return all sides.
    pub fn order_side(mut self, order_side: OrderSide) -> Self {
        self.order_side = Some(order_side);
        self
    }

    /// Start date to fetch orders from, inclusive.
    pub fn start_date(mut self, start_date: String) -> Self {
        self.start_date = Some(start_date);
        self
    }

    /// An optional end date for the query window, exclusive. If provided only orders with creation time before this date will be returned.
    pub fn end_date(mut self, end_date: String) -> Self {
        self.end_date = Some(end_date);
        self
    }

    /// Only returns the orders where the quote, base or underlying asset matches the provided asset filter(s) (e.g. 'BTC').
    pub fn asset_filters(mut self, asset_filters: &[String]) -> Self {
        self.asset_filters = Some(asset_filters.to_vec());
        self
    }

    /// A pagination limit with no default set. If has_next is true, additional orders are available to be fetched with pagination; also the cursor value in the response can be passed as cursor parameter in the subsequent request.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Cursor used for pagination. When provided, the response returns responses after this cursor.
    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }

    /// Sort results by a field, results use unstable pagination. Default is sort by creation time.
    pub fn sort_by(mut self, sort_by: OrderSortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }
}

/// Represents parameters that are optional for List Fills API request.
///
/// # Required Fields
///
#[derive(Serialize, Debug, Clone)]
pub struct OrderListFillsQuery {
    /// The ID(s) of order(s).
    pub order_ids: Option<Vec<String>>,
    /// The ID(s) of the trades of fills.
    pub trade_ids: Option<Vec<String>>,
    /// The ID(s) of the product(s) to filter fills by.
    pub product_ids: Option<Vec<String>>,
    /// Start date. Only fills with a trade time at or after this start date are returned.
    pub start_sequence_timestamp: Option<String>,
    /// End date. Only fills with a trade time before this start date are returned.
    pub end_sequence_timestamp: Option<String>,
    /// Maximum number of fills to return in response. Defaults to 100.
    pub limit: u32,
    /// Cursor used for pagination. When provided, the response returns responses after this cursor.
    pub cursor: Option<String>,
    /// Sort results by a field, results use unstable pagination. Default is sort by creation time.
    pub sort_by: Option<OrderSortBy>,
}

impl Query for OrderListFillsQuery {
    fn check(&self) -> CbResult<()> {
        if self.limit == 0 {
            return Err(CbError::BadQuery(
                "limit must be greater than 0".to_string(),
            ));
        } else if let (Some(start), Some(end)) =
            (&self.start_sequence_timestamp, &self.end_sequence_timestamp)
        {
            if start > end {
                return Err(CbError::BadQuery(
                    "start_sequence_timestamp must be before end_sequence_timestamp".to_string(),
                ));
            }
        } else if let Some(sort_by) = &self.sort_by {
            if *sort_by == OrderSortBy::Unknown {
                return Err(CbError::BadQuery("sort_by must not be unknown".to_string()));
            }
        }

        Ok(())
    }

    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional_vec("order_ids", &self.order_ids)
            .push_optional_vec("trade_ids", &self.trade_ids)
            .push_optional_vec("product_ids", &self.product_ids)
            .push_optional("start_sequence_timestamp", &self.start_sequence_timestamp)
            .push_optional("end_sequence_timestamp", &self.end_sequence_timestamp)
            .push("limit", self.limit)
            .push_optional("cursor", &self.cursor)
            .push_optional("sort_by", &self.sort_by)
            .build()
    }
}

impl Default for OrderListFillsQuery {
    fn default() -> Self {
        Self {
            order_ids: None,
            trade_ids: None,
            product_ids: None,
            start_sequence_timestamp: None,
            end_sequence_timestamp: None,
            limit: 100,
            cursor: None,
            sort_by: None,
        }
    }
}

impl OrderListFillsQuery {
    /// Creates a new instance of a Query to list fills.
    pub fn new() -> Self {
        Self::default()
    }

    /// The ID(s) of order(s).
    pub fn order_ids(mut self, order_ids: &[String]) -> Self {
        self.order_ids = Some(order_ids.to_vec());
        self
    }

    /// The ID(s) of the trades of fills.
    pub fn trade_ids(mut self, trade_ids: &[String]) -> Self {
        self.trade_ids = Some(trade_ids.to_vec());
        self
    }

    /// The ID(s) of the product(s) to filter.
    pub fn product_ids(mut self, product_ids: &[String]) -> Self {
        self.product_ids = Some(product_ids.to_vec());
        self
    }

    /// Start date. Only fills with a trade time at or after this start date are returned.
    pub fn start_sequence_timestamp(mut self, start_sequence_timestamp: String) -> Self {
        self.start_sequence_timestamp = Some(start_sequence_timestamp);
        self
    }

    /// End date. Only fills with a trade time before this start date are returned.
    pub fn end_sequence_timestamp(mut self, end_sequence_timestamp: String) -> Self {
        self.end_sequence_timestamp = Some(end_sequence_timestamp);
        self
    }

    /// Maximum number of fills to return in response. Defaults to 100.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// Cursor used for pagination. When provided, the response returns responses after this cursor.
    pub fn cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }

    /// Sort results by a field, results use unstable pagination. Default is sort by creation time.
    pub fn sort_by(mut self, sort_by: OrderSortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }
}
