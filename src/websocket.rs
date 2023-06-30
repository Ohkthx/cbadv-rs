//! # Coinbase Advanced Websocket Client
//!
//! `websocket` gives access to the websocket stream to receive updates in a streamlined fashion.
//! Many parts of the REST API suggest using websockets instead due to ratelimits and being quicker
//! for large amount of constantly changing data.

use crate::order::OrderUpdate;
use crate::product::{MarketTradesUpdate, ProductUpdate, TickerUpdate};
use crate::time;
use crate::utils::{CBAdvError, Result, Signer};

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::{connect_async, tungstenite, MaybeTlsStream, WebSocketStream};

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;
type Callback = fn(Result<Message>);

/// WebSocket Channels that can be subscribed to.
#[allow(dead_code, non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Channel {
    /// Sends all products and currencies on a preset interval.
    STATUS,
    /// Real-time price updates every time a match happens.
    TICKER,
    /// Real-time price updates every 5000 milli-seconds.
    TICKER_BATCH,
    /// All updates and easiest way to keep order book snapshot
    LEVEL2,
    /// Only sends messages that include the authenticated user.
    USER,
    /// Real-time updates every time a market trade happens.
    MARKET_TRADES,
    /// Real-time pings from server to keep connections open.
    HEARTBEATS,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Channel::STATUS => write!(f, "status"),
            Channel::TICKER => write!(f, "ticker"),
            Channel::TICKER_BATCH => write!(f, "ticker_batch"),
            Channel::LEVEL2 => write!(f, "level2"),
            Channel::USER => write!(f, "user"),
            Channel::MARKET_TRADES => write!(f, "market_trades"),
            Channel::HEARTBEATS => write!(f, "heartbeats"),
        }
    }
}

/// Messages that could be received from the WebSocket.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    /// Sends all products and currencies on a preset interval.
    Status(StatusMessage),
    /// Real-time price updates every time a match happens.
    Ticker(TickerMessage),
    /// All updates and easiest way to keep order book snapshot
    TickerBatch(TickerMessage),
    /// All updates and easiest way to keep order book snapshot
    Level2(Level2Message),
    /// Only sends messages that include the authenticated user.
    User(UserMessage),
    /// Real-time updates every time a market trade happens.
    MarketTrades(MarketTradesMessage),
    /// Real-time pings from server to keep connections open.
    Heartbeats(HeartbeatsMessage),
    /// Subscription updates.
    Subscribe(SubscribeMessage),
}

/// Data received from the WebSocket for Level2 Events.
#[derive(Deserialize, Debug)]
pub struct Level2Update {
    pub side: String,
    pub event_time: String,
    pub price_level: String,
    pub new_quantity: String,
}

/// Data received from the WebSocket for Subscription Update Events.
#[derive(Deserialize, Debug, Default)]
pub struct SubscribeUpdate {
    #[serde(default)]
    pub status: Vec<String>,
    #[serde(default)]
    pub ticker: Vec<String>,
    #[serde(default)]
    pub ticker_batch: Vec<String>,
    #[serde(default)]
    pub level2: Option<Vec<String>>,
    #[serde(default)]
    pub user: Option<Vec<String>>,
    #[serde(default)]
    pub market_trades: Option<Vec<String>>,
    #[serde(default)]
    pub heartbeats: Option<Vec<String>>,
}

/// Status Event received from the WebSocket, contained inside the Status Message.
#[derive(Deserialize, Debug)]
pub struct StatusEvent {
    pub r#type: String,
    pub products: Vec<ProductUpdate>,
}

/// Ticker Event received from the WebSocket, contained inside the Ticker Message.
#[derive(Deserialize, Debug)]
pub struct TickerEvent {
    pub r#type: String,
    pub tickers: Vec<TickerUpdate>,
}

/// Level2 Event received from the WebSocket, contained inside the Level2 Message.
#[derive(Deserialize, Debug)]
pub struct Level2Event {
    pub r#type: String,
    pub product_id: String,
    pub updates: Vec<Level2Update>,
}

/// User Event received from the WebSocket, contained inside the User Message.
#[derive(Deserialize, Debug)]
pub struct UserEvent {
    pub r#type: String,
    pub orders: Vec<OrderUpdate>,
}

/// Market Trades Event received from the WebSocket, contained inside the Market Trades Message.
#[derive(Deserialize, Debug)]
pub struct MarketTradesEvent {
    pub r#type: String,
    pub trades: Vec<MarketTradesUpdate>,
}

/// Heartbeats Event received from the WebSocket, contained inside the Heartbeats Message.
#[derive(Deserialize, Debug)]
pub struct HeartbeatsEvent {
    pub current_time: String,
    pub heartbeat_counter: u64,
}

/// Subscribe Event received from the WebSocket, contained inside the Subscribe Message.
#[derive(Deserialize, Debug)]
pub struct SubscribeEvent {
    pub subscriptions: SubscribeUpdate,
}

/// Subscribe Event received from the WebSocket, contained inside the Subscribe Message.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct StatusMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<StatusEvent>,
}

/// Message received from the WebSocket API. Contains updates on products and currencies.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TickerMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<TickerEvent>,
}

/// Message received from the WebSocket API. All order updates for a products. Best way to
/// keep a snapshot of the order book.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Level2Message {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<Level2Event>,
}

/// Message received from the WebSocket API. Contains order updates strictly for the user.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct UserMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<UserEvent>,
}

/// Message received from the WebSocket API. Real-time updates everytime a market trade happens.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct MarketTradesMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<MarketTradesEvent>,
}

/// Message received from the WebSocket API. Real-time pings from the server to keep connections
/// open.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct HeartbeatsMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<HeartbeatsEvent>,
}

/// Message received from the WebSocket API. Provides updates for the current subscriptions.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct SubscribeMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<SubscribeEvent>,
}

/// Subscription is sent to the WebSocket to enable updates for specified Channels.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct Subscription {
    pub r#type: String,
    pub product_ids: Vec<String>,
    pub channel: String,
    pub api_key: String,
    pub timestamp: String,
    pub signature: String,
}

/// Represents a Client for the Websocket API. Provides easy-access to subscribing and listening to
/// the WebSocket.
#[allow(dead_code)]
pub struct Client {
    signer: Signer,
    pub socket: Option<(Socket, Response)>,
}

impl Client {
    /// Resource for the API.
    const RESOURCE: &str = "wss://advanced-trade-ws.coinbase.com";

    /// Creates a new instance of a Client. This is a wrapper for Signer and contains a socket to
    /// the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `key` - A string that holds the key for the API service.
    /// * `secret` - A string that holds the secret for the API service.
    pub fn new(key: &str, secret: &str) -> Self {
        Self {
            signer: Signer::new(key.to_string(), secret.to_string()),
            socket: None,
        }
    }

    /// Connects to the WebSocket. This is required before subscribing, unsubscribing, and
    /// listening for updates.
    pub async fn connect(&mut self) -> Result<()> {
        match connect_async(Self::RESOURCE).await {
            Ok(response) => {
                self.socket = Some(response);
                Ok(())
            }
            Err(_) => Err(CBAdvError::BadConnection(
                "unable to get handshake".to_string(),
            )),
        }
    }

    /// Closes the WebSocket entirely.
    pub async fn close(&mut self) -> Result<()> {
        match self.socket {
            None => Err(CBAdvError::BadConnection(
                "need to connect first.".to_string(),
            )),

            Some((ref mut socket, _)) => {
                let _ = socket.close(None).await;
                Ok(())
            }
        }
    }

    /// Starts the listener which returns Messages via a callback function provided by the user.
    /// This allows the user to get objects out of the WebSocket stream for additional processing.
    /// the WebSocket. If it is unable to parse an object received, the user is supplied
    /// CBAdvError::BadParse along with the data it failed to parse.
    ///
    /// # Arguments
    ///
    /// * `callback` - A callback function that is trigger and passed the Message received via
    /// WebSocket, if an error occurred.
    pub async fn listen(&mut self, callback: Callback) -> Result<()> {
        match self.socket {
            None => Err(CBAdvError::BadConnection(
                "need to connect first.".to_string(),
            )),

            Some((ref mut socket, _)) => {
                // Get the read portion.
                let (_, read) = socket.split();

                // Read messages and send to the callback as they come in.
                let read_future = read.for_each(|message| async {
                    let data = message.unwrap().to_string();

                    // Parse the message.
                    match serde_json::from_str(&data) {
                        Ok(message) => callback(Ok(message)),
                        _ => callback(Err(CBAdvError::BadParse(format!(
                            "unable to parse message: {}",
                            data
                        )))),
                    }
                });

                read_future.await;
                Ok(())
            }
        }
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
        product_ids: &Vec<String>,
        subscribe: bool,
    ) -> Result<()> {
        // Set the correct direction for the update.
        let update = match subscribe {
            true => "subscribe".to_string(),
            false => "unsubscribe".to_string(),
        };

        let timestamp = time::now().to_string();
        let channel = channel.to_string();

        // Get the signature for authentication.
        let signature = self
            .signer
            .get_ws_signature(&timestamp, &channel, product_ids);

        // Create the subscription/unsubscription.
        let sub = Subscription {
            r#type: update,
            product_ids: product_ids.clone(),
            channel,
            api_key: self.signer.api_key.clone(),
            timestamp,
            signature,
        };

        match self.socket {
            None => Err(CBAdvError::BadConnection(
                "need to connect first.".to_string(),
            )),

            Some((ref mut socket, _)) => {
                // Serialize and send the update to the API.
                let body_str = serde_json::to_string(&sub).unwrap();
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
    pub async fn subscribe(&mut self, channel: Channel, product_ids: &Vec<String>) -> Result<()> {
        self.update(channel, product_ids, true).await
    }

    /// Unsubscribes from the product IDs for the Channel provided. This will stop additional updates
    /// coming in via the `listener` for these products.
    ///
    /// # Arguments
    ///
    /// * `channel` - The Channel that is being changed to.
    /// * `product_ids` - A vector of product IDs to no longer listen for.
    pub async fn unsubscribe(&mut self, channel: Channel, product_ids: &Vec<String>) -> Result<()> {
        self.update(channel, product_ids, false).await
    }
}
