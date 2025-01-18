use serde::{Deserialize as SerdeDeserialize, Serialize};

use crate::types::WebSocketReader;

use super::{SecureSubscription, UnsignedSubscription};

/// WebSocket Channels that can be subscribed to.
#[derive(Serialize, SerdeDeserialize, PartialEq, Debug, Eq, Hash, Clone)]
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
    /// All updates and easiest way to keep order book snapshot.
    #[serde(alias = "l2_data")]
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
    /// Bids / Buy side.
    Bid,
    /// Asks / Offer side.
    /// NOTE: As of 20241209, the API has a typo and uses "offer" instead of "ask".
    #[serde(alias = "offer")]
    Ask,
}

/// Types for the endpoints.
#[derive(PartialEq, Debug, Eq, Clone, Hash)]
pub enum EndpointType {
    Public,
    User,
}

/// WebSocket Reader Endpoints.
#[derive(Debug)]
pub enum Endpoint {
    Public((EndpointType, WebSocketReader)),
    User((EndpointType, WebSocketReader)),
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub(crate) enum Subscription {
    Secure(SecureSubscription),
    Unsigned(UnsignedSubscription),
}
