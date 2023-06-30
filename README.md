<p align="center">
    <a href="https://ko-fi.com/G2G0J79MY" title="Donate to this project using Ko-fi">
        <img src="https://img.shields.io/badge/donate-black?style=for-the-badge&logo=kofi&logoColor=cba6f7&label=KOFI&labelColor=11111b&color=cba6f7"
            alt="Buy me a coffee! Ko-fi"></a>
    <a href="https://www.blockchain.com/explorer/addresses/btc/bc1q75w3cgutug8qdxw3jlmqnkjlv9alt3jr7ftha0" title="Donate with Bitcoin!">
        <img src="https://img.shields.io/badge/donate-black?style=for-the-badge&logo=bitcoin&logoColor=f38ba8&label=BTC&labelColor=11111b&color=f38ba8"
            alt="Donate with Bitcoin!"></a>
    <a href="https://etherscan.io/address/0x7d75f6a9c021fcc70691fec73368198823fb0f60" title="Donate with Ethereum!">
        <img src="https://img.shields.io/badge/donate-black?style=for-the-badge&logo=ethereum&logoColor=fab387&label=ETH&labelColor=11111b&color=fab387"
            alt="Donate with Ethereum!"></a>
    <a href="https://bscscan.com/address/0x7d75f6a9c021fcc70691fec73368198823fb0f60" title="Donate with BNB (Binance)!">
        <img src="https://img.shields.io/badge/donate-black?style=for-the-badge&logo=binance&logoColor=f9e2af&label=BNB&labelColor=11111b&color=f9e2af"
            alt="Donate with BNB!"></a>
<br>
    <a href="https://crates.io/crates/cbadv" title="crates.io download counter.">
        <img src="https://img.shields.io/crates/d/cbadv?style=for-the-badge&logoColor=89b4fa&labelColor=11111b&color=89b4fa"
            alt="crates.io downloads"></a>
    <a href="https://github.com/ohkthx/xIPL" title="Size of the repo!">
        <img src="https://img.shields.io/github/repo-size/Ohkthx/cbadv-rs?style=for-the-badge&logo=codesandbox&logoColor=89dceb&labelColor=11111b&color=89dceb"
            alt="No data."></a>
    <a href="https://github.com/ohkthx/xIPL" title="Lines of code.">
        <img src="https://img.shields.io/tokei/lines/GitHub/Ohkthx/cbadv-rs?style=for-the-badge&logo=circle&logoColor=a6e3a1&labelColor=11111b&color=a6e3a1"
            alt="No data."></a>
</p>

# cbadv-rs, Coinbase Advanced API

`cbadv-rs` grants access to the **Coinbase Advanced** REST and WebSocket API.

The **cbadv-rs** project is designed to help me get my feet wet in Rust. By no means should others consider using this in the near future, especially with the hopes of making money. This is entirely for testing purposes and I am not responsible for your losses. As note that this project is a work-in-progress and subject to change with time. Some functions, structs, enums, etc may be renamed to adhere to better styling guidelines or optimized for efficiency. With these disclaimers aside and if you enjoy this project, you can choose to credit me with any gains made.

Contributions are encouraged! The API reference can be seen at [Coinbase Advanced API](https://docs.cloud.coinbase.com/advanced-trade-api/reference). If you wish to add this to your project, either use `cargo add cbadv` or add the following line to your dependencies section in **Cargo.toml**:

```toml
[dependencies]
cbadv = { git = "https://github.com/ohkthx/cbadv-rs" }
```

## Features
- Easy-to-use Client.
- Configuration file to hold API Key and API Secret.
- Covers all REST endpoints currently accessible (20230630).
- Covers all WebSocket endpoints currently accessible (20230630).

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

These functions were created to cover common functionality but not initially part of the Coinbase Advanced API. They may require several API requests to accomplish their results.

- **Accounts** [client.account]
  - Get Account by ID [client.account.get_by_id] - Gets an account by the ID (ex BTC or ETH)
- **Orders** [client.order]
  - Get All Orders [client.order.get_all] - Obtains all orders for a product.
  - Cancel All Orders [client.order.cancel_all] - Cancels all OPEN orders for a product.

### TODO

Test all endpoints that are currently untested.

## Configuration

The default configuration is unusable due to the API requiring a Key and Secret. You can create, modify, and delete API Keys and Secrets with this [link](https://www.coinbase.com/settings/api).

Copy the `config.toml.sample` to `config.toml` and add in your API information. The `config.toml` file will automatically be read on launch to access your accounts API information. Unlike the depreciated Coinbase Pro API, there's no longer access to Public API endpoints. All access requires authentication. The key and secret is authentication requirements for HTTP requests to be properly [signed](https://docs.cloud.coinbase.com/advanced-trade-api/docs/rest-api-auth) and accepted by Coinbase.

## Examples

Check above in the **Covered API requests** section for possibly covered examples.

- **Account API**: [account_api_example.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/src/bin/account_api_example.rs)
- **Product API**: [product_api_example.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/src/bin/product_api_example.rs)
- **Fee API**: [fee_api_example.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/src/bin/fee_api_example.rs)
- **Order API**: [order_api_example.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/src/bin/order_api_example.rs)
- **WebSocket API**: [websocket_example.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/src/bin/websocket_example.rs)

## Tips Appreciated!

Wallet addresses are provided below, or click the badges above!
```
Bitcoin (BTC):  bc1q75w3cgutug8qdxw3jlmqnkjlv9alt3jr7ftha0
Ethereum (ETH): 0x7d75f6a9c021fcc70691fec73368198823fb0f60
Binance (BNB):  0x7d75f6a9c021fcc70691fec73368198823fb0f60
```
