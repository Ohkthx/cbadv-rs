use serde::{Deserialize, Serialize};

use crate::order::OrderSide;
use crate::product::{Candle, ProductType};
use crate::utils::deserialize_numeric;

use super::Level2Side;

#[derive(Deserialize, Debug)]
pub struct Level2Update {
    pub side: Level2Side,
    pub event_time: String,
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price_level: f64,
    #[serde(deserialize_with = "deserialize_numeric")]
    pub new_quantity: f64,
}

#[derive(Deserialize, Debug, Default)]
pub struct SubscribeUpdate {
    #[serde(default)]
    pub status: Vec<String>,
    #[serde(default)]
    pub ticker: Vec<String>,
    #[serde(default)]
    pub ticker_batch: Vec<String>,
    #[serde(default)]
    pub level2: Option<Vec<String>>,
    #[serde(default)]
    pub user: Option<Vec<String>>,
    #[serde(default)]
    pub market_trades: Option<Vec<String>>,
    #[serde(default)]
    pub heartbeats: Option<Vec<String>>,
}

/// Represents a Product received from the Websocket API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductUpdate {
    /// Type of the product.
    pub product_type: ProductType,
    /// ID of the product.
    pub id: String,
    /// Symbol of the base currency.
    pub base_currency: String,
    /// Symbol of the quote currency.
    pub quote_currency: String,
    /// Minimum amount base value can be increased or decreased at once.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub base_increment: f64,
    /// Minimum amount quote value can be increased or decreased at once.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub quote_increment: f64,
    /// Name of the product.
    pub display_name: String,
    /// Status of the product.
    pub status: String,
    /// Additional status message.
    pub status_message: String,
    /// Minimum amount of funds.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub min_market_funds: f64,
}

/// Represents a Market Trade received from the Websocket API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketTradesUpdate {
    /// Trade identity.
    pub trade_id: String,
    /// ID of the product.
    pub product_id: String,
    /// Price of the product.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price: f64,
    /// Size for the trade.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub size: f64,
    /// Side: BUY or SELL.
    pub side: OrderSide,
    /// Time for the market trade.
    pub time: String,
}

/// Represents a Candle update received from the Websocket API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandleUpdate {
    /// Product ID (Pair, ex 'BTC-USD')
    pub product_id: String,
    /// Candle for the update.
    #[serde(flatten)]
    pub data: Candle,
}

/// Represents a Ticker update received from the Websocket API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TickerUpdate {
    /// Ticker update type.
    pub r#type: String,
    /// Product ID (Pair, ex 'BTC-USD')
    pub product_id: String,
    /// Current price for the product.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price: f64,
    /// 24hr Volume for the product.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub volume_24_h: f64,
    /// 24hr Lowest price.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub low_24_h: f64,
    /// 24hr Highest price.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub high_24_h: f64,
    /// 52w (52 weeks) Lowest price.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub low_52_w: f64,
    /// 52w (52 weeks) Highest price.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub high_52_w: f64,
    /// 24hr Price percentage change.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price_percent_chg_24_h: f64,
}

/// Order updates for a user from a websocket.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdate {
    /// Type of the update.
    pub r#type: String,
    /// Client Order ID (Normally a UUID)
    pub client_order_id: String,
    #[serde(deserialize_with = "deserialize_numeric")]
    pub cumulative_quantity: f64,
    #[serde(deserialize_with = "deserialize_numeric")]
    pub leaves_quantity: f64,
    /// Average price for the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub avg_price: f64,
    /// Total fees for the order.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub total_fees: f64,
    /// Status of the order.
    pub status: String,
    /// Product ID.
    pub product_id: String,
    /// Date-time when the order was created.
    pub creation_time: String,
    /// BUY or SELL.
    pub order_side: OrderSide,
    /// Type of the order.
    pub order_type: String,
}
