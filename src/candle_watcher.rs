//! Candle Watcher is the underlying object used to track candle updates.

use std::cmp::Ord;
use std::collections::HashMap;

use chrono::Utc;

use crate::constants::websocket::GRANULARITY;
use crate::models::product::Candle;
use crate::models::websocket::{CandleUpdate, Channel, Event, Message};
use crate::traits::{CandleCallback, MessageCallback};
use crate::types::CbResult;
use crate::ws::Endpoint;
use crate::WebSocketClient;

/// Tracks the candle watcher task.
pub(crate) struct CandleWatcher<T>
where
    T: CandleCallback,
{
    /// Holds the most recent candle processed for each product. [key: Product Id, value: Candle]
    candles: HashMap<String, Candle>,
    /// User-defined object that implements `CandleCallback`, triggered on completed candles.
    user_watcher: T,
}

impl<T> CandleWatcher<T>
where
    T: CandleCallback,
{
    /// Starts the task that tracks candles for completion.
    ///
    /// # Arguments
    ///
    /// * `reader` - WebSocket reader to receive updates.
    /// * `user_obj` - User object that implements `CandleCallback` to receive completed candles.
    pub(crate) async fn start(mut client: WebSocketClient, endpoint: Endpoint, user_obj: T)
    where
        T: CandleCallback + Send + Sync + 'static,
    {
        let tracker = Self {
            candles: HashMap::new(),
            user_watcher: user_obj,
        };

        // Start the listener.
        client.listen_trait(endpoint, tracker).await;
    }

    /// Returns a completed candle if a newer candle is received.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product this candle belongs to.
    /// * `new_candle` - The new candle update received from the WebSocket.
    fn check_candle(&mut self, product_id: &str, new_candle: Candle) -> Option<Candle> {
        // Retrieve the current candle for the product.
        match self.candles.get(product_id) {
            Some(existing_candle) => {
                if existing_candle.start < new_candle.start {
                    // A newer candle has been received; replace the existing candle.
                    let completed_candle = self.candles.remove(product_id).unwrap();
                    self.candles.insert(product_id.to_string(), new_candle);
                    Some(completed_candle) // Return the completed candle.
                } else {
                    // Update the existing candle without considering it complete.
                    self.candles.insert(product_id.to_string(), new_candle);
                    None
                }
            }
            None => {
                // No existing candle; add the new candle as the initial one.
                self.candles.insert(product_id.to_string(), new_candle);
                None
            }
        }
    }

    /// Extracts candle updates from a WebSocket message.
    ///
    /// # Arguments
    ///
    /// * `message` - The WebSocket message to extract updates from.
    ///
    /// # Returns
    ///
    /// A vector of `CandleUpdate` sorted by timestamp (newest first).
    fn extract_candle_updates(&self, message: &Message) -> Vec<CandleUpdate> {
        let mut updates: Vec<CandleUpdate> = message
            .events
            .iter()
            .filter_map(|event| {
                if let Event::Candles(candles_event) = event {
                    Some(candles_event.candles.clone())
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        // Sort updates by timestamp (newest first).
        updates.sort_by(|a, b| b.data.start.cmp(&a.data.start));
        updates
    }

    /// Processes a vector of candle updates.
    ///
    /// # Arguments
    ///
    /// * `updates` - The sorted vector of `CandleUpdate` to process.
    fn process_candle_updates(&mut self, mut updates: Vec<CandleUpdate>) {
        if let Some(update) = updates.pop() {
            let product_id = update.product_id.clone();
            let new_candle = update.data;

            if let Some(completed_candle) = self.check_candle(&product_id, new_candle) {
                self.trigger_user_callback(product_id, completed_candle);
            }
        }
    }

    /// Triggers the user's callback with a completed candle.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The ID of the product associated with the candle.
    /// * `completed_candle` - The completed candle to send to the callback.
    fn trigger_user_callback(&mut self, product_id: String, completed_candle: Candle) {
        let now = Utc::now().timestamp() as u64;
        let start_time = now - (now % (GRANULARITY * 2));

        self.user_watcher
            .candle_callback(start_time, product_id, completed_candle);
    }
}

impl<T> MessageCallback for CandleWatcher<T>
where
    T: CandleCallback + Send + Sync,
{
    /// Handles incoming messages and processes candle updates.
    fn message_callback(&mut self, msg: CbResult<Message>) {
        match msg {
            Ok(message) => {
                if message.channel != Channel::Candles {
                    return; // Ignore non-candle messages.
                }

                // Extract candle updates and process them.
                let updates = self.extract_candle_updates(&message);
                if updates.is_empty() {
                    return; // No updates to process.
                }

                // Process the most recent update and handle completed candles.
                self.process_candle_updates(updates);
            }
            Err(err) => {
                eprintln!("!WEBSOCKET ERROR! {}", err);
            }
        }
    }
}
