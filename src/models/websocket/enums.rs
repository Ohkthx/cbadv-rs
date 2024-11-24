use serde::{Deserialize as SerdeDeserialize, Serialize};

/// WebSocket Channels that can be subscribed to.
#[derive(Serialize, SerdeDeserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
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
    /// Real-time updates every time a market trade happens.
    MarketTrades,
    /// Real-time pings from server to keep connections open.
    Heartbeats,
    /// Only sends messages that include the authenticated user.
    User,
    /// Real-time updates every time a user's futures balance changes.
    FuturesBalanceSummary,
    /// Updates to subscription status.
    Subscriptions,
}

#[derive(Serialize, SerdeDeserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Snapshot,
    Update,
}

#[derive(Serialize, SerdeDeserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Level2Side {
    Bid,
    Ask,
}

/// WebSocket Channel Access.
#[derive(PartialEq, Debug)]
pub(crate) enum ChannelAccess {
    Public,
    Secure,
}
