//! # WebSocket User API Example, check out the WebSocket API for all functionality.
//!
//! Shows how to:
//! - Connect WebSocket Client.
//! - Setup Listener and parse messages.
//! - Subscribe to channels.
//! - Unsubscribe to channels.

use std::process::exit;

use cbadv::config::{self, BaseConfig};
use cbadv::models::websocket::{Channel, Message};
use cbadv::traits::MessageCallback;
use cbadv::types::CbResult;
use cbadv::{async_trait, WebSocketClientBuilder};

/// Example of an object with an attached callback function for messages.
struct CallbackObject {
    /// Total amount of messages processed.
    total_processed: usize,
}

#[async_trait]
impl MessageCallback for CallbackObject {
    /// This is used to parse messages. It is passed to the `listen` function to pull Messages out of
    /// the stream.
    async fn message_callback(&mut self, msg: CbResult<Message>) {
        let rcvd = match msg {
            Ok(message) => format!("{message:?}"), // Leverage Debug for all Message variants
            Err(error) => format!("Error: {error}"), // Handle WebSocket errors
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
                println!("File exists, {err}");
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
        .auto_reconnect(true)
        .max_retries(20)
        .build()
        .map_err(|e| {
            eprintln!("!ERROR! {e}");
            exit(1);
        })
        .unwrap();

    // Callback Object.
    let callback = CallbackObject { total_processed: 0 };

    // Connect to the websocket, a subscription needs to be sent within 5 seconds.
    // If a subscription is not sent, Coinbase will close the connection.
    let readers = client
        .connect()
        .await
        .expect("Could not connect to WebSocket.");

    // Basic subscriptions.
    client.subscribe(&Channel::Heartbeats, &[]).await.unwrap();
    client.subscribe(&Channel::User, &[]).await.unwrap();

    let listener = tokio::spawn(async move {
        client.listen(readers, callback).await;
    });

    // Passes the parser callback and listens for messages.
    listener.await.unwrap();
}
