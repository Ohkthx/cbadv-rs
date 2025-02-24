//! # Coinbase Advanced Websocket Client
//!
//! `websocket` gives access to the websocket stream to receive updates in a streamlined fashion.
//! Many parts of the REST API suggest using websockets instead due to ratelimits and being quicker
//! for large amount of constantly changing data.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use futures::stream::{SplitSink, Stream};
use futures::task::{noop_waker_ref, Context, Poll};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{Error as WsError, Message as WsMessage};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::constants::websocket::{PUBLIC_ENDPOINT, SECURE_ENDPOINT};
use crate::errors::CbError;
use crate::jwt::Jwt;
use crate::models::websocket::{
    Channel, Endpoint, EndpointStream, EndpointType, Message, SecureSubscription, Subscription,
    UnsignedSubscription, WebSocketEndpoints, WebSocketSubscriptions,
};
use crate::time;
use crate::token_bucket::{RateLimits, TokenBucket};
use crate::types::CbResult;

#[cfg(feature = "config")]
use crate::config::ConfigFile;

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Obtains the endpoint associated with the channel.
fn get_channel_endpoint(channel: &Channel) -> EndpointType {
    match channel {
        Channel::Subscriptions
        | Channel::Heartbeats
        | Channel::Status
        | Channel::Ticker
        | Channel::TickerBatch
        | Channel::MarketTrades
        | Channel::Level2
        | Channel::Candles => EndpointType::Public,
        Channel::User | Channel::FuturesBalanceSummary => EndpointType::User,
    }
}

/// Builds a new WebSocket Client (`WebSocketClient`) that directly interacts with the Coinbase Advanced API.
pub struct WebSocketClientBuilder {
    api_key: Option<String>,
    api_secret: Option<String>,
    use_public: bool,
    use_user: bool,
    max_retries: u32,
    public_bucket: Arc<Mutex<TokenBucket>>,
    secure_bucket: Arc<Mutex<TokenBucket>>,
}

impl Default for WebSocketClientBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            use_public: true, // By default, enable public connection.
            use_user: false,  // By default, disable user connection.
            max_retries: 0,   // By default, do not auto-reconnect.
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
    /// Creates a new `WebSocketClientBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Uses a configuration to initialize the the authentication.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration that implements `ConfigFile` trait.
    #[cfg(feature = "config")]
    pub fn with_config<T>(mut self, config: &T) -> Self
    where
        T: ConfigFile,
    {
        self.api_key = Some(config.coinbase().api_key.to_string());
        self.api_secret = Some(config.coinbase().api_secret.to_string());
        self.use_user = true;
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
        self.use_user = true;
        self
    }

    /// Enables or disables the public connection.
    ///
    /// # Arguments
    ///
    /// * `enable` - Enable or disable the public connection.
    pub fn use_public(mut self, enable: bool) -> Self {
        self.use_public = enable;
        self
    }

    /// Enables or disables auto-reconnecting the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `enable` - Enable or disable auto-reconnecting the WebSocket.
    pub fn auto_reconnect(mut self, enable: bool) -> Self {
        if enable {
            self.max_retries = 14;
        } else {
            self.max_retries = 0;
        }
        self
    }

    /// Sets the maximum number of retries for auto-reconnecting the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `max_retries` - Maximum number of retries.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Builds the `WebSocketClient`.
    ///
    /// # Errors
    ///
    /// Returns a `CbError` if the API key or secret are missing or if both public and secure connections are disabled.
    pub fn build(self) -> CbResult<WebSocketClient> {
        // Ensure at least one connection is enabled.
        if !self.use_public && !self.use_user {
            return Err(CbError::BadConnection(
                "At least one of public or secure connections must be enabled.".to_string(),
            ));
        }

        // Create JWT if user connection is enabled.
        let jwt = if self.use_user {
            let key = self.api_key.ok_or_else(|| {
                CbError::BadPrivateKey("API key is required for authentication.".to_string())
            })?;
            let secret = self.api_secret.ok_or_else(|| {
                CbError::BadPrivateKey("API secret is required for authentication.".to_string())
            })?;
            Some(Jwt::new(&key, &secret)?)
        } else {
            None
        };

        Ok(WebSocketClient {
            jwt,
            public_bucket: self.public_bucket,
            secure_bucket: self.secure_bucket,
            public_tx: Arc::new(Mutex::new(None)),
            secure_tx: Arc::new(Mutex::new(None)),
            enable_public: self.use_public,
            enable_user: self.use_user,
            max_retries: self.max_retries,
            subscriptions: Arc::new(Mutex::new(WebSocketSubscriptions::new())),
        })
    }
}

/// A WebSocket Client used to interactive with the Coinbase Advanced API. Provides easy-access to subscribing and listening to the WebSocket.
pub struct WebSocketClient {
    /// Signs the messages sent.
    pub(crate) jwt: Option<Jwt>,
    /// Public bucket.
    pub(crate) public_bucket: Arc<Mutex<TokenBucket>>,
    /// Secure bucket.
    pub(crate) secure_bucket: Arc<Mutex<TokenBucket>>,
    /// Writes data to the public stream, gets sent to the API.
    pub(crate) public_tx: Arc<Mutex<Option<SplitSink<Socket, WsMessage>>>>,
    /// Writes data to the secure stream, gets sent to the API.
    pub(crate) secure_tx: Arc<Mutex<Option<SplitSink<Socket, WsMessage>>>>,
    /// Enable public connection.
    pub(crate) enable_public: bool,
    /// Enable secure user connection.
    pub(crate) enable_user: bool,
    /// Automatically reconnect the WebSocket after a disconnection.
    pub(crate) max_retries: u32,
    /// Tracked subscriptions.
    pub(crate) subscriptions: Arc<Mutex<WebSocketSubscriptions>>,
}

impl Clone for WebSocketClient {
    fn clone(&self) -> Self {
        Self {
            jwt: self.jwt.clone(),
            public_bucket: self.public_bucket.clone(),
            secure_bucket: self.secure_bucket.clone(),
            public_tx: self.public_tx.clone(),
            secure_tx: self.secure_tx.clone(),
            enable_public: self.enable_public,
            enable_user: self.enable_user,
            max_retries: self.max_retries,
            subscriptions: self.subscriptions.clone(),
        }
    }
}

impl WebSocketClient {
    /// Connects to the endpoints specified in the builder. This is required before subscribing to any channels.
    ///
    /// # Errors
    ///
    /// Returns a `CbError` if the WebSocket connection fails.
    pub async fn connect(&self) -> CbResult<WebSocketEndpoints> {
        let mut endpoints = WebSocketEndpoints::default();

        if self.enable_public {
            let endpoint = self.connect_endpoint(&EndpointType::Public).await?;
            endpoints.add(EndpointType::Public, endpoint);
        }

        if self.enable_user {
            let endpoint = self.connect_endpoint(&EndpointType::User).await?;
            endpoints.add(EndpointType::User, endpoint);
        }

        Ok(endpoints)
    }

    /// Connects to the WebSocket endpoint.
    async fn connect_endpoint(&self, endpoint_type: &EndpointType) -> CbResult<Endpoint> {
        match endpoint_type {
            EndpointType::Public => {
                let (public_socket, _) = connect_async(PUBLIC_ENDPOINT).await.map_err(|why| {
                    CbError::BadConnection(format!(
                        "Unable to establish public WebSocket connection: {why}",
                    ))
                })?;
                let (public_sink, stream) = public_socket.split();
                {
                    let mut tx = self.public_tx.lock().await;
                    *tx = Some(public_sink);
                }
                Ok(Endpoint::Public((EndpointType::Public, stream)))
            }
            EndpointType::User => {
                let (secure_socket, _) = connect_async(SECURE_ENDPOINT).await.map_err(|why| {
                    CbError::BadConnection(format!(
                        "Unable to establish secure user WebSocket connection: {why}",
                    ))
                })?;
                let (secure_sink, stream) = secure_socket.split();
                {
                    let mut tx = self.secure_tx.lock().await;
                    *tx = Some(secure_sink);
                }
                Ok(Endpoint::User((EndpointType::User, stream)))
            }
        }
    }

    /// Reconnects to a specific endpoint. Returns the reader of the endpoint.
    ///
    /// # Errors
    ///
    /// Returns a `CbError` if the WebSocket connection fails.
    async fn handle_reconnect(&mut self, endpoint_type: &EndpointType) -> CbResult<Endpoint> {
        let endpoint = self.connect_endpoint(endpoint_type).await?;

        // Re-subscribe to previous channels for this endpoint.
        let subs = {
            let subscriptions = self.subscriptions.lock().await;
            subscriptions.get(endpoint_type).await
        };

        // Add the subscriptions back.
        for (channel, product_ids) in subs {
            self.subscribe(&channel, &product_ids).await?;
        }

        Ok(endpoint)
    }

    /// Waits for a reconnection to occur. This is used when a WebSocket connection is lost.
    ///
    /// # Errors
    ///
    /// Returns a `CbError` if the WebSocket connection fails or auto-reconnect is disabled.
    async fn wait_on_reconnect(&mut self, endpoint_type: &EndpointType) -> CbResult<Endpoint> {
        if self.max_retries == 0 {
            return Err(CbError::BadConnection(
                "Auto-reconnect is disabled. Exiting...".to_string(),
            ));
        }

        let mut retries = 0;
        let mut retry_delay = 2;

        // Rety until max retries hit.
        while retries < self.max_retries {
            match self.handle_reconnect(endpoint_type).await {
                Ok(endpoint) => return Ok(endpoint),
                Err(why) => {
                    eprintln!(
                        "Failed to reconnect WebSocket: {why}. Retrying in {retry_delay} seconds..."
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(retry_delay)).await;
                    retries += 1;
                    retry_delay = (retry_delay * 2).min(60);
                }
            }
        }

        Err(CbError::BadConnection(format!(
            "Failed to reconnect WebSocket after {retries} attempts."
        )))
    }

    /// Reconnects to the WebSocket endpoint. Returns a new `EndpointStream`.
    /// This is used when the WebSocket connection is lost.
    ///
    /// # Arguments
    ///
    /// * `stream` - The current `EndpointStream` that was being listened to.
    ///
    /// # Errors
    ///
    /// Returns a `CbError` if the WebSocket connection fails.
    pub async fn reconnect<E>(&mut self, stream: E) -> CbResult<WebSocketEndpoints>
    where
        E: Into<EndpointStream>,
    {
        let mut new_endpoints = WebSocketEndpoints::default();

        match stream.into() {
            EndpointStream::Single(route, _) => {
                // Reconnect and return a new Single EndpointStream.
                match self.wait_on_reconnect(&route).await {
                    Ok(endpoint) => {
                        new_endpoints.add(route, endpoint);
                        Ok(new_endpoints)
                    }
                    Err(why) => Err(why),
                }
            }
            EndpointStream::Multiple(_) => {
                // Obtain all the endpoints that need to be reconnected.
                let keys = {
                    let subs = self.subscriptions.lock().await;
                    subs.get_keys()
                };

                // Iterate over each endpoint and attempt to reconnect.
                for endpoint_type in keys {
                    match self.wait_on_reconnect(&endpoint_type).await {
                        Ok(new_endpoint) => {
                            new_endpoints.add(endpoint_type.clone(), new_endpoint);
                        }
                        Err(why) => {
                            return Err(why);
                        }
                    }
                }

                if new_endpoints.is_empty() {
                    return Err(CbError::BadConnection(
                        "Failed to reconnect to any endpoints.".to_string(),
                    ));
                }

                Ok(new_endpoints)
            }
        }
    }

    /// Listens to WebSocket readers, supporting both single and multiple endpoints.
    ///
    /// # Arguments
    ///
    /// * `endpoints` - A single `Endpoint` or multiple `WebSocketEndpoints`.
    /// * `callback` - The asynchronous closure to invoke on each message.
    pub async fn listen<E, F, Fut>(&mut self, endpoints: E, mut callback: F)
    where
        E: Into<EndpointStream>,
        F: FnMut(CbResult<Message>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send,
    {
        let mut stream = endpoints.into();

        loop {
            while let Some(message) = stream.next().await {
                if let Some(result) = Self::process_message(message) {
                    if let Err(CbError::BadConnection(_)) = &result {
                        // Handle reconnection logic.
                        match self.reconnect(stream).await {
                            Ok(new_stream) => {
                                stream = new_stream.into();
                                break; // Exit inner loop to reconnect.
                            }
                            Err(why) => {
                                eprintln!("Failed to reconnect: {why}");
                                return; // Exit function if reconnection fails
                            }
                        }
                    }

                    // Invoke the asynchronous closure with the result.
                    callback(result).await;
                }
            }
        }
    }

    /// Fetches messages from the WebSocket stream with a limit on the number of messages to fetch.
    ///
    /// NOTE: Adequate pauses / sleeps between calls should be added to prevent busy-looping.
    ///
    /// # Arguments
    ///
    /// * `stream` - The WebSocket stream to get messages from.
    /// * `limit` - The maximum number of messages to fetch. Use `usize::MAX` to fetch all messages.
    /// * `action` - The action to take on each message.
    ///
    /// # Errors
    ///
    /// Returns a `String` if the user returns an error within the action.
    pub fn fetch_sync<F>(
        &self,
        stream: &mut EndpointStream,
        limit: usize,
        mut action: F,
    ) -> Result<(), String>
    where
        F: FnMut(CbResult<Message>) -> Result<(), String>,
    {
        let mut count = 0;

        while count <= limit || limit == usize::MAX {
            // Use poll_next to check for available messages without waiting.
            match Pin::new(&mut *stream).poll_next(&mut Context::from_waker(noop_waker_ref())) {
                Poll::Ready(Some(message)) => {
                    // Process and add the message to the result vector if valid.
                    if let Some(result) = Self::process_message(message) {
                        action(result)?;
                    }

                    count += 1;
                }
                Poll::Ready(None) | Poll::Pending => {
                    // No more messages available or stream is pending; exit the loop.
                    break;
                }
            }
        }

        Ok(())
    }

    /// Asynchronously fetches messages from the WebSocket stream with a limit on the number of messages to fetch.
    ///
    /// NOTE: Adequate pauses / sleeps between calls should be added to prevent busy-looping.
    ///
    /// # Arguments
    ///
    /// * `stream` - The WebSocket stream to get messages from.
    /// * `limit` - The maximum number of messages to fetch. Use `usize::MAX` to fetch all messages.
    /// * `action` - The action to take on each message.
    ///
    /// # Errors
    ///
    /// Returns a `String` if the user returns an error within the action.
    pub async fn fetch_async<F, Fut>(
        &self,
        stream: &mut EndpointStream,
        limit: usize,
        mut action: F,
    ) -> Result<(), String>
    where
        F: FnMut(CbResult<Message>) -> Fut,
        Fut: Future<Output = Result<(), String>>,
    {
        let mut count = 0;

        while count <= limit || limit == usize::MAX {
            // Use poll_next to check for available messages without waiting.
            match Pin::new(&mut *stream).poll_next(&mut Context::from_waker(noop_waker_ref())) {
                Poll::Ready(Some(message)) => {
                    // Process and add the message to the result vector if valid.
                    if let Some(result) = Self::process_message(message) {
                        action(result).await?;
                    }

                    count += 1;
                }
                Poll::Ready(None) | Poll::Pending => {
                    // No more messages available or stream is pending; exit the loop.
                    break;
                }
            }
        }

        Ok(())
    }

    /// Waits for a token to be consumable for the correct bucket.
    async fn wait_on_bucket(&mut self, endpoint: &EndpointType) {
        match endpoint {
            EndpointType::Public => {
                let mut locked_bucket = self.public_bucket.lock().await;
                locked_bucket.wait_on().await;
            }
            EndpointType::User => {
                let mut locked_bucket = self.secure_bucket.lock().await;
                locked_bucket.wait_on().await;
            }
        }
    }

    /// Processes the WebSocket message and returns a `Message` if successful.
    ///
    /// # Arguments
    ///
    /// * `message` - The WebSocket message to process.
    pub fn process_message(message: Result<WsMessage, WsError>) -> Option<CbResult<Message>> {
        match message {
            Ok(msg) => match msg {
                WsMessage::Text(data) => {
                    let result = serde_json::from_str::<Message>(&data).map_err(|why| {
                        CbError::BadParse(format!("Unable to parse message: {data}. Error: {why}"))
                    });
                    Some(result)
                }
                WsMessage::Ping(_)
                | WsMessage::Pong(_)
                | WsMessage::Binary(_)
                | WsMessage::Frame(_) => None, // Ignored.
                WsMessage::Close(frame) => {
                    eprintln!("WebSocket closed: {frame:?}");
                    Some(Err(CbError::BadConnection("WebSocket closed".to_string())))
                }
            },
            Err(why) => Some(Err(CbError::BadConnection(format!(
                "WebSocket error: {why}"
            )))),
        }
    }

    /// Updates the WebSocket with either additional subscriptions or unsubscriptions. This is
    /// wrapped by `subscribe` and `unsubscribe` and sends out a Subsciptions data type.
    ///
    /// # Arguments
    ///
    /// * `channel` - The Channel that is being updated.
    /// * `product_ids` - A vector of product IDs that are being changed.
    /// * `action` - The action being taken (either "subscribe" or "unsubscribe").
    /// * `endpoint` - The endpoint type (either public or user).
    async fn update(
        &mut self,
        channel: &Channel,
        product_ids: &[String],
        action: &str,
        endpoint: &EndpointType,
    ) -> CbResult<()> {
        // Create the subscription/unsubscription.
        let sub = if *endpoint == EndpointType::Public {
            Subscription::Unsigned(UnsignedSubscription {
                r#type: action.to_string(),
                product_ids: product_ids.to_vec(),
                channel: channel.clone(),
                timestamp: time::now().to_string(),
            })
        } else {
            Subscription::Secure(SecureSubscription {
                r#type: action.to_string(),
                product_ids: product_ids.to_vec(),
                channel: channel.clone(),
                jwt: self
                    .jwt
                    .as_ref()
                    .ok_or_else(|| {
                        CbError::BadPrivateKey("User authentication required.".to_string())
                    })
                    .unwrap()
                    .encode(None)?,
            })
        };

        let body = serde_json::to_string(&sub).map_err(|why| {
            CbError::BadSerialization(format!("Failed to serialize subscription: {why}"))
        })?;
        let body_message = WsMessage::text(body);

        // Wait until a token is available to make the request. Immediately consume it.
        self.wait_on_bucket(endpoint).await;

        match endpoint {
            EndpointType::Public => {
                let mut tx = self.public_tx.lock().await;
                if let Some(socket) = tx.as_mut() {
                    socket.send(body_message).await.map_err(|why| {
                        CbError::BadConnection(format!(
                            "Failed to send message over WebSocket: {why}"
                        ))
                    })
                } else {
                    Err(CbError::BadConnection(
                        "Public WebSocket connection not established. Call `connect()` first."
                            .to_string(),
                    ))
                }
            }
            EndpointType::User => {
                let mut tx = self.secure_tx.lock().await;
                if let Some(socket) = tx.as_mut() {
                    socket.send(body_message).await.map_err(|why| {
                        CbError::BadConnection(format!(
                            "Failed to send message over WebSocket: {why}"
                        ))
                    })
                } else {
                    Err(CbError::BadConnection(
                        "Secure WebSocket connection not established. Call `connect()` first."
                            .to_string(),
                    ))
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
    ///
    /// # Errors
    ///
    /// Returns a `CbError` if the public or secure user connection is not enabled.
    pub async fn subscribe(&mut self, channel: &Channel, product_ids: &[String]) -> CbResult<()> {
        let route = &get_channel_endpoint(channel);
        match route {
            EndpointType::Public if !self.enable_public => {
                return Err(CbError::BadConnection(
                    "Public connection is not enabled.".to_string(),
                ));
            }
            EndpointType::User if !self.enable_user => {
                return Err(CbError::BadConnection(
                    "Secure user connection is not enabled.".to_string(),
                ));
            }
            _ => {}
        }

        // Send the subscription.
        self.update(channel, product_ids, "subscribe", route)
            .await?;

        {
            // Update the subscriptions.
            let mut subs = self.subscriptions.lock().await;
            subs.add(channel, product_ids, route).await;
        }

        Ok(())
    }

    /// Unsubscribes from the product IDs for the Channel provided. This will stop additional updates
    /// coming in via the `listener` for these products.
    ///
    /// # Arguments
    ///
    /// * `channel` - The Channel that is being changed to.
    /// * `product_ids` - A vector of product IDs to no longer listen for.
    ///
    /// # Errors
    ///
    /// Returns a `CbError` if the public or secure user connection is not enabled.
    pub async fn unsubscribe(&mut self, channel: &Channel, product_ids: &[String]) -> CbResult<()> {
        let route = &get_channel_endpoint(channel);
        match route {
            EndpointType::Public if !self.enable_public => {
                return Err(CbError::BadConnection(
                    "Public connection is not enabled.".to_string(),
                ));
            }
            EndpointType::User if !self.enable_user => {
                return Err(CbError::BadConnection(
                    "Secure user connection is not enabled.".to_string(),
                ));
            }
            _ => {}
        }

        // Send the unsubscription.
        self.update(channel, product_ids, "unsubscribe", route)
            .await?;

        {
            // Update the subscriptions.
            let mut subs = self.subscriptions.lock().await;
            subs.remove(channel, product_ids, route).await;
        }
        Ok(())
    }
}
