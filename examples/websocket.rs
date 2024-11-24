//! # WebSocket API Example, check out the WebSocket API for all functionality.
//!
//! Shows how to:
//! - Connect WebSocket Client.
//! - Setup Listener and parse messages.
//! - Subscribe to channels.
//! - Unsubscribe to channels.

use std::process::exit;
use std::sync::Arc;

use cbadv::config::{self, BaseConfig};
use cbadv::traits::MessageCallback;
use cbadv::types::CbResult;
use cbadv::ws::{Channel, Message};
use cbadv::{WebSocketClient, WebSocketClientBuilder};
use futures::lock::Mutex;

/// Example of an object with an attached callback function for messages.
struct CallbackObject {
    /// Total amount of messages processed.
    total_processed: usize,
}

impl MessageCallback for CallbackObject {
    /// This is used to parse messages. It is passed to the `listen` function to pull Messages out of
    /// the stream.
    fn message_callback(&mut self, msg: CbResult<Message>) {
        let rcvd = match msg {
            Ok(message) => format!("{:?}", message), // Leverage Debug for all Message variants
            Err(error) => format!("Error: {}", error), // Handle WebSocket errors
        };

        // Update the callback object's properties and log the message.
        self.total_processed += 1;
        println!("{:<5}> {}\n", self.total_processed, rcvd);
    }
}

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config: BaseConfig = match config::load("config.toml") {
        Ok(c) => c,
        Err(err) => {
            println!("Could not load configuration file.");
            if config::exists("config.toml") {
                println!("File exists, {}", err);
                exit(1);
            }

            // Create a new configuration file.
            config::create_base_config("config.toml").unwrap();
            println!("Empty configuration file created, please update it.");
            exit(1);
        }
    };

    let mut client = WebSocketClientBuilder::new()
        .with_config(&config)
        .build()
        .map_err(|e| {
            eprintln!("!ERROR! {}", e);
            exit(1);
        })
        .unwrap();

    // Callback Object.
    let cb_obj = Arc::new(Mutex::new(CallbackObject { total_processed: 0 }));

    // Connect to the websocket, a subscription needs to be sent within 5 seconds.
    // If a subscription is not sent, Coinbase will close the connection.
    let readers = client
        .connect()
        .await
        .expect("Could not connect to WebSocket");

    let public = readers.public.expect("Could not get public reader");
    let listener = tokio::spawn(WebSocketClient::listen_reader_trait(public, cb_obj));

    // Products of interest.
    let products = vec!["BTC-USD".to_string(), "ETH-USD".to_string()];

    // Heartbeats is a great way to keep a connection alive and not timeout.
    client.sub(Channel::Heartbeats, &[]).await.unwrap();

    // Subscribe to user orders.
    client.sub(Channel::User, &products).await.unwrap();

    // Get updates (subscribe) on products and currencies.
    client.sub(Channel::Candles, &products).await.unwrap();

    // Stop obtaining (unsubscribe) updates on products and currencies.
    client.unsub(Channel::Status, &products).await.unwrap();

    // Passes the parser callback and listens for messages.
    listener.await.unwrap();
}
