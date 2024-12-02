//! # Utilities and supporting functions.
//!
//! `utils` is a collection of helpful tools that may be required throughout the rest of the API.

use std::fmt::{Display, Write};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;

use crate::models::websocket::Message;
use crate::traits::MessageCallback;
use crate::types::CbResult;

/// Builds the URL Query to be sent to the API.
pub(crate) struct QueryBuilder {
    query: String,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryBuilder {
    /// Constructs a new `QueryBuilder`.
    pub(crate) fn new() -> Self {
        Self {
            query: String::new(),
        }
    }

    /// Adds a key-value pair to the query string.
    pub(crate) fn push<T: Display>(mut self, key: &str, value: T) -> Self {
        if !self.query.is_empty() {
            self.query.push('&');
        }

        write!(self.query, "{key}={value}").unwrap();
        self
    }

    /// Adds a key-value pair to the query string if the value is present.
    pub(crate) fn push_optional<T: Display>(self, key: &str, value: &Option<T>) -> Self {
        if let Some(v) = value {
            self.push(key, v)
        } else {
            self
        }
    }

    /// Adds multiple key-value pairs from an optional vector.
    pub(crate) fn push_optional_vec<T: Display>(
        mut self,
        key: &str,
        values: &Option<Vec<T>>,
    ) -> Self {
        if let Some(values) = values {
            for value in values {
                self = self.push(key, value);
            }
        }
        self
    }

    /// Builds and returns the final query string.
    pub(crate) fn build(self) -> String {
        self.query
    }
}

type BoxCallback =
    Box<dyn Fn(CbResult<Message>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Used to wrap callback functions for the WebSocket Client's `listen()` function..
pub struct FunctionCallback {
    callback: Arc<BoxCallback>,
}

impl FunctionCallback {
    /// Creates a new `FunctionCallback` from an asynchronous function.
    ///
    /// # Arguments
    ///
    /// * `async_fn` - The asynchronous function to be called when a message is received.
    pub fn from_async<F, Fut>(async_fn: F) -> Self
    where
        F: Fn(CbResult<Message>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let callback = move |msg: CbResult<Message>| -> Pin<Box<dyn Future<Output = ()> + Send>> {
            let fut = async_fn(msg);
            Box::pin(fut)
        };

        Self {
            callback: Arc::new(Box::new(callback)),
        }
    }

    /// Creates a new `FunctionCallback` from a synchronous function.
    ///
    /// # Arguments
    ///
    /// * `sync_fn` - The synchronous function to be called when a message is received.
    pub fn from_sync<F>(sync_fn: F) -> Self
    where
        F: Fn(CbResult<Message>) + Send + Sync + 'static,
    {
        let sync_fn = Arc::new(sync_fn);

        let callback = {
            let sync_fn = Arc::clone(&sync_fn);
            move |msg: CbResult<Message>| -> Pin<Box<dyn Future<Output = ()> + Send>> {
                let sync_fn = Arc::clone(&sync_fn);
                Box::pin(async move {
                    (sync_fn)(msg);
                })
            }
        };

        Self {
            callback: Arc::new(Box::new(callback)),
        }
    }
}

#[async_trait]
impl MessageCallback for FunctionCallback {
    async fn message_callback(&mut self, msg: CbResult<Message>) {
        let callback = Arc::clone(&self.callback);
        (callback)(msg).await;
    }
}
