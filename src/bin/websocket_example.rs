use cbadv::utils::Result;
use cbadv::websocket::{Channel, Client, Message};
use cbadv::{config, websocket};
use tokio;

/// This is used to parse messages. It is passed to the `listen` function to pull Messages out of
/// the stream.
fn parser_callback(msg: Result<Message>) {
    let rcvd = match msg {
        Ok(value) => match value {
            Message::Status(v) => format!("{:?}", v),
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

    println!("> {}\n", rcvd);
}

#[tokio::main]
async fn main() {
    // Load the configuration file.
    let config = config::load("config.toml").unwrap();

    // Create a client to interact with the API.
    let mut client = Client::new(&config.cb_api_key, &config.cb_api_secret);

    // Connect to the websocket, a subscription needs to be sent within 5 seconds.
    // If a subscription is not sent, Coinbase will close the connection.
    let reader = client.connect().await.unwrap();
    let future = tokio::spawn(websocket::listener(reader, parser_callback));

    // Products of interest.
    let products = vec!["BTC-USD".to_string(), "ETH-USD".to_string()];

    // Subscribe to user orders.
    client.subscribe(Channel::USER, &products).await.unwrap();

    // Get updates on products and currencies.
    client.subscribe(Channel::STATUS, &products).await.unwrap();

    // Heartbeats is a great way to keep a connection alive and not timeout.
    client
        .subscribe(Channel::HEARTBEATS, &vec![])
        .await
        .unwrap();

    // Passes the parser callback and listens for messages.
    future.await.unwrap().unwrap();
}
