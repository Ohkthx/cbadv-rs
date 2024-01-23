//! Task Tracker is the underlying object used to track candle updates.

use crate::product::{Candle, CandleUpdate};
use crate::time::Granularity;
use crate::utils::CbResult;
use crate::websocket::{
    self, CandleCallback, CandlesEvent, Message, MessageCallback, WebSocketReader,
};

use chrono::Utc;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;

/// Granularity of Candles from the WebSocket Candle subscription.
/// NOTE: This is a restriction by CoinBase and cannot be currently changed (20231015)
const WEBSOCKET_GRANULARITY: Granularity = Granularity::FiveMinute;

/// Tracks the candle watcher task.
pub struct TaskTracker<T>
where
    T: CandleCallback,
{
    /// Holds most recent candle processed for each product. [key: Product Id, value: Candle]
    candles: HashMap<String, Candle>,
    /// User-defined object that implements `CandleCallback``, triggered on completed candles.
    user_watcher: T,
}

impl<T> TaskTracker<T>
where
    T: CandleCallback,
{
    /// Starts the task that tracks candles for completion.
    ///
    /// # Arguments
    ///
    /// * `reader` - WebSocket reader to receive updates.
    /// * `user_obj` - User object that implements `CandleCallback` to receive completed candles.
    pub async fn start(reader: WebSocketReader, user_obj: T)
    where
        T: CandleCallback,
    {
        let tracker = Self {
            candles: HashMap::new(),
            user_watcher: user_obj,
        };

        // Start the listener.
        websocket::listener_with(reader, tracker).await;
    }

    /// Returns a completed candle. A completed candle means a candle with a newer timestamp was passed
    /// to the function indicating the prior candle is done.
    ///
    /// # Arguements
    ///
    /// * `product_id` - Product the candle belongs to.
    /// * `new_candle` - New candles obtained from the WebSocket.
    fn check_candle(&mut self, product_id: &str, new_candle: Candle) -> Option<Candle> {
        // Obtain candle from the tracked candles.
        match self.candles.get(product_id) {
            Some(candle) => {
                if candle.start < new_candle.start {
                    // Remove/eject complete candle, and replace with new series candle.
                    let old = self.candles.remove(product_id).unwrap();
                    self.candles.insert(product_id.to_string(), new_candle);
                    return Some(old as Candle);
                } else {
                    // Replace existing.
                    self.candles.insert(product_id.to_string(), new_candle);
                }
            }
            None => {
                // Candle not found, create first instance.
                self.candles.insert(product_id.to_string(), new_candle);
            }
        }
        None
    }
}

impl<T> MessageCallback for TaskTracker<T>
where
    T: CandleCallback,
{
    /// Required to pass TaskTracker to the websocket listener.
    fn message_callback(&mut self, msg: CbResult<Message>) {
        // Filter all non-candle and empty updates.
        let ev: Vec<CandlesEvent> = match msg {
            Ok(value) => match value {
                Message::Candles(value) => {
                    if value.events.is_empty() {
                        // No events / updates to process.
                        return;
                    }
                    // Events being worked with.
                    value.events
                }
                // Non-candle message.
                _ => return,
            },
            // WebSocket error.
            Err(err) => {
                println!("!WEBSOCKET ERROR! {}", err);
                return;
            }
        };

        // Combine all updates and sort by more recent -> oldest.
        let mut updates: Vec<CandleUpdate> = ev.iter().flat_map(|c| c.candles.clone()).collect();

        match updates.len().cmp(&1usize) {
            // Sort if there are more than 1 update.
            Ordering::Greater => updates.sort_by(|a, b| b.data.start.cmp(&a.data.start)),
            // No updates to process.
            Ordering::Less => return,
            _ => (),
        };

        // Check the most recent candle based on `start`, see if there is a completed series.
        let update = updates.remove(0);
        let product_id: String = update.product_id;
        let candle = match self.check_candle(&product_id, update.data) {
            Some(c) => c,
            None => return,
        };

        // Get the current "start" time for a candle.
        let now: u64 = Utc::now().timestamp() as u64;
        let start: u64 = now - (now % (Granularity::to_secs(&WEBSOCKET_GRANULARITY) as u64 * 2));

        // Call the users object.
        self.user_watcher.candle_callback(start, product_id, candle);
    }
}
