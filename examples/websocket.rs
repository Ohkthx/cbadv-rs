//! # WebSocket API Example, check out the WebSocket API for all functionality.
//!
//! Shows how to:
//! - Connect WebSocket Client.
//! - Setup Listener and parse messages.
//! - Subscribe to channels.
//! - Unsubscribe to channels.

use std::process::exit;
use std::time::{Duration, Instant};

use cbadv::models::websocket::{Channel, EndpointStream, Message};
use cbadv::types::CbResult;
use cbadv::WebSocketClientBuilder;

/// This is used to parse messages. It is passed to the `listen` function to pull Messages out of
/// the stream.
fn message_action(msg: CbResult<Message>) {
    let rcvd = match msg {
        Ok(message) => format!("{message:?}"), // Leverage Debug for all Message variants
        Err(error) => format!("Error: {error}"), // Handle WebSocket errors
    };

    // Update the callback object's properties and log the message.
    println!("{rcvd}\n");
}

#[tokio::main]
async fn main() {
    // Create a client that can only access private streams.
    let mut client = WebSocketClientBuilder::new()
        .auto_reconnect(true)
        .max_retries(20)
        .build()
        .map_err(|e| {
            eprintln!("!ERROR! {e}");
            exit(1);
        })
        .unwrap();

    // Connect to the websocket, a subscription needs to be sent within 5 seconds.
    // If a subscription is not sent, Coinbase will close the connection.
    let readers = client
        .connect()
        .await
        .expect("Could not connect to WebSocket");

    // Products of interest.
    let products = vec!["BTC-USDC".to_string(), "ETH-USDC".to_string()];

    // Heartbeats is a great way to keep a connection alive and not timeout.
    client.subscribe(&Channel::Heartbeats, &[]).await.unwrap();

    // Get updates (subscribe) on products and currencies.
    client
        .subscribe(&Channel::Candles, &products)
        .await
        .unwrap();

    // Get updates (subscribe) on products and currencies.
    client.subscribe(&Channel::Level2, &products).await.unwrap();

    // Stop obtaining (unsubscribe) updates on products and currencies.
    client
        .unsubscribe(&Channel::Status, &products)
        .await
        .unwrap();

    let mut count = 0;
    const TICK_RATE: u64 = 1000 / 60;
    let mut last_tick = Instant::now();
    let mut stream: EndpointStream = readers.into();

    loop {
        // Fetch messages from the WebSocket stream.
        client.fetch_sync(&mut stream, 100, |msg| {
            count += 1;
            print!("{count}: ");
            message_action(msg);
        });

        // Calculate the time since the last tick and sleep for the remaining time to hit the tick rate.
        let last_tick_ms = last_tick.elapsed().as_millis();
        let timeout = match u64::try_from(last_tick_ms) {
            Ok(ms) => TICK_RATE.saturating_sub(ms),
            Err(why) => {
                eprintln!("Conversion error: {why}");
                TICK_RATE
            }
        };

        // Sleep for the remaining time to hit the tick rate. Prevent busy loop.
        tokio::time::sleep(Duration::from_millis(timeout)).await;
        last_tick = Instant::now();
    }
}
