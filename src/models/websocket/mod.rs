//! # Coinbase Advanced Websocket Client
//!
//! `websocket` gives access to the websocket stream to receive updates in a streamlined fashion.
//! Many parts of the REST API suggest using websockets instead due to ratelimits and being quicker
//! for large amount of constantly changing data.

mod enums;
mod events;
mod message;
mod responses;

pub use enums::*;
pub use events::*;
pub use message::*;
pub use responses::*;

use serde::Serialize;

use crate::types::WebSocketReader;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub(crate) enum Subscription {
    Secure(SecureSubscription),
    Unsigned(UnsignedSubscription),
}

/// Secure (authenticated) Subscription is sent to the WebSocket to enable updates for specified Channels.
#[derive(Serialize, Debug)]
pub(crate) struct SecureSubscription {
    /// Subscribing or unsubscribing.
    pub(crate) r#type: String,
    /// Product IDs to (un)subscribe to.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) product_ids: Vec<String>,
    /// Channel to (un)subscribe to.
    pub(crate) channel: Channel,
    /// JWT token for authentication.
    pub(crate) jwt: String,
}

/// Unsigned (public) Subscription is sent to the WebSocket to enable updates for specified Channels.
#[derive(Serialize, Debug)]
pub(crate) struct UnsignedSubscription {
    /// Subscribing or unsubscribing.
    pub(crate) r#type: String,
    /// Product IDs to (un)subscribe to.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) product_ids: Vec<String>,
    /// Channel to (un)subscribe to.
    pub(crate) channel: Channel,
    /// Timestamp for the subscription.
    pub(crate) timestamp: String,
}

/// Used to store the WebSocket readers for the public and user channels.
pub struct WebSocketReaders {
    /// Public channel reader.
    pub public: Option<WebSocketReader>,
    /// User channel reader.
    pub user: Option<WebSocketReader>,
}
