//! All constants for various API Endpoints

/// Root resource for the API
pub(crate) const API_ROOT_URI: &str = "api.coinbase.com";
pub(crate) const API_SANDBOX_ROOT_URI: &str = "api-sandbox.coinbase.com";
pub(crate) const CRATE_USER_AGENT: &str = "cbadv/Rust";

/// Accounts API constants
pub(crate) mod accounts {
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/accounts";
    pub(crate) const LIST_ACCOUNT_MAXIMUM: u32 = 250;
}

/// Convert API constants
pub(crate) mod convert {
    pub(crate) const QUOTE_ENDPOINT: &str = "/api/v3/brokerage/convert/quote";
    pub(crate) const TRADE_ENDPOINT: &str = "/api/v3/brokerage/convert/trade";
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
    pub(crate) const CREATE_PREVIEW_ENDPOINT: &str = "/api/v3/brokerage/orders/preview";
    pub(crate) const EDIT_PREVIEW_ENDPOINT: &str = "/api/v3/brokerage/orders/edit_preview";
    pub(crate) const BATCH_ENDPOINT: &str = "/api/v3/brokerage/orders/historical/batch";
    pub(crate) const FILLS_ENDPOINT: &str = "/api/v3/brokerage/orders/historical/fills";
    pub(crate) const CLOSE_POSITION_ENDPOINT: &str = "/api/v3/brokerage/orders/close_position";
}

/// Portfolios API constants
pub(crate) mod portfolios {
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/portfolios";
    pub(crate) const MOVE_FUNDS_ENDPOINT: &str = "/api/v3/brokerage/portfolios/move_funds";
}

/// Products API constants
pub(crate) mod products {
    pub(crate) const CANDLE_MAXIMUM: u32 = 350;
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/products";
    pub(crate) const BID_ASK_ENDPOINT: &str = "/api/v3/brokerage/best_bid_ask";
    pub(crate) const PRODUCT_BOOK_ENDPOINT: &str = "/api/v3/brokerage/product_book";
}

/// Payment API constants
pub(crate) mod payments {
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/payment_methods";
}

/// Data API constants
pub(crate) mod data {
    pub(crate) const KEY_PERMISSIONS_ENDPOINT: &str = "/api/v3/brokerage/key_permissions";
}

/// Public API constants
pub(crate) mod public {
    pub(crate) const SERVERTIME_ENDPOINT: &str = "/api/v3/brokerage/time";
    pub(crate) const PRODUCT_BOOK_ENDPOINT: &str = "/api/v3/brokerage/market/product_book";
    pub(crate) const RESOURCE_ENDPOINT: &str = "/api/v3/brokerage/market/products";
}

/// Websocket API constants
pub(crate) mod websocket {
    pub(crate) const PUBLIC_ENDPOINT: &str = "wss://advanced-trade-ws.coinbase.com";
    pub(crate) const SECURE_ENDPOINT: &str = "wss://advanced-trade-ws-user.coinbase.com";
}

/// Amount of tokens per second refilled.
pub(crate) mod ratelimits {
    pub(crate) const SECURE_REST_REFRESH_RATE: f64 = 30.0;
    pub(crate) const PUBLIC_REST_REFRESH_RATE: f64 = 10.0;
    pub(crate) const SECURE_WEBSOCKET_REFRESH_RATE: f64 = 750.0;
    pub(crate) const PUBLIC_WEBSOCKET_REFRESH_RATE: f64 = 8.0;
}
