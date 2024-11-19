//! # Coinbase Advanced Websocket Client
//!
//! `websocket` gives access to the websocket stream to receive updates in a streamlined fashion.
//! Many parts of the REST API suggest using websockets instead due to ratelimits and being quicker
//! for large amount of constantly changing data.

use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, tungstenite, MaybeTlsStream, WebSocketStream};

use crate::constants::websocket::RESOURCE_ENDPOINT;
use crate::errors::CbAdvError;
use crate::http_agent::{HttpAgent, SecureHttpAgent};
use crate::models::websocket::{Channel, Subscription};
use crate::task_tracker::TaskTracker;
use crate::time;
use crate::traits::{CandleCallback, MessageCallback};
use crate::types::{CbResult, MessageCallbackFn, WebSocketReader};

#[cfg(feature = "config")]
use crate::config::ConfigFile;

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Represents a Client for the Websocket API. Provides easy-access to subscribing and listening to
/// the WebSocket.
pub struct WebSocketClient {
    /// Signs the messages sent.
    agent: SecureHttpAgent,
    /// Writes data to the stream, gets sent to the API.
    socket_tx: Option<SplitSink<Socket, tungstenite::Message>>,
}

impl WebSocketClient {
    /// Creates a new instance of a Client. This is a wrapper for Signer and contains a socket to
    /// the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `key` - A string that holds the key for the API service.
    /// * `secret` - A string that holds the secret for the API service.
    /// * `use_sandbox` - A boolean that determines if the sandbox should be used.
    pub fn new(key: &str, secret: &str, use_sandbox: bool) -> CbResult<Self> {
        Ok(Self {
            agent: SecureHttpAgent::new(key, secret, false, use_sandbox)?,
            socket_tx: None,
        })
    }

    /// Creates a new instance of a Client using a configuration file. This is a wrapper for
    /// Signer and contains a socket to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration that implements ConfigFile trait.
    #[cfg(feature = "config")]
    pub fn from_config<T>(config: &T) -> CbResult<Self>
    where
        T: ConfigFile,
    {
        Self::new(
            &config.coinbase().api_key,
            &config.coinbase().api_secret,
            config.coinbase().use_sandbox,
        )
    }

    /// Connects to the WebSocket. This is required before subscribing, unsubscribing, and
    /// listening for updates. A reader is returned to allow for `listener` to parse events.
    pub async fn connect(&mut self) -> CbResult<WebSocketReader> {
        match connect_async(RESOURCE_ENDPOINT).await {
            Ok((socket, _)) => {
                let (sink, stream) = socket.split();
                self.socket_tx = Some(sink);
                Ok(stream)
            }
            Err(_) => Err(CbAdvError::BadConnection(
                "unable to get handshake".to_string(),
            )),
        }
    }

    /// Starts the listener which returns Messages via a callback function provided by the user.
    /// This allows the user to get objects out of the WebSocket stream for additional processing.
    /// the WebSocket. If it is unable to parse an object received, the user is supplied
    /// CBAdvError::BadParse along with the data it failed to parse.
    ///
    /// # Arguments
    ///
    /// * `reader` - Allows the listener to receive messages. `Obtained from connect``.
    /// * `callback` - A callback function that is trigger and passed the Message received via WebSocket, if an error occurred.
    pub async fn listener(reader: WebSocketReader, callback: MessageCallbackFn) {
        // Read messages and send to the callback as they come in.
        let read_future = reader.for_each(|message| {
            let data: String = match message {
                Ok(value) => value.to_string(),
                Err(err) => format!("websocket sent the following error, {}", err),
            };

            // Parse the message.
            match serde_json::from_str(&data) {
                Ok(message) => callback(Ok(message)),
                _ => callback(Err(CbAdvError::BadParse(format!(
                    "unable to parse message: {}",
                    data
                )))),
            }

            async {}
        });

        read_future.await
    }

    /// Starts the listener with a callback object that implements the `MessageCallback` trait.
    /// This allows the user to get objects out of the WebSocket stream for additional processing.
    /// the WebSocket. If it is unable to parse an object received, the user is supplied
    /// CBAdvError::BadParse along with the data it failed to parse.
    ///
    /// # Arguments
    ///
    /// * `reader` - Allows the listener to receive messages. `Obtained from connect``.
    /// * `callback_obj` - A callback object that implements `MessageCallback` trait.
    pub async fn listener_with<T>(reader: WebSocketReader, callback_obj: T)
    where
        T: MessageCallback,
    {
        // Make the callback object mutable.
        let mut obj: T = callback_obj;

        // Read messages and send to the callback as they come in.
        let read_future = reader.for_each(|message| {
            let data: String = match message {
                Ok(value) => value.to_string(),
                Err(err) => format!("websocket sent the following error, {}", err),
            };

            // Parse the message.
            match serde_json::from_str(&data) {
                Ok(message) => obj.message_callback(Ok(message)),
                _ => obj.message_callback(Err(CbAdvError::BadParse(format!(
                    "unable to parse message: {}",
                    data
                )))),
            }

            async {}
        });

        read_future.await
    }

    /// Updates the WebSocket with either additional subscriptions or unsubscriptions. This is
    /// wrapped by `subscribe` and `unsubscribe` and sends out a Subsciptions data type.
    ///
    /// # Arguments
    ///
    /// * `channel` - The Channel that is being updated.
    /// * `product_ids` - A vector of product IDs that are being changed.
    /// * `subscribe` - Subscription updates, this is true. Unsubscribing this is false.
    async fn update(
        &mut self,
        channel: Channel,
        product_ids: &[String],
        subscribe: bool,
    ) -> CbResult<()> {
        // Set the correct direction for the update.
        let update = match subscribe {
            true => "subscribe".to_string(),
            false => "unsubscribe".to_string(),
        };

        let timestamp = time::now().to_string();
        let channel = channel.to_string();

        // Create the subscription/unsubscription.
        let sub = Subscription {
            r#type: update,
            product_ids: product_ids.to_vec(),
            channel,
            jwt: self.agent.get_jwt(None)?,
            timestamp,
        };

        match self.socket_tx {
            None => Err(CbAdvError::BadConnection(
                "need to connect first.".to_string(),
            )),

            Some(ref mut socket) => {
                // Serialize and send the update to the API.
                let body_str = serde_json::to_string(&sub).unwrap();

                // Wait until a token is available to make the request. Immediately consume it.
                self.agent.bucket_mut().wait_on().await;

                match socket.send(tungstenite::Message::text(body_str)).await {
                    Ok(_) => Ok(()),
                    _ => Ok(()),
                }
            }
        }
    }

    /// Subscribes to the Channel provided with interests in the specified product IDs.
    /// These updates can be viewed with calling the `listen` function and setting a callback to
    /// receive the Messages on.
    ///
    /// # Arguments
    ///
    /// * `channel` - The Channel that is being subscribed to.
    /// * `product_ids` - A vector of product IDs to listen for.
    pub async fn subscribe(&mut self, channel: Channel, product_ids: &[String]) -> CbResult<()> {
        self.update(channel, product_ids, true).await
    }

    /// Shorthand version of `subscribe`.
    ///
    /// # Arguments
    ///
    /// * `channel` - The Channel that is being subscribed to.
    /// * `product_ids` - A vector of product IDs to listen for.
    pub async fn sub(&mut self, channel: Channel, product_ids: &[String]) -> CbResult<()> {
        self.subscribe(channel, product_ids).await
    }

    /// Unsubscribes from the product IDs for the Channel provided. This will stop additional updates
    /// coming in via the `listener` for these products.
    ///
    /// # Arguments
    ///
    /// * `channel` - The Channel that is being changed to.
    /// * `product_ids` - A vector of product IDs to no longer listen for.
    pub async fn unsubscribe(&mut self, channel: Channel, product_ids: &[String]) -> CbResult<()> {
        self.update(channel, product_ids, false).await
    }

    /// Shorthand version of `unsubscribe`.
    ///
    /// # Arguments
    ///
    /// * `channel` - The Channel that is being changed to.
    /// * `product_ids` - A vector of product IDs to no longer listen for.
    pub async fn unsub(&mut self, channel: Channel, product_ids: &[String]) -> CbResult<()> {
        self.unsubscribe(channel, product_ids).await
    }

    /// Watches candles for a set of products, producing candles once they are considered complete.
    ///
    /// # Argument
    ///
    /// * `products` - Products to watch for candles for.
    /// * `watcher` - User-defined struct that implements `CandleCallback` to send completed candles to.
    pub async fn watch_candles<T>(
        &mut self,
        products: &[String],
        watcher: T,
    ) -> CbResult<JoinHandle<()>>
    where
        T: CandleCallback + Send + 'static,
    {
        // Connect and spawn a task.
        let reader = match self.connect().await {
            Ok(reader) => reader,
            Err(err) => return Err(err),
        };

        // Starts task to watch candles using users watcher.
        let listener = tokio::spawn(TaskTracker::start(reader, watcher));

        // Keep the connection open.
        match self.sub(Channel::Heartbeats, &[]).await {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        // Subscribe to the candle updates for the products.
        match self.sub(Channel::Candles, products).await {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        Ok(listener)
    }
}
