//! # Coinbase Advanced Product API
//!
//! `product` gives access to the Product API and the various endpoints associated with it.
//! This allows you to obtain product information such as: Ticker (Market Trades), Product and
//! Currency information, Product Book, and Best Bids and Asks for multiple products.

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::traits::Query;
use crate::utils::{deserialize_numeric, QueryBuilder};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    /// Unknown product type.
    #[serde(rename = "UNKOWNN_PRODUCT_TYPE")]
    Unknown,
    /// Spot product type.
    Spot,
    /// Future product type.
    Future,
}

impl fmt::Display for ProductType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for ProductType {
    fn as_ref(&self) -> &str {
        match self {
            ProductType::Unknown => "UNKNOWN_PRODUCT_TYPE",
            ProductType::Spot => "SPOT",
            ProductType::Future => "FUTURE",
        }
    }
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
    pub side: String,
    /// Time for the market trade.
    pub time: String,
}

/// Session details for the product.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionDetails {
    /// Whether or not the session is currently open.
    pub is_session_open: bool,
    /// Time the session opened.
    pub open_time: String,
    /// Time the session closed.
    pub close_time: String,
}

/// Perpetual details for the product.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerpetualDetails {
    pub open_interest: String,
    pub funding_rate: String,
    pub funding_time: String,
}

/// Future details for the product.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FutureDetails {
    pub venue: String,
    pub contract_code: String,
    pub contract_expiry: String,
    pub contract_size: String,
    pub contract_root_unit: String,
    /// Descriptive name for the product series, eg "Nano Bitcoin Futures".
    pub group_description: String,
    pub contract_expiry_timezone: String,
    /// Short version of the group_description, eg "Nano BTC".
    pub group_short_description: String,
    /// Possible values: [UNKNOWN_RISK_MANAGEMENT_TYPE, MANAGED_BY_FCM, MANAGED_BY_VENUE]
    pub risk_managed_by: String,
    /// Possible values: [UNKNOWN_CONTRACT_EXPIRY_TYPE, EXPIRING]
    pub contract_expiry_type: String,
    pub perpetual_details: Option<PerpetualDetails>,
    /// Name of the contract.
    pub contract_display_name: String,
}

/// Represents a Product received from the REST API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    /// The trading pair.
    pub product_id: String,
    /// The current price for the product, in quote currency.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price: f64,
    /// The amount the price of the product has changed, in percent, in the last 24 hours.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price_percentage_change_24h: f64,
    /// The trading volume for the product in the last 24 hours.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub volume_24h: f64,
    /// The percentage amount the volume of the product has changed in the last 24 hours.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub volume_percentage_change_24h: f64,
    /// Minimum amount base value can be increased or decreased at once.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub base_increment: f64,
    /// Minimum amount quote value can be increased or decreased at once.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub quote_increment: f64,
    /// Minimum size that can be represented of quote currency.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub quote_min_size: f64,
    /// Maximum size that can be represented of quote currency.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub quote_max_size: f64,
    /// Minimum size that can be represented of base currency.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub base_min_size: f64,
    /// Maximum size that can be represented of base currency.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub base_max_size: f64,
    /// Name of the base currency.
    pub base_name: String,
    /// Name of the quote currency.
    pub quote_name: String,
    /// Whether or not the product is on the user's watchlist.
    pub watched: bool,
    /// Whether or not the product is disabled for trading.
    pub is_disabled: bool,
    /// Whether or not the product is 'new'.
    pub new: bool,
    /// Status of the product.
    pub status: String,
    /// Whether or not orders of the product can only be cancelled, not placed or edited.
    pub cancel_only: bool,
    /// Whether or not orders of the product can only be limit orders, not market orders.
    pub limit_only: bool,
    /// Whether or not orders of the product can only be posted, not cancelled.
    pub post_only: bool,
    /// Whether or not the product is disabled for trading for all market participants.
    pub trading_disabled: bool,
    /// Whether or not the product is in auction mode.
    pub auction_mode: bool,
    /// Possible values: [SPOT, FUTURE]
    pub product_type: ProductType,
    /// Symbol of the quote currency.
    pub quote_currency_id: String,
    /// Symbol of the base currency.
    pub base_currency_id: String,
    /// Session details.
    pub fcm_trading_session_details: Option<SessionDetails>,
    /// The current midpoint of the bid-ask spread, in quote currency.
    pub mid_market_price: String,
    /// Product id for the corresponding unified book.
    pub alias: String,
    /// Product ids that this product serves as an alias for.
    pub alias_to: Vec<String>,
    /// Symbol of the base display currency.
    pub base_display_symbol: String,
    /// Symbol of the quote display currency.
    pub quote_display_symbol: String,
    /// Whether or not the product is in view only mode.
    pub view_only: bool,
    /// Minimum amount price can be increased or decreased at once.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price_increment: f64,
    /// Future product details.
    pub future_product_details: Option<FutureDetails>,
}

/// Represents a Bid or an Ask entry for a product.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAsk {
    /// Current bid or ask price.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price: f64,
    /// Current bid or ask size.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub size: f64,
}

/// Represents a product book for a product.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductBook {
    /// The trading pair.
    pub product_id: String,
    /// Time of the product book.
    pub time: String,
    /// Array of current bids.
    pub bids: Vec<BidAsk>,
    /// Array of current asks.
    pub asks: Vec<BidAsk>,
    #[serde(default)]
    pub last: String,
    #[serde(default)]
    pub mid_market: String,
    #[serde(default)]
    pub spread_bps: String,
    #[serde(default)]
    pub spread_absolute: String,
}

/// Represents a candle for a product.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Candle {
    /// Timestamp for bucket start time, in UNIX time.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub start: u64,
    /// Lowest price during the bucket interval.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub low: f64,
    /// Highest price during the bucket interval.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub high: f64,
    /// Opening price (first trade) in the bucket interval.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub open: f64,
    /// Closing price (last trade) in the bucket interval.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub close: f64,
    /// Volume of trading activity during the bucket interval.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub volume: f64,
}

/// Represents a trade for a product.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    /// The ID of the trade that was placed.
    pub trade_id: String,
    /// The trading pair.
    pub product_id: String,
    /// The price of the trade, in quote currency.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub price: f64,
    /// The size of the trade, in base currency.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub size: f64,
    /// The time of the trade.
    pub time: String,
    /// Possible values: [UNKNOWN_ORDER_SIDE, BUY, SELL]
    pub side: String,
    /// The best bid for the `product_id`, in quote currency.
    /// NOTE: (20230705) API gives an empty string not a number.
    pub bid: String,
    /// The best ask for the `product_id`, in quote currency.
    /// NOTE: (20230705) API gives an empty string not a number.
    pub ask: String,
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

/// Represents a ticker for a product.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticker {
    /// List of trades for the product.
    pub trades: Vec<Trade>,
    /// The best bid for the `product_id`, in quote currency.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub best_bid: f64,
    /// The best ask for the `product_id`, in quote currency.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub best_ask: f64,
}

/// Represents a list of Products received from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct ListProductsResponse {
    /// Array of objects, each representing one product.
    pub(crate) products: Vec<Product>,
    // Number of products that were returned.
    // NOTE: Disabled because `.len()` exists on the vector.
    // num_products: i32,
}

/// Represents a candle response from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct CandleResponse {
    /// Array of candles for the product.
    pub(crate) candles: Vec<Candle>,
}

/// Represents a best bid and ask response from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct BidAskResponse {
    /// Array of product books.
    pub(crate) pricebooks: Vec<ProductBook>,
}

/// Represents a product book response from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct ProductBookResponse {
    /// Price book for the product.
    pub(crate) pricebook: ProductBook,
}

/// Represents parameters that are optional for List Products API request.
#[derive(Serialize, Default, Debug)]
pub struct ListProductsQuery {
    /// A limit describing how many products to return.
    pub limit: Option<u32>,
    /// Number of products to offset before returning.
    pub offset: Option<u32>,
    /// Type of products to return. Valid options: SPOT or FUTURE
    pub product_type: Option<ProductType>,
    /// List of product IDs to return.
    pub product_ids: Option<Vec<String>>,
    /// If true, return all products of all product types (including expired futures contracts).
    pub get_all_products: Option<bool>,
}

impl Query for ListProductsQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_u32_optional("limit", self.limit)
            .push_u32_optional("offset", self.offset)
            .push_optional("product_type", &self.product_type)
            .with_optional_vec("product_ids", &self.product_ids)
            .push_bool_optional("get_all_products", self.get_all_products)
            .build()
    }
}

/// Represents parameters for Ticker Product API request.
#[derive(Serialize, Debug)]
pub struct TickerQuery {
    /// Number of trades to return.
    pub limit: u32,
}

impl Query for TickerQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new().push("limit", self.limit).build()
    }
}

/// Represents parameters for Ticker Product API request.
#[derive(Serialize, Debug)]
pub struct BidAskQuery {
    pub product_ids: Vec<String>,
}

impl Query for BidAskQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .with_optional_vec("product_ids", &Some(self.product_ids.clone()))
            .build()
    }
}

/// Represents parameters for Ticker Product API request.
#[derive(Serialize, Debug)]
pub struct ProductBookQuery {
    pub product_id: String,
    /// Number of products to return.
    pub limit: Option<u32>,
}

impl Query for ProductBookQuery {
    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push("product_id", &self.product_id)
            .push_u32_optional("limit", self.limit)
            .build()
    }
}
