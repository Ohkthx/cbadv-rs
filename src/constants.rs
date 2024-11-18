//! All constants for various API Endpoints

/// Root resource for the API
pub(crate) const API_ROOT_URI: &str = "api.coinbase.com";
pub(crate) const API_SANDBOX_ROOT_URI: &str = "api-public.sandbox.exchange.coinbase.com";
pub(crate) const CRATE_USER_AGENT: &str = "cbadv/Rust";

/// Accounts API constants
pub(crate) mod accounts {
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/accounts";
}

/// Convert API constants
pub(crate) mod convert {
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/convert";
    pub(crate) const QUOTE_ENDPOINT: &str = "/api/v3/brokerage/convert/quote";
}

/// Fees API constants
pub(crate) mod fees {
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/transaction_summary";
}

/// Orders API constants
pub(crate) mod orders {
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/orders";
    pub(crate) const CANCEL_BATCH_ENDPOINT: &str = "/api/v3/brokerage/orders/batch_cancel";
    pub(crate) const EDIT_ENDPOINT: &str = "/api/v3/brokerage/orders/edit";
    pub(crate) const EDIT_PREVIEW_ENDPOINT: &str = "/api/v3/brokerage/orders/edit_preview";
    pub(crate) const BATCH_ENDPOINT: &str = "/api/v3/brokerage/orders/historical/batch";
    pub(crate) const FILLS_ENDPOINT: &str = "/api/v3/brokerage/orders/historical/fills";
}

/// Products API constants
pub(crate) mod products {
    pub(crate) const CANDLE_MAXIMUM: u64 = 350;
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/products";
    pub(crate) const BID_ASK_ENDPOINT: &str = "/api/v3/brokerage/best_bid_ask";
    pub(crate) const PRODUCT_BOOK_ENDPOINT: &str = "/api/v3/brokerage/product_book";
}

/// Public API constants
pub(crate) mod public {
    pub(crate) const SERVERTIME_ENDPOINT: &str = "/api/v3/brokerage/time";
    pub(crate) const PRODUCT_BOOK_ENDPOINT: &str = "/api/v3/brokerage/market/product_book";
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/market/products";
}

/// REST API constants
pub(crate) mod rest {
    pub(crate) const SERVICE: &str = "retail_rest_api_proxy";
}

/// Websocket API constants
pub(crate) mod websocket {
    pub(crate) const RESOURCE_ENDPOINT: &str = "wss://advanced-trade-ws.coinbase.com";

    /// Granularity of Candles from the WebSocket Candle subscription.
    /// NOTE: This is a restriction by CoinBase and cannot be currently changed (20240125)
    pub(crate) const GRANULARITY: u64 = 300;
    pub(crate) const SERVICE: &str = "public_websocket_api";
}

/// Amount of tokens per second refilled.
pub(crate) mod ratelimits {
    pub(crate) const REST_REFRESH_RATE: f64 = 30.0;
    pub(crate) const WEBSOCKET_REFRESH_RATE: f64 = 750.0;
}
