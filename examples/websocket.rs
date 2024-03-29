//! # WebSocket API Example, check out the WebSocket API for all functionality.
//!
//! Shows how to:
//! - Connect WebSocket Client.
//! - Setup Listener and parse messages.
//! - Subscribe to channels.
//! - Unsubscribe to channels.

use cbadv::config::{self, BaseConfig};
use cbadv::utils::Result;
use cbadv::websocket::{self, Channel, Message, MessageCallback};
use std::process::exit;
use tokio;

/// Example of an object with an attached callback function for messages.
struct CallbackObject {
    /// Total amount of messages processed.
    total_processed: usize,
}

impl MessageCallback for CallbackObject {
    /// This is used to parse messages. It is passed to the `listen` function to pull Messages out of
    /// the stream.
    fn message_callback(&mut self, msg: Result<Message>) {
        let rcvd = match msg {
            Ok(value) => match value {
                Message::Status(v) => format!("{:?}", v),
                Message::Candles(v) => format!("{:?}", v),
                Message::Ticker(v) => format!("{:?}", v),
                Message::TickerBatch(v) => format!("{:?}", v),
                Message::Level2(v) => format!("{:?}", v),
                Message::User(v) => format!("{:?}", v),
                Message::MarketTrades(v) => format!("{:?}", v),
                Message::Heartbeats(v) => format!("{:?}", v),
                Message::Subscribe(v) => format!("{:?}", v),
            },
            Err(error) => format!("{}", error),
        };

        // Using the callback objects properties.
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

    // Create a client to interact with the API.
    let mut client = websocket::from_config(&config);

    // Callback Object
    let cb_obj: CallbackObject = CallbackObject { total_processed: 0 };

    // Connect to the websocket, a subscription needs to be sent within 5 seconds.
    // If a subscription is not sent, Coinbase will close the connection.
    let reader = client.connect().await.unwrap();
    let listener = tokio::spawn(websocket::listener_with(reader, cb_obj));

    // Products of interest.
    let products = vec!["BTC-USD".to_string(), "ETH-USD".to_string()];

    // Subscribe to user orders.
    client.sub(Channel::USER, &products).await.unwrap();

    // Get updates (subscribe) on products and currencies.
    client.sub(Channel::CANDLES, &products).await.unwrap();

    // Heartbeats is a great way to keep a connection alive and not timeout.
    client.sub(Channel::HEARTBEATS, &vec![]).await.unwrap();

    // Stop obtaining (unsubscribe) updates on products and currencies.
    client.unsub(Channel::STATUS, &products).await.unwrap();

    // Passes the parser callback and listens for messages.
    listener.await.unwrap();
}
