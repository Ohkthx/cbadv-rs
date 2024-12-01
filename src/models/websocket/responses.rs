use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, DisplayFromStr};

use crate::order::{OrderSide, OrderStatus, OrderType, TimeInForce, TriggerStatus};
use crate::product::{Candle, ProductType};

use super::Level2Side;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Level2Update {
    pub side: Level2Side,
    pub event_time: String,
    #[serde_as(as = "DisplayFromStr")]
    pub price_level: f64,
    #[serde_as(as = "DisplayFromStr")]
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
#[serde_as]
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
    #[serde_as(as = "DisplayFromStr")]
    pub base_increment: f64,
    /// Minimum amount quote value can be increased or decreased at once.
    #[serde_as(as = "DisplayFromStr")]
    pub quote_increment: f64,
    /// Name of the product.
    pub display_name: String,
    /// Status of the product.
    pub status: String,
    /// Additional status message.
    pub status_message: String,
    /// Minimum amount of funds.
    #[serde_as(as = "DisplayFromStr")]
    pub min_market_funds: f64,
}

/// Represents a Market Trade received from the Websocket API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketTradesUpdate {
    /// Trade identity.
    pub trade_id: String,
    /// ID of the product.
    pub product_id: String,
    /// Price of the product.
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    /// Size for the trade.
    #[serde_as(as = "DisplayFromStr")]
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
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TickerUpdate {
    /// Ticker update type.
    pub r#type: String,
    /// Product ID (Pair, ex 'BTC-USD')
    pub product_id: String,
    /// Current price for the product.
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    /// 24hr Volume for the product.
    #[serde_as(as = "DisplayFromStr")]
    pub volume_24_h: f64,
    /// 24hr Lowest price.
    #[serde_as(as = "DisplayFromStr")]
    pub low_24_h: f64,
    /// 24hr Highest price.
    #[serde_as(as = "DisplayFromStr")]
    pub high_24_h: f64,
    /// 52w (52 weeks) Lowest price.
    #[serde_as(as = "DisplayFromStr")]
    pub low_52_w: f64,
    /// 52w (52 weeks) Highest price.
    #[serde_as(as = "DisplayFromStr")]
    pub high_52_w: f64,
    /// 24hr Price percentage change.
    #[serde_as(as = "DisplayFromStr")]
    pub price_percent_chg_24_h: f64,
}

/// Order updates for a user from a websocket.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderUpdate {
    #[serde_as(as = "DisplayFromStr")]
    pub avg_price: f64,
    pub cancel_reason: String,
    pub client_order_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub completion_percentage: f64,
    pub contract_expiry_type: String,
    #[serde_as(as = "DisplayFromStr")]
    pub cumulative_quantity: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub filled_value: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub leaves_quantity: f64,
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub limit_price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub number_of_fills: u32,
    pub order_id: String,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    #[serde_as(as = "DisplayFromStr")]
    pub outstanding_hold_amount: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub post_only: bool,
    pub product_id: String,
    pub product_type: ProductType,
    pub reject_reason: Option<String>,
    pub retail_portfolio_id: String,
    pub risk_managed_by: String,
    pub status: OrderStatus,
    #[serde_as(as = "DefaultOnError<Option<DisplayFromStr>>")]
    #[serde(default)]
    pub stop_price: Option<f64>,
    pub time_in_force: TimeInForce,
    #[serde_as(as = "DisplayFromStr")]
    pub total_fees: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub total_value_after_fees: f64,
    pub trigger_status: TriggerStatus,
    pub creation_time: String,
    pub end_time: String,
    pub start_time: String,
}
