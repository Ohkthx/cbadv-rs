<p align="center">
    <a href="https://github.com/Ohkthx/cbadv-rs#tips-appreciated" title="Donate with Bitcoin!">
        <img src="https://img.shields.io/badge/donate-black?style=for-the-badge&logo=bitcoin&logoColor=f38ba8&label=BITCOIN&labelColor=11111b&color=f38ba8"
            alt="Donate with Bitcoin!"></a>
    <a href="https://github.com/Ohkthx/cbadv-rs#tips-appreciated" title="Donate with Ethereum!">
        <img src="https://img.shields.io/badge/donate-black?style=for-the-badge&logo=ethereum&logoColor=fab387&label=ETHEREUM&labelColor=11111b&color=fab387"
            alt="Donate with Ethereum!"></a>
    <a href="https://github.com/Ohkthx/cbadv-rs#tips-appreciated" title="Donate with BNB (Binance)!">
        <img src="https://img.shields.io/badge/donate-black?style=for-the-badge&logo=binance&logoColor=f9e2af&label=BINANCE&labelColor=11111b&color=f9e2af"
            alt="Donate with BNB!"></a>
<br>
    <a href="https://crates.io/crates/cbadv" title="crates.io version.">
        <img src="https://img.shields.io/crates/v/cbadv?style=for-the-badge&logoColor=89b4fa&labelColor=11111b&color=89b4fa"
            alt="crates.io version"></a>
    <a href="https://crates.io/crates/cbadv" title="crates.io download counter.">
        <img src="https://img.shields.io/crates/d/cbadv?style=for-the-badge&logoColor=89dceb&labelColor=11111b&color=89dceb"
            alt="crates.io downloads"></a>
    <a href="https://github.com/ohkthx/cbadv-rs" title="Size of the repo!">
        <img src="https://img.shields.io/github/repo-size/Ohkthx/cbadv-rs?style=for-the-badge&logoColor=a6e3a1&labelColor=11111b&color=a6e3a1"
</p>

# Asynchronous CoinBase Advanced API

The objective of this crate is to grant asynchronous access to the **CoinBase Advanced** REST and WebSocket API. Included with the crate are ways to organize your API Keys and Secrets inside of a configuration file.

This project is current a work-in-progress. Changes between versions can vary greatly as this API becomes more refined and adapts to CoinBase Advances changing state. I ask you to understand that I am not liable for any issues you may encounter while this project is in this state and encourage you to verify and test before committing to using this yourself in a serious manner.

Contributions are encouraged! The API reference can be seen at [CoinBase Advanced API](https://docs.cloud.coinbase.com/advanced-trade-api/reference). If you wish to add this to your project, either use `cargo add cbadv` or add the following line to your dependencies section in **Cargo.toml**:

```toml
[dependencies]
cbadv = { git = "https://github.com/ohkthx/cbadv-rs" }
```

## Features
- Asynchronous.
- Easy-to-use REST and WebSocket clients.
- Configuration file to hold API Key and API Secret. `features = ["config"]`
- Covers all REST endpoints currently accessible (20230705).
- Covers all WebSocket endpoints currently accessible (20230705).

## Documentation

Most of the documentation can be accessed by clicking the following link: [docs.rs](https://docs.rs/cbadv/latest/cbadv/). That documentation is automatically generated and also accessible from [crates.io](https://crates.io/crates/cbadv).

### Covered API requests

#### WebSocket API

Client: `use cbadv::websocket::Client`

- **Authentication** [client.connect]
- **Subscribe** [client.subscribe]
- **Unsubscribe** [client.unsubscribe]
- **Channels Supported**
  - Status [Channel::STATUS]
  - Candles [Channel::CANDLES]
  - Ticker [Channel::TICKER]
  - Ticker Batch [Channel::TICKER_BATCH]
  - Level2 [Channel::LEVEL2]
  - User [Channel::USER]
  - Market Trades [Channel::MARKET_TRADES]


#### REST API

Client: `use cbadv::rest::Client`

- **Accounts [client.account]**
  - List Accounts [client.account.get_bulk]
  - Get Account [client.account.get]
- **Products [client.product]**
  - Get Best Bid / Ask [client.product.best_bid_ask]
  - Get Product Book [client.product.product_book]
  - List Products [client.product.get_bulk]
  - Get Product [client.product.get]
  - Get Product Candles [client.product.candles]
  - Get Market Trades (Ticker) [client.product.ticker]
- **Orders [client.order]**
  - Create Order 
    - Market IOC (untested) [client.order.create_market]
    - Limit GTC [client.order.create_limit_gtc]
    - Limit GTD (untested) [client.order.create_limit_gtd]
    - Stop Limit GTC (untested) [client.order.create_stop_limit_gtc]
    - Stop Limit GTD (untested) [client.order.create_stop_limit_gtd]
  - Cancel Orders [client.order.cancel]
  - List Orders [client.order.get_bulk]
  - List Fills (untested) [client.order.fills]
  - Get Order [client.order.get]
- **Fees [client.fee]**
  - Get Transaction Summary [client.fee.get]

### Added Requests

These functions were created to cover common functionality but not initially part of the CoinBase Advanced API. They may require several API requests to accomplish their results.

- **Accounts** [client.account]
  - Get Account by ID [client.account.get_by_id] - Gets an account by the ID (ex BTC or ETH)
  - Get All [client.account.get_all] - Gets all accounts.
- **Products** [client.product]
  - Get Candles (Extended) [client.product.candles_ext] - Obtains more than the limit (300) candles.
- **Orders** [client.order]
  - Get All Orders [client.order.get_all] - Obtains all orders for a product.
  - Cancel All Orders [client.order.cancel_all] - Cancels all OPEN orders for a product.

### TODO

Test all endpoints that are currently untested.

## Configuration Feature

Configuration requires you to add the 'config' feature (`features = ["config"]`) to your `Cargo.toml`. The default configuration is unusable due to the API requiring a Key and Secret. You can create, modify, and delete API Keys and Secrets with this [link](https://www.coinbase.com/settings/api).

Copy the `config.toml.sample` to `config.toml` and add in your API information. The `config.toml` file will automatically be read on launch to access your accounts API information. Unlike the depreciated CoinBase Pro API, there's no longer access to Public API endpoints. All access requires authentication. The key and secret is authentication requirements for HTTP requests to be properly [signed](https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-auth) and accepted by CoinBase.

Example of enabled `config` feature in `Cargo.toml`.

```toml
[dependencies]
cbadv = { version = "*", features = ["config"] }
```

## Examples

Check above in the **Covered API requests** section for possibly covered examples.

- **Account API**: [account_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/account_api.rs)
  - `cargo run --example account_api --features="config"`
- **Product API**: [product_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/product_api.rs)
  - `cargo run --example product_api --features="config"`
- **Fee API**: [fee_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/fee_api.rs)
  - `cargo run --example fee_api --features="config"`
- **Order API**: [order_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/order_api.rs)
  - `cargo run --example order_api --features="config"`
- **WebSocket API**: [websocket.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/websocket.rs)
  - `cargo run --example websocket --features="config"`

## Tips Appreciated!

Wallet addresses are provided below, or click the badges above!
```
Ethereum (ETH): 0x7d75f6a9c021fcc70691fec73368198823fb0f60
Bitcoin (BTC):  bc1q75w3cgutug8qdxw3jlmqnkjlv9alt3jr7ftha0
Binance (BNB):  0x7d75f6a9c021fcc70691fec73368198823fb0f60
```
