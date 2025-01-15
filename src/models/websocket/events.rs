use serde::{Deserialize, Serialize};

use super::{
    CandleUpdate, EventType, FuturesBalanceSummaryUpdate, Level2Update, MarketTradesUpdate,
    OrderUpdate, ProductUpdate, SubscribeUpdate, TickerUpdate,
};

/// Events that could be received in a message.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Events {
    Status(Vec<StatusEvent>),
    Candles(Vec<CandlesEvent>),
    Ticker(Vec<TickerEvent>),
    TickerBatch(Vec<TickerEvent>),
    Level2(Vec<Level2Event>),
    User(Vec<UserEvent>),
    MarketTrades(Vec<MarketTradesEvent>),
    Heartbeats(Vec<HeartbeatsEvent>),
    Subscribe(Vec<SubscribeEvent>),
    FuturesBalanceSummary(Vec<FuturesSummaryBalanceEvent>),
}

/// The status event containing updates to products.
#[derive(Serialize, Deserialize, Debug)]
pub struct StatusEvent {
    pub r#type: EventType,
    pub products: Vec<ProductUpdate>,
}

/// The candles event containing updates to candles.
#[derive(Serialize, Deserialize, Debug)]
pub struct CandlesEvent {
    pub r#type: EventType,
    pub candles: Vec<CandleUpdate>,
}

/// The ticker event containing updates to tickers.
#[derive(Serialize, Deserialize, Debug)]
pub struct TickerEvent {
    pub r#type: EventType,
    pub tickers: Vec<TickerUpdate>,
}

/// The level2 event containing updates to the order book.
#[derive(Serialize, Deserialize, Debug)]
pub struct Level2Event {
    pub r#type: EventType,
    pub product_id: String,
    pub updates: Vec<Level2Update>,
}

/// The user event containing updates to orders.
#[derive(Serialize, Deserialize, Debug)]
pub struct UserEvent {
    pub r#type: EventType,
    pub orders: Vec<OrderUpdate>,
}

/// The market trades event containing updates to trades.
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketTradesEvent {
    pub r#type: EventType,
    pub trades: Vec<MarketTradesUpdate>,
}

/// The heartbeats event containing the current time and heartbeat counter.
#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatsEvent {
    pub current_time: String,
    pub heartbeat_counter: u64,
}

/// The subscribe event containing the current subscriptions.
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeEvent {
    pub subscriptions: SubscribeUpdate,
}

/// The futures summary balance event containing the current futures account balance.
#[derive(Serialize, Deserialize, Debug)]
pub struct FuturesSummaryBalanceEvent {
    pub r#type: EventType,
    pub fcm_balance_summary: FuturesBalanceSummaryUpdate,
}
