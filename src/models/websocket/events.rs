use serde::Deserialize;

use super::{
    CandleUpdate, EventType, Level2Update, MarketTradesUpdate, OrderUpdate, ProductUpdate,
    SubscribeUpdate, TickerUpdate,
};

/// Events that could be received in a message.
#[derive(Debug)]
pub enum Event {
    Status(StatusEvent),
    Candles(CandlesEvent),
    Ticker(TickerEvent),
    TickerBatch(TickerEvent),
    Level2(Level2Event),
    User(UserEvent),
    MarketTrades(MarketTradesEvent),
    Heartbeats(HeartbeatsEvent),
    Subscribe(SubscribeEvent),
}

/// The status event containing updates to products.
#[derive(Deserialize, Debug)]
pub struct StatusEvent {
    pub r#type: EventType,
    pub products: Vec<ProductUpdate>,
}

/// The candles event containing updates to candles.
#[derive(Deserialize, Debug)]
pub struct CandlesEvent {
    pub r#type: EventType,
    pub candles: Vec<CandleUpdate>,
}

/// The ticker event containing updates to tickers.
#[derive(Deserialize, Debug)]
pub struct TickerEvent {
    pub r#type: EventType,
    pub tickers: Vec<TickerUpdate>,
}

/// The level2 event containing updates to the order book.
#[derive(Deserialize, Debug)]
pub struct Level2Event {
    pub r#type: EventType,
    pub product_id: String,
    pub updates: Vec<Level2Update>,
}

/// The user event containing updates to orders.
#[derive(Deserialize, Debug)]
pub struct UserEvent {
    pub r#type: EventType,
    pub orders: Vec<OrderUpdate>,
}

/// The market trades event containing updates to trades.
#[derive(Deserialize, Debug)]
pub struct MarketTradesEvent {
    pub r#type: EventType,
    pub trades: Vec<MarketTradesUpdate>,
}

/// The heartbeats event containing the current time and heartbeat counter.
#[derive(Deserialize, Debug)]
pub struct HeartbeatsEvent {
    pub current_time: String,
    pub heartbeat_counter: u64,
}

/// The subscribe event containing the current subscriptions.
#[derive(Deserialize, Debug)]
pub struct SubscribeEvent {
    pub subscriptions: SubscribeUpdate,
}
