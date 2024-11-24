//! # Coinbase Advanced Websocket Client
//!
//! `websocket` gives access to the websocket stream to receive updates in a streamlined fashion.
//! Many parts of the REST API suggest using websockets instead due to ratelimits and being quicker
//! for large amount of constantly changing data.

use std::sync::Arc;

use futures::lock::Mutex;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, tungstenite, MaybeTlsStream, WebSocketStream};

use crate::constants::websocket::{PUBLIC_ENDPOINT, SECURE_ENDPOINT};
use crate::errors::CbAdvError;
use crate::jwt::Jwt;
use crate::models::websocket::{
    Channel, ChannelAccess, SecureSubscription, Subscription, UnsignedSubscription,
};
use crate::task_tracker::TaskTracker;
use crate::time;
use crate::token_bucket::{RateLimits, TokenBucket};
use crate::traits::{CandleCallback, MessageCallback};
use crate::types::{CbResult, MessageCallbackFn, WebSocketReader};

#[cfg(feature = "config")]
use crate::config::ConfigFile;
use crate::ws::WebSocketReaders;

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Obtains the access level for the endpoint.
fn get_channel_access(channel: &Channel) -> ChannelAccess {
    match channel {
        Channel::Subscriptions => ChannelAccess::Public,
        Channel::Heartbeats => ChannelAccess::Public,
        Channel::Status => ChannelAccess::Public,
        Channel::Ticker => ChannelAccess::Public,
        Channel::TickerBatch => ChannelAccess::Public,
        Channel::MarketTrades => ChannelAccess::Public,
        Channel::Level2 => ChannelAccess::Public,
        Channel::Candles => ChannelAccess::Public,
        Channel::User => ChannelAccess::Secure,
        Channel::FuturesBalanceSummary => ChannelAccess::Secure,
    }
}

/// Builder to create a WebSocketClient.
pub struct WebSocketClientBuilder {
    /// Signs the messages sent.
    api_key: Option<String>,
    api_secret: Option<String>,
    enable_public: bool,
    enable_user: bool,
    public_bucket: Arc<Mutex<TokenBucket>>,
    secure_bucket: Arc<Mutex<TokenBucket>>,
}

impl Default for WebSocketClientBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            enable_public: true, // By default, enable public connection.
            enable_user: false,  // By default, do not enable secure connection.
            public_bucket: Arc::new(Mutex::new(TokenBucket::new(
                RateLimits::max_tokens(false, true),
                RateLimits::refresh_rate(false, true),
            ))),
            secure_bucket: Arc::new(Mutex::new(TokenBucket::new(
                RateLimits::max_tokens(false, false),
                RateLimits::refresh_rate(false, false),
            ))),
        }
    }
}

impl WebSocketClientBuilder {
    /// Creates a new WebSocketClientBuilder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Uses a configuration to initialize the the authentication.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration that implements ConfigFile trait.
    #[cfg(feature = "config")]
    pub fn with_config<T>(mut self, config: &T) -> Self
    where
        T: ConfigFile,
    {
        self.api_key = Some(config.coinbase().api_key.to_string());
        self.api_secret = Some(config.coinbase().api_secret.to_string());
        self.enable_user = true;
        self
    }

    /// Uses the provided key and secret to initialize the authentication.
    ///
    /// # Arguments
    ///
    /// * `key` - API key.
    /// * `secret` - API secret.
    pub fn with_authentication(mut self, key: &str, secret: &str) -> Self {
        self.api_key = Some(key.to_string());
        self.api_secret = Some(secret.to_string());
        self.enable_user = true;
        self
    }

    /// Enables or disables the public connection.
    ///
    /// # Arguments
    ///
    /// * `enable` - Enable or disable the public connection.
    pub fn enable_public(mut self, enable: bool) -> Self {
        self.enable_public = enable;
        self
    }

    /// Enables or disables the secure user connection.
    ///
    /// # Arguments
    ///
    /// * `enable` - Enable or disable the secure user connection.
    pub fn enable_user(mut self, enable: bool) -> Self {
        self.enable_user = enable;
        self
    }

    /// Builds the WebSocketClient.
    pub fn build(self) -> CbResult<WebSocketClient> {
        // Ensure at least one connection is enabled.
        if !self.enable_public && !self.enable_user {
            return Err(CbAdvError::BadConnection(
                "At least one of public or secure connections must be enabled.".to_string(),
            ));
        }

        // Create JWT if user connection is enabled.
        let jwt = if self.enable_user {
            let key = self.api_key.ok_or_else(|| {
                CbAdvError::BadPrivateKey("API key is required for authentication.".to_string())
            })?;
            let secret = self.api_secret.ok_or_else(|| {
                CbAdvError::BadPrivateKey("API secret is required for authentication.".to_string())
            })?;
            Some(Jwt::new(&key, &secret)?)
        } else {
            None
        };

        Ok(WebSocketClient {
            jwt,
            public_bucket: self.public_bucket,
            secure_bucket: self.secure_bucket,
            public_tx: None,
            secure_tx: None,
            enable_public: self.enable_public,
            enable_user: self.enable_user,
        })
    }
}

/// Represents a Client for the Websocket API. Provides easy-access to subscribing and listening to
/// the WebSocket.
pub struct WebSocketClient {
    /// Signs the messages sent.
    pub(crate) jwt: Option<Jwt>,
    /// Public bucket.
    pub(crate) public_bucket: Arc<Mutex<TokenBucket>>,
    /// Secure bucket.
    pub(crate) secure_bucket: Arc<Mutex<TokenBucket>>,
    /// Writes data to the public stream, gets sent to the API.
    pub(crate) public_tx: Option<SplitSink<Socket, tungstenite::Message>>,
    /// Writes data to the secure stream, gets sent to the API.
    pub(crate) secure_tx: Option<SplitSink<Socket, tungstenite::Message>>,
    /// Enable public connection.
    pub(crate) enable_public: bool,
    /// Enable secure user connection.
    pub(crate) enable_user: bool,
}

impl WebSocketClient {
    /// Connects to the WebSocket. This is required before subscribing, unsubscribing, and
    /// listening for updates. A public and secure user reader is returned to allow for `listener` to parse events.
    pub async fn connect(&mut self) -> CbResult<WebSocketReaders> {
        let mut public_stream = None;
        let mut secure_stream = None;

        if self.enable_public {
            let (public_socket, _) = connect_async(PUBLIC_ENDPOINT).await.map_err(|e| {
                CbAdvError::BadConnection(format!(
                    "Unable to establish public WebSocket connection: {}",
                    e
                ))
            })?;
            let (public_sink, stream) = public_socket.split();
            self.public_tx = Some(public_sink);
            public_stream = Some(stream);
        }

        if self.enable_user {
            let (secure_socket, _) = connect_async(SECURE_ENDPOINT).await.map_err(|e| {
                CbAdvError::BadConnection(format!(
                    "Unable to establish secure WebSocket connection: {}",
                    e
                ))
            })?;
            let (secure_sink, stream) = secure_socket.split();
            self.secure_tx = Some(secure_sink);
            secure_stream = Some(stream);
        }

        Ok(WebSocketReaders {
            public: public_stream,
            user: secure_stream,
        })
    }

    /// Waits for a token to be consumable for the correct bucket.
    async fn wait_on_bucket(&mut self, endpoint: &ChannelAccess) {
        match endpoint {
            ChannelAccess::Public => {
                let mut locked_bucket = self.public_bucket.lock().await;
                locked_bucket.wait_on().await;
            }
            ChannelAccess::Secure => {
                let mut locked_bucket = self.secure_bucket.lock().await;
                locked_bucket.wait_on().await;
            }
        }
    }

    /// Listens to a single WebSocket reader using a function callback.
    ///
    /// # Arguments
    ///
    /// * `reader` - A WebSocket reader for a single channel (e.g., public or user).
    /// * `callback` - A function callback that processes WebSocket messages.
    pub async fn listen_reader_fn(mut reader: WebSocketReader, callback: MessageCallbackFn) {
        while let Some(message) = reader.next().await {
            let data = match message {
                Ok(msg) => msg.to_string(),
                Err(err) => {
                    callback(Err(CbAdvError::BadConnection(format!(
                        "WebSocket error: {}",
                        err
                    ))));
                    continue;
                }
            };

            match serde_json::from_str(&data) {
                Ok(parsed_message) => callback(Ok(parsed_message)),
                Err(err) => callback(Err(CbAdvError::BadParse(format!(
                    "Unable to parse message: {}. Error: {}",
                    data, err
                )))),
            }
        }
    }

    /// Listens to WebSocket readers using a function callback.
    ///
    /// # Arguments
    ///
    /// * `readers` - WebSocket readers for the public and user channels.
    /// * `callback` - A function callback that processes WebSocket messages.
    pub async fn listen_readers_fn(
        &mut self,
        readers: WebSocketReaders,
        callback: MessageCallbackFn,
    ) -> Vec<JoinHandle<()>> {
        let mut handles = Vec::new();

        if let Some(public_reader) = readers.public {
            let handle = tokio::spawn(Self::listen_reader_fn(public_reader, callback));
            handles.push(handle);
        }

        if let Some(user_reader) = readers.user {
            let handle = tokio::spawn(Self::listen_reader_fn(user_reader, callback));
            handles.push(handle);
        }

        handles
    }

    /// Listens to a single WebSocket reader using a callback object that implements `MessageCallback`.
    ///
    /// # Arguments
    ///
    /// * `reader` - A WebSocket reader for a single channel (e.g., public or user).
    /// * `callback_obj` - A callback object that implements the `MessageCallback` trait.
    pub async fn listen_reader_trait<T>(mut reader: WebSocketReader, callback_obj: Arc<Mutex<T>>)
    where
        T: MessageCallback + Send + Sync + 'static,
    {
        while let Some(message) = reader.next().await {
            let data = match message {
                Ok(msg) => msg.to_string(),
                Err(err) => {
                    let mut cb_obj = callback_obj.lock().await;
                    cb_obj.message_callback(Err(CbAdvError::BadConnection(format!(
                        "WebSocket error: {}",
                        err
                    ))));
                    continue;
                }
            };

            match serde_json::from_str(&data) {
                Ok(parsed_message) => {
                    let mut cb_obj = callback_obj.lock().await;
                    cb_obj.message_callback(Ok(parsed_message));
                }
                Err(err) => {
                    let mut cb_obj = callback_obj.lock().await;
                    cb_obj.message_callback(Err(CbAdvError::BadParse(format!(
                        "Unable to parse message: {}. Error: {}",
                        data, err
                    ))));
                }
            }
        }
    }

    /// Listens to WebSocket readers using a callback object that implements `MessageCallback`.
    ///
    /// # Arguments
    ///
    /// * `readers` - WebSocket readers for the public and user channels.
    /// * `callback_obj` - A callback object that implements the `MessageCallback` trait.
    pub async fn listen_readers_trait<T>(
        readers: WebSocketReaders,
        callback_obj: Arc<Mutex<T>>,
    ) -> Vec<JoinHandle<()>>
    where
        T: MessageCallback + Send + Sync + 'static,
    {
        let mut handles = Vec::new();

        if let Some(public_reader) = readers.public {
            let cb_obj = callback_obj.clone();
            let handle = tokio::spawn(Self::listen_reader_trait(public_reader, cb_obj));
            handles.push(handle);
        }

        if let Some(user_reader) = readers.user {
            let cb_obj = callback_obj.clone();
            let handle = tokio::spawn(Self::listen_reader_trait(user_reader, cb_obj));
            handles.push(handle);
        }

        handles
    }

    /// Updates the WebSocket with either additional subscriptions or unsubscriptions. This is
    /// wrapped by `subscribe` and `unsubscribe` and sends out a Subsciptions data type.
    ///
    /// # Arguments
    ///
    /// * `channel` - The Channel that is being updated.
    /// * `product_ids` - A vector of product IDs that are being changed.
    /// * `subscribe` - Subscription updates, this is true. Unsubscribing this is false.
    pub(crate) async fn update(
        &mut self,
        channel: Channel,
        product_ids: &[String],
        subscribe: bool,
        endpoint: ChannelAccess,
    ) -> CbResult<()> {
        // Set the correct direction for the update.
        let update_type = if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        }
        .to_string();

        // Create the subscription/unsubscription.
        let sub = if endpoint == ChannelAccess::Public {
            Subscription::Unsigned(UnsignedSubscription {
                r#type: update_type,
                product_ids: product_ids.to_vec(),
                channel,
                timestamp: time::now().to_string(),
            })
        } else {
            Subscription::Secure(SecureSubscription {
                r#type: update_type,
                product_ids: product_ids.to_vec(),
                channel,
                jwt: self
                    .jwt
                    .as_ref()
                    .ok_or_else(|| {
                        CbAdvError::BadPrivateKey("User authentication required.".to_string())
                    })
                    .unwrap()
                    .encode(None)?,
            })
        };

        let body = serde_json::to_string(&sub).map_err(|e| {
            CbAdvError::BadSerialization(format!("Failed to serialize subscription: {}", e))
        })?;

        // Wait until a token is available to make the request. Immediately consume it.
        self.wait_on_bucket(&endpoint).await;

        let socket = match endpoint {
            ChannelAccess::Public => self.public_tx.as_mut().ok_or_else(|| {
                CbAdvError::BadConnection(
                    "Public WebSocket connection not established. Call `connect()` first."
                        .to_string(),
                )
            })?,
            ChannelAccess::Secure => self.secure_tx.as_mut().ok_or_else(|| {
                CbAdvError::BadConnection(
                    "Secure WebSocket connection not established. Call `connect()` first."
                        .to_string(),
                )
            })?,
        };

        socket
            .send(tungstenite::Message::text(body))
            .await
            .map_err(|e| {
                CbAdvError::BadConnection(format!("Failed to send message over WebSocket: {}", e))
            })
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
        let endpoint = get_channel_access(&channel);
        match endpoint {
            ChannelAccess::Public if !self.enable_public => {
                return Err(CbAdvError::BadConnection(
                    "Public connection is not enabled.".to_string(),
                ));
            }
            ChannelAccess::Secure if !self.enable_user => {
                return Err(CbAdvError::BadConnection(
                    "Secure user connection is not enabled.".to_string(),
                ));
            }
            _ => {}
        }
        self.update(channel, product_ids, true, endpoint).await
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
        let endpoint = get_channel_access(&channel);
        match endpoint {
            ChannelAccess::Public if !self.enable_public => {
                return Err(CbAdvError::BadConnection(
                    "Public connection is not enabled.".to_string(),
                ));
            }
            ChannelAccess::Secure if !self.enable_user => {
                return Err(CbAdvError::BadConnection(
                    "Secure user connection is not enabled.".to_string(),
                ));
            }
            _ => {}
        }
        self.update(channel, product_ids, false, endpoint).await
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
        T: CandleCallback + Send + Sync + 'static,
    {
        if !self.enable_public {
            return Err(CbAdvError::BadConnection(
                "Public connection is not enabled.".to_string(),
            ));
        }

        // Connect and spawn a task.
        match self.connect().await?.public {
            Some(public) => {
                // Start task to watch candles using user's watcher.
                let listener = tokio::spawn(TaskTracker::start(public, watcher));
                // Keep the connection open by subscribing to heartbeats.
                self.sub(Channel::Heartbeats, &[]).await?;
                // Subscribe to the candle updates for the products.
                self.sub(Channel::Candles, products).await?;
                Ok(listener)
            }
            None => Err(CbAdvError::BadConnection(
                "Public connection is not connected.".to_string(),
            )),
        }
    }
}
