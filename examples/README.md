<p align="center">
    <a href="https://crates.io/crates/cbadv" title="crates.io version.">
        <img src="https://img.shields.io/crates/v/cbadv?style=for-the-badge&logoColor=89b4fa&labelColor=11111b&color=89b4fa"
            alt="crates.io version"></a>
    <a href="https://crates.io/crates/cbadv" title="crates.io download counter.">
        <img src="https://img.shields.io/crates/d/cbadv?style=for-the-badge&logoColor=89dceb&labelColor=11111b&color=89dceb"
            alt="crates.io downloads"></a>
    <a href="https://github.com/ohkthx/cbadv-rs" title="Size of the repo!">
        <img src="https://img.shields.io/github/repo-size/Ohkthx/cbadv-rs?style=for-the-badge&logoColor=a6e3a1&labelColor=11111b&color=a6e3a1"
</p>

# Examples

The following examples are for testing and demonstrating the use of the crate. Please review the examples before running them to fully understand what is happening and how they are used. If you have any suggestions, feel free to let me know!


## Account API

Demonstrates how to use the Account API, accessbile at: [account_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/account_api.rs)

**Command**:
  - `cargo run --example account_api --features="config"`

## Product API

Demonstrates how to use the Product API, accessbile at: [product_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/product_api.rs)

**Command**:
  - `cargo run --example product_api --features="config"`

## Fee API

Demonstrates how to use the Fee API, accessbile at: [fee_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/fee_api.rs)

**Command**:
  - `cargo run --example fee_api --features="config"`

## Order API

Demonstrates how to use the Order API, accessbile at: [order_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/order_api.rs)

**Command**:
  - `cargo run --example order_api --features="config"`

## WebSocket API

Demonstrates how to use the WebSocket API, accessbile at: [websocket.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/websocket.rs)

**Command**:
  - `cargo run --example websocket --features="config"`

### WebSocket API - Watch Candles

Demonstrates how to use the Watch Candles via the WebSocket API, accessbile at: [watch_candles.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/watch_candles.rs) These candles are limited to 5 minute granularity and cannot be currently changed (as of 20231019).

**Command**:
  - `cargo run --example watch_candles --features="config"`


## Custom Configurations

Demonstrates how to create a custom configuration file to meet your needs in integration, accessbile at: [custom_config.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/custom_config.rs)

**Command**:
  - `cargo run --example custom_config --features="config"`
