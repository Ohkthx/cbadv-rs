//! Traits used to allow interfacing with advanced functionality for end-users.

use crate::models::{product::Candle, websocket::Message};
use crate::types::CbResult;

/// Used to pass to a callback to the candle watcher on a successful ejection.
pub trait CandleCallback {
    /// Called when a candle is succesfully ejected.
    ///
    /// # Arguments
    ///
    /// * `current_start` - Current UTC timestamp for a start.
    /// * `product_id` - Product the candle belongs to.
    /// * `candle` - Candle that was recently completed.
    fn candle_callback(&mut self, current_start: u64, product_id: String, candle: Candle);
}

/// Used to pass objects to the listener for greater control over message processing.
pub trait MessageCallback {
    /// This is called when processing a message from the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `msg` - Message or Error received from the WebSocket.
    fn message_callback(&mut self, msg: CbResult<Message>);
}

/// Used to pass query/paramters for a URL.
pub(crate) trait Query {
    /// Used to convert a struct into query/paramters for a URL.
    fn to_query(&self) -> String;
}

/// Represents an empty query.
pub(crate) struct NoQuery;
impl Query for NoQuery {
    fn to_query(&self) -> String {
        String::new()
    }
}
