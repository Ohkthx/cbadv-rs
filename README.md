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
            alt="GitHub repo size"></a>
</p>

---

# Asynchronous CoinBase Advanced API

The **cbadv-rs** crate provides high-performance, asynchronous access to the Coinbase Advanced REST and WebSocket APIs. This project includes features to securely configure API keys and secrets, making it suitable for developers seeking robust API integration.

This project is currently a work-in-progress. While the crate is usable, API changes or updates may occur as Coinbase Advanced evolves. Please thoroughly test before using in production.

To get started, add this crate to your project using `cargo add cbadv` or manually add the following to your `Cargo.toml`:

```toml
[dependencies]
cbadv = { git = "https://github.com/ohkthx/cbadv-rs", branch = "main" }
```

---

## Table of Contents

- [Features](#features)
- [Documentation](#documentation)
- [Configuration](#configuration)
- [Examples](#examples)
- [API Coverage](#api-coverage)
  - [WebSocket API](#websocket-api)
  - [REST API](#rest-api)
  - [Extended Features](#extended-features)
- [TODO](#todo)
- [Contributing](#contributing)
- [Tips Appreciated!](#tips-appreciated)

---

## Features

- Asynchronous API access with support for REST and WebSocket protocols.
- Authenticated and Public REST Clients.
- Convenient configuration file support for API keys (`features = ["config"]`).
- Comprehensive coverage of all accessible REST and WebSocket endpoints (as of **20231206**).
- Numerous examples for seamless integration and testing.

---

## Documentation

Full API documentation is available at [docs.rs](https://docs.rs/cbadv/latest/cbadv/). You can also find helpful information on [crates.io](https://crates.io/crates/cbadv).

---

## API Coverage

### WebSocket API

Client: `use cbadv::WebSocketClient`

- **Authentication**: `client.connect`
- **Subscribe**: `client.subscribe` or `client.sub`
- **Unsubscribe**: `client.unsubscribe` or `client.unsub`
- **Channels Supported**:
  - `Channel::STATUS`: Status
  - `Channel::CANDLES`: Candles
  - `Channel::TICKER`: Ticker
  - `Channel::TICKER_BATCH`: Ticker Batch
  - `Channel::LEVEL2`: Level 2 Market Data
  - `Channel::USER`: User-Specific Updates
  - `Channel::MARKET_TRADES`: Market Trades

### REST API

Clients: `use cbadv::RestClient` and `use cbadv::PublicRestClient`

- **Accounts (`client.account`)**:
  - List Accounts: `client.account.get_bulk`
  - Get Account: `client.account.get`
- **Products (`client.product`)**:
  - Get Best Bid/Ask: `client.product.best_bid_ask`
  - Get Product Book: `client.product.product_book`
  - List Products: `client.product.get_bulk`
  - Get Product Details: `client.product.get`
  - Get Product Candles: `client.product.candles`
  - Get Market Trades (Ticker): `client.product.ticker`
- **Orders (`client.order`)**:
  - Create Order:
    - Market IOC (untested): `client.order.create_market`
    - Limit GTC: `client.order.create_limit_gtc`
    - Limit GTD (untested): `client.order.create_limit_gtd`
    - Stop Limit GTC (untested): `client.order.create_stop_limit_gtc`
    - Stop Limit GTD (untested): `client.order.create_stop_limit_gtd`
  - Edit Order: `client.order.edit`
  - Preview Order Edit: `client.order.preview_edit`
  - Cancel Order: `client.order.cancel`
  - List Orders: `client.order.get_bulk`
  - List Fills (untested): `client.order.fills`
  - Get Order: `client.order.get`
- **Fees (`client.fee`)**:
  - Get Transaction Summary: `client.fee.get`
- **Converts (`client.convert`)**:
  - Create Quote (untested): `client.convert.create_quote`
  - Get Convert (untested): `client.convert.get`
  - Commit Convert (untested): `client.convert.commit`
- **Portfolios (`client.portfolio`)**:
  - Create Portfolio: `client.portfolio.create`
  - List Portfolios: `client.portfolio.get_all`
  - Get Portfolio Breakdown: `client.portfolio.get`
  - Edit Portfolio: `client.portfolio.edit`
  - Delete Portfolio: `client.portfolio.delete`
  - Move Funds (untested): `client.portfolio.move_funds`
- **Public (`client.public`)**:
  - Get API Unix Server Time: `client.public.server_time`
  - Get Product Book: `client.public.product_book`
  - List Products: `client.public.products`
  - Get Product: `client.public.product`
  - Get Product Candles: `client.public.candles`
  - Get Product Ticker: `client.public.ticker`

---

## Configuration

To enable the configuration feature, include it in your `Cargo.toml`:

```toml
[dependencies]
cbadv = { version = "*", features = ["config"] }
```

Set up `config.toml` with your API credentials. A sample file can be found at `config.toml.sample`. See the [custom configuration example](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/custom_config.rs) for advanced setups.

---

## Examples

Explore the [examples directory](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/) for usage scenarios.

---

## TODO

- Test unverified endpoints.
- Expand examples to cover more advanced cases.

---

## Contributing

Contributions are welcome! Fork the repository, create a feature branch, and submit a pull request.

---

## Tips Appreciated!

Support this project via cryptocurrency donations:

**Ethereum (ETH):** 0x7d75f6a9c021fcc70691fec73368198823fb0f60  
**Bitcoin (BTC):** bc1q75w3cgutug8qdxw3jlmqnkjlv9alt3jr7ftha0  
**Binance (BNB):** 0x7d75f6a9c021fcc70691fec73368198823fb0f60
