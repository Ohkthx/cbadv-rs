//! # Coinbase Advanced Websocket Client
//!
//! `websocket` gives access to the websocket stream to receive updates in a streamlined fashion.
//! Many parts of the REST API suggest using websockets instead due to ratelimits and being quicker
//! for large amount of constantly changing data.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::models::order::OrderUpdate;
use crate::models::product::{CandleUpdate, MarketTradesUpdate, ProductUpdate, TickerUpdate};
use crate::utils::deserialize_numeric;

/// WebSocket Channels that can be subscribed to.
#[derive(Serialize, Deserialize, Debug)]
pub enum Channel {
    /// Sends all products and currencies on a preset interval.
    Status,
    /// Updates every second. Candles are grouped into buckets (granularities) of five minutes.
    Candles,
    /// Real-time price updates every time a match happens.
    Ticker,
    /// Real-time price updates every 5000 milli-seconds.
    TickerBatch,
    /// All updates and easiest way to keep order book snapshot
    Level2,
    /// Only sends messages that include the authenticated user.
    User,
    /// Real-time updates every time a market trade happens.
    MarketTrades,
    /// Real-time pings from server to keep connections open.
    Heartbeats,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Channel::Status => write!(f, "status"),
            Channel::Candles => write!(f, "candles"),
            Channel::Ticker => write!(f, "ticker"),
            Channel::TickerBatch => write!(f, "ticker_batch"),
            Channel::Level2 => write!(f, "level2"),
            Channel::User => write!(f, "user"),
            Channel::MarketTrades => write!(f, "market_trades"),
            Channel::Heartbeats => write!(f, "heartbeats"),
        }
    }
}

/// Messages that could be received from the WebSocket.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    /// Sends all products and currencies on a preset interval.
    Status(StatusMessage),
    /// Updates every second. Candles are grouped into buckets (granularities) of five minutes.
    Candles(CandlesMessage),
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
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price_level: f64,
    #[serde(deserialize_with = "deserialize_numeric")]
    pub new_quantity: f64,
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

/// Candles Event received from the WebSocket, contained inside the Candles Message.
#[derive(Deserialize, Debug)]
pub struct CandlesEvent {
    pub r#type: String,
    pub candles: Vec<CandleUpdate>,
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

/// Message received from the WebSocket API. Contains updates on product statuses.
#[derive(Deserialize, Debug)]
pub struct StatusMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<StatusEvent>,
}

/// Message received from the WebSocket API. Contains updates on candles.
#[derive(Deserialize, Debug)]
pub struct CandlesMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<CandlesEvent>,
}

/// Message received from the WebSocket API. Contains updates on products and currencies.
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
#[derive(Deserialize, Debug)]
pub struct Level2Message {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<Level2Event>,
}

/// Message received from the WebSocket API. Contains order updates strictly for the user.
#[derive(Deserialize, Debug)]
pub struct UserMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<UserEvent>,
}

/// Message received from the WebSocket API. Real-time updates everytime a market trade happens.
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
#[derive(Deserialize, Debug)]
pub struct HeartbeatsMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<HeartbeatsEvent>,
}

/// Message received from the WebSocket API. Provides updates for the current subscriptions.
#[derive(Deserialize, Debug)]
pub struct SubscribeMessage {
    pub channel: String,
    pub client_id: String,
    pub timestamp: String,
    pub sequence_num: u64,
    pub events: Vec<SubscribeEvent>,
}

/// Subscription is sent to the WebSocket to enable updates for specified Channels.
#[derive(Serialize, Debug)]
pub(crate) struct Subscription {
    pub(crate) r#type: String,
    pub(crate) product_ids: Vec<String>,
    pub(crate) channel: String,
    pub(crate) api_key: String,
    pub(crate) timestamp: String,
    pub(crate) signature: String,
}
