<p align="center">
    <a href="https://crates.io/crates/cbadv" title="View on crates.io">
        <img src="https://img.shields.io/crates/v/cbadv?style=for-the-badge&logoColor=89b4fa&labelColor=11111b&color=89b4fa"
            alt="crates.io version"></a>
    <a href="https://crates.io/crates/cbadv" title="Download counter on crates.io">
        <img src="https://img.shields.io/crates/d/cbadv?style=for-the-badge&logoColor=89dceb&labelColor=11111b&color=89dceb"
            alt="crates.io downloads"></a>
    <a href="https://github.com/Ohkthx/cbadv-rs" title="Repository size">
        <img src="https://img.shields.io/github/repo-size/Ohkthx/cbadv-rs?style=for-the-badge&logoColor=a6e3a1&labelColor=11111b&color=a6e3a1"
            alt="GitHub repo size"></a>
</p>

# cbadv-rs: Coinbase Advanced Trading API Wrapper

Welcome to **cbadv-rs**, a Rust crate for interacting with the Coinbase Advanced Trading API. This library provides easy-to-use interfaces for various Coinbase APIs such as Account, Product, Fee, Order, Portfolio, Public, Sandbox, and WebSocket.

## Table of Contents

- [Examples](#examples)
  - [Account API](#account-api)
  - [Product API](#product-api)
  - [Fee API](#fee-api)
  - [Order API](#order-api)
  - [Portfolio API](#portfolio-api)
  - [Payment API](#payment-api)
  - [Data API](#data-api)
  - [Public API](#public-api)
  - [Sandbox API](#sandbox-api)
  - [WebSocket API](#websocket-api)
    - [Watch Candles (WebSocket API)](#watch-candles-websocket-api)
  - [Custom Configurations](#custom-configurations)
- [Contributing](#contributing)
- [License](#license)

---

## Examples

This section showcases example usage of the crate. Each example demonstrates a different API or functionality. Before running these examples, review the corresponding source code to understand how they work. If you have any suggestions, feel free to open an issue or submit a pull request!

### Account API

Learn how to use the Account API. Example source: [account_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/account_api.rs)

**Run the example**:

```bash
cargo run --example account_api --features="config"
```

---

### Product API

Learn how to use the Product API. Example source: [product_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/product_api.rs)

**Run the example**:

```bash
cargo run --example product_api --features="config"
```

---

### Fee API

Learn how to use the Fee API. Example source: [fee_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/fee_api.rs)

**Run the example**:

```bash
cargo run --example fee_api --features="config"
```

---

### Order API

Learn how to use the Order API. Example source: [order_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/order_api.rs)

**Run the example**:

```bash
cargo run --example order_api --features="config"
```

---

### Portfolio API

Learn how to use the Portfolio API. Example source: [portfolio_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/portfolio_api.rs)

**Run the example**:

```bash
cargo run --example portfolio_api --features="config"
```

---

### Payment API

Learn how to use the Payment API. Example source: [payment_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/payment_api.rs)

**Run the example**:

```bash
cargo run --example payment_api --features="config"
```

---

### Data API

Learn how to use the Data API. Example source: [data_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/data_api.rs)

**Run the example**:

```bash
cargo run --example data_api --features="config"
```

---

### Public API

Learn how to use the Public API. Example source: [public_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/public_api.rs)

**Run the example**:

```bash
cargo run --example public_api --features="config"
```

---

### Sandbox API

Learn how to use the Sandbox API for testing without affecting real accounts. Example source: [sandbox_api.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/sandbox_api.rs)

**Run the example**:

```bash
cargo run --example sandbox_api --features="config"
```

---

### WebSocket API

Learn how to use the WebSocket API for real-time data. Example source: [websocket.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/websocket.rs)

**Run the example**:

```bash
cargo run --example websocket --features="config"
```

#### Watch Candles (WebSocket API)

Learn how to watch candlestick data via the WebSocket API. Currently, only 5-minute granularity is supported (as of 2023-10-19). Example source: [watch_candles.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/watch_candles.rs)

**Run the example**:

```bash
cargo run --example watch_candles --features="config"
```

---

### Custom Configurations

Learn how to create custom configuration files tailored to your integration needs. Example source: [custom_config.rs](https://github.com/Ohkthx/cbadv-rs/tree/main/examples/custom_config.rs)

**Run the example**:

```bash
cargo run --example custom_config --features="config"
```

---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve this crate. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT). See the [LICENSE](LICENSE) file for details.
