//! # WebSocket API Example, check out the WebSocket API for all functionality.
//!
//! Shows how to:
//! - Connect WebSocket Client.
//! - Setup Listener and parse messages.
//! - Subscribe to channels.
//! - Unsubscribe to channels.

use std::process::exit;

use cbadv::models::websocket::{Channel, EndpointType, Message};
use cbadv::types::CbResult;
use cbadv::{FunctionCallback, WebSocketClientBuilder};

/// This is used to parse messages. It is passed to the `listen` function to pull Messages out of
/// the stream.
fn message_callback(msg: CbResult<Message>) {
    let rcvd = match msg {
        Ok(message) => format!("{:?}", message), // Leverage Debug for all Message variants
        Err(error) => format!("Error: {}", error), // Handle WebSocket errors
    };

    // Update the callback object's properties and log the message.
    println!("{}\n", rcvd);
}

#[tokio::main]
async fn main() {
    // Create a client that can only access private streams.
    let mut client = WebSocketClientBuilder::new()
        .auto_reconnect(true)
        .max_retries(20)
        .build()
        .map_err(|e| {
            eprintln!("!ERROR! {}", e);
            exit(1);
        })
        .unwrap();

    // Assign the callback function to an object.
    let callback = FunctionCallback::from_sync(message_callback);

    // Connect to the websocket, a subscription needs to be sent within 5 seconds.
    // If a subscription is not sent, Coinbase will close the connection.
    let mut readers = client
        .connect()
        .await
        .expect("Could not connect to WebSocket");

    let public = readers
        .take_endpoint(&EndpointType::Public)
        .expect("Could not get public reader");

    let listened_client = client.clone();
    let listener = tokio::spawn(async move {
        let mut listened_client = listened_client;
        listened_client.listen(public, callback).await;
    });

    // Products of interest.
    let products = vec!["BTC-USD".to_string(), "ETH-USD".to_string()];

    // Heartbeats is a great way to keep a connection alive and not timeout.
    client.sub(&Channel::Heartbeats, &[]).await.unwrap();

    // Get updates (subscribe) on products and currencies.
    client.sub(&Channel::Candles, &products).await.unwrap();

    // Stop obtaining (unsubscribe) updates on products and currencies.
    client.unsub(&Channel::Status, &products).await.unwrap();

    // Passes the parser callback and listens for messages.
    listener.await.unwrap();
}
