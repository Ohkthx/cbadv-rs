use std::collections::{HashMap, HashSet};
use std::pin::Pin;
use std::sync::Arc;

use futures::Stream;
use futures_util::stream::{self, SelectAll};
use serde::Serialize;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{Error as WsError, Message as WsMessage};

use super::{Channel, Endpoint, EndpointType};
use crate::types::Socket;

type SplitStream = stream::SplitStream<Socket>;
type ChannelSubscriptions = HashMap<Channel, Vec<String>>;

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

/// Holds all WebSocket endpoints.
#[derive(Debug, Default)]
pub struct WebSocketEndpoints {
    /// Endpoints accessible by key.
    pub(crate) endpoints: HashMap<EndpointType, Endpoint>,
}

impl WebSocketEndpoints {
    /// Create a new `WebSocketEndpoints`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an endpoint to the `WebSocketEndpoints`.
    ///
    /// # Arguments
    ///
    /// * `endpoint_type` - The type of endpoint.
    pub fn add(&mut self, endpoint_type: EndpointType, endpoint: Endpoint) {
        self.endpoints.insert(endpoint_type, endpoint);
    }

    /// Check if the `WebSocketEndpoints` contains an endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint_type` - The type of endpoint.
    pub fn get(&self, endpoint_type: &EndpointType) -> Option<&Endpoint> {
        self.endpoints.get(endpoint_type)
    }

    /// Check if the `WebSocketEndpoints` contains a mutable reference to an endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint_type` - The type of endpoint.
    pub fn get_mut(&mut self, endpoint_type: &EndpointType) -> Option<&mut Endpoint> {
        self.endpoints.get_mut(endpoint_type)
    }

    /// Take an endpoint from the `WebSocketEndpoints`.
    ///
    /// # Arguments
    ///
    /// * `endpoint_type` - The type of endpoint.
    pub fn take_endpoint(&mut self, endpoint_type: &EndpointType) -> Option<Endpoint> {
        self.endpoints.remove(endpoint_type)
    }

    /// Get the public endpoint.
    pub fn public(&self) -> Option<&Endpoint> {
        self.get(&EndpointType::Public)
    }

    /// Get the user endpoint.
    pub fn user(&self) -> Option<&Endpoint> {
        self.get(&EndpointType::User)
    }

    /// Converts the `WebSocketEndpoints` into a vector of Endpoints.
    pub(crate) fn extract_to_vec(&mut self) -> Vec<Endpoint> {
        let mut endpoints = Vec::new();
        for (_, endpoint) in self.endpoints.drain() {
            endpoints.push(endpoint);
        }

        endpoints
    }
}

/// Stores the current subscriptions for each channel for each endpoint.
#[derive(Debug, Clone)]
pub(crate) struct WebSocketSubscriptions {
    /// The subscriptions for each channel for each endpoint.
    pub(crate) data: HashMap<EndpointType, Arc<Mutex<ChannelSubscriptions>>>,
}

impl Default for WebSocketSubscriptions {
    fn default() -> Self {
        let data = HashMap::new();
        Self { data }
    }
}

impl WebSocketSubscriptions {
    /// Create a new `WebSocketSubscriptions`.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Add subscriptions to the specified channel.

    pub(crate) async fn add(
        &mut self,
        channel: &Channel,
        product_ids: &[String],
        endpoint: &EndpointType,
    ) {
        // Get or insert the Arc<Mutex<...>> for the endpoint.
        let subs_mutex = self
            .data
            .entry(endpoint.clone())
            .or_insert_with(|| Arc::new(Mutex::new(HashMap::new())))
            .clone();

        // Add the product IDs to the subscriptions.
        let mut subs = subs_mutex.lock().await;
        subs.entry(channel.clone())
            .and_modify(|ids| {
                let existing_ids: HashSet<String> = ids.iter().cloned().collect();
                for id in product_ids {
                    if !existing_ids.contains(id) {
                        ids.push(id.clone());
                    }
                }
            })
            .or_insert_with(|| product_ids.to_vec());
    }

    /// Remove the specified product IDs from the subscriptions.
    pub(crate) async fn remove(
        &mut self,
        channel: &Channel,
        product_ids: &[String],
        endpoint: &EndpointType,
    ) {
        if let Some(subs_mutex) = self.data.get(endpoint) {
            let mut subs = subs_mutex.lock().await;

            // Remove the product IDs from the subscriptions.
            if let Some(ids) = subs.get_mut(channel) {
                ids.retain(|id| !product_ids.contains(id));
            }
        }
    }

    /// Get the subscriptions for the specified endpoint.
    pub(crate) async fn get(&self, endpoint: &EndpointType) -> HashMap<Channel, Vec<String>> {
        if let Some(subs_mutex) = self.data.get(endpoint) {
            let subs = subs_mutex.lock().await;
            subs.clone()
        } else {
            HashMap::new()
        }
    }

    /// Obtains all of the keys (endpoints) that have subscriptions.
    pub(crate) fn get_keys(&self) -> Vec<EndpointType> {
        let keys: Vec<EndpointType> = self.data.keys().cloned().collect();
        keys
    }
}

/// Stream of WebSocket messages from one or more endpoints.
pub enum EndpointStream {
    /// A single endpoint stream.
    Single(EndpointType, SplitStream),
    /// Multiple endpoint streams.
    Multiple(SelectAll<SplitStream>),
}

impl From<Endpoint> for EndpointStream {
    fn from(endpoint: Endpoint) -> Self {
        match endpoint {
            // Convert the endpoint into a single stream.
            Endpoint::Public((route, reader)) | Endpoint::User((route, reader)) => {
                EndpointStream::Single(route, reader)
            }
        }
    }
}

impl From<Vec<Endpoint>> for EndpointStream {
    fn from(endpoints: Vec<Endpoint>) -> Self {
        let mut select_all = SelectAll::new();

        // Iterate over each endpoint and convert it into a single stream.
        for endpoint in endpoints {
            match endpoint {
                // Convert the endpoint into a single stream.
                Endpoint::Public((_, reader)) | Endpoint::User((_, reader)) => {
                    select_all.push(reader);
                }
            }
        }

        EndpointStream::Multiple(select_all)
    }
}

impl Stream for EndpointStream {
    type Item = Result<WsMessage, WsError>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.get_mut() {
            EndpointStream::Single(_, stream) => Pin::new(stream).poll_next(cx),
            EndpointStream::Multiple(stream) => Pin::new(stream).poll_next(cx),
        }
    }
}
