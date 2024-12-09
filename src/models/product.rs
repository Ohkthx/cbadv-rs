//! # Coinbase Advanced Product API
//!
//! `product` gives access to the Product API and the various endpoints associated with it.
//! This allows you to obtain product information such as: Ticker (Market Trades), Product and
//! Currency information, Product Book, and Best Bids and Asks for multiple products.

use core::fmt;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, DisplayFromStr};

use crate::constants::products::CANDLE_MAXIMUM;
use crate::errors::CbError;
use crate::models::websocket::CandleUpdate;
use crate::time::{self, Granularity};
use crate::traits::Query;
use crate::types::CbResult;
use crate::utils::QueryBuilder;

use super::order::OrderSide;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    /// Unknown product type.
    #[serde(rename = "UNKNOWN_PRODUCT_TYPE")]
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

/// Represents the trading session state.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SessionState {
    #[serde(rename = "FCM_TRADING_SESSION_STATE_UNDEFINED")]
    Undefined,
    #[serde(rename = "FCM_TRADING_SESSION_STATE_PRE_OPEN")]
    PreOpen,
    #[serde(rename = "FCM_TRADING_SESSION_STATE_PRE_OPEN_NO_CANCEL")]
    PreOpenNoCancel,
    #[serde(rename = "FCM_TRADING_SESSION_STATE_OPEN")]
    Open,
    #[serde(rename = "FCM_TRADING_SESSION_STATE_CLOSE")]
    Close,
}

/// Reasons for a trading session to close.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CloseReason {
    #[serde(rename = "FCM_TRADING_SESSION_CLOSED_REASON_UNDEFINED")]
    Undefined,
    #[serde(rename = "FCM_TRADING_SESSION_CLOSED_REASON_REGULAR_MARKET_CLOSE")]
    RegularMarketClose,
    #[serde(rename = "FCM_TRADING_SESSION_CLOSED_REASON_EXCHANGE_MAINTENANCE")]
    ExchangeMaintenance,
    #[serde(rename = "FCM_TRADING_SESSION_CLOSED_REASON_VENDOR_MAINTENANCE")]
    VendorMaintenance,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductVenue {
    #[serde(rename = "UNKNOWN_VENUE_TYPE")]
    Unknown,
    Cbe,
    Fcm,
    Intx,
}

/// Fcm specific scheduled maintenance details.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Maintenance {
    /// Start time of the maintenance.
    pub start: String,
    /// End time of the maintenance.
    pub end: String,
}

/// Session details for the product.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionDetails {
    /// Whether or not the session is currently open.
    pub is_session_open: bool,
    /// Time the session opened.
    pub open_time: String,
    /// Time the session closed.
    pub close_time: String,
    /// The current state of the session.
    pub session_state: SessionState,
    /// Whether or not after-hours order entry
    pub after_hours_order_entry_disabled: bool,
    /// Reason the session closed.
    pub closed_reason: CloseReason,
    /// Whether or not the session is in maintenance.
    #[serde_as(as = "DefaultOnError")]
    #[serde(default)]
    pub maintenance: Option<Maintenance>,
}

/// Perpetual details for the product.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerpetualDetails {
    pub open_interest: String,
    pub funding_rate: String,
    pub funding_time: String,
    pub max_leverage: String,
    pub base_asset_uuid: String,
    pub underlying_type: String,
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
    /// Short version of the `group_description`, eg "Nano BTC".
    pub group_short_description: String,
    /// Possible values: [`UNKNOWN_RISK_MANAGEMENT_TYPE`, `MANAGED_BY_FCM`, `MANAGED_BY_VENUE`]
    pub risk_managed_by: String,
    /// Possible values: [`UNKNOWN_CONTRACT_EXPIRY_TYPE`, EXPIRING]
    pub contract_expiry_type: String,
    pub perpetual_details: Option<PerpetualDetails>,
    /// Name of the contract.
    pub contract_display_name: String,
    pub time_to_expiry_ms: String,
    pub non_crypto: bool,
    pub contract_expiry_name: String,
    pub twenty_four_by_seven: bool,
}

/// Represents a Product received from the REST API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    /// The trading pair.
    pub product_id: String,
    /// The current price for the product, in quote currency.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub price: f64,
    /// The amount the price of the product has changed, in percent, in the last 24 hours.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub price_percentage_change_24h: f64,
    /// The trading volume for the product in the last 24 hours.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub volume_24h: f64,
    /// The percentage amount the volume of the product has changed in the last 24 hours.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub volume_percentage_change_24h: f64,
    /// Minimum amount base value can be increased or decreased at once.
    #[serde_as(as = "DisplayFromStr")]
    pub base_increment: f64,
    /// Minimum amount quote value can be increased or decreased at once.
    #[serde_as(as = "DisplayFromStr")]
    pub quote_increment: f64,
    /// Minimum size that can be represented of quote currency.
    #[serde_as(as = "DisplayFromStr")]
    pub quote_min_size: f64,
    /// Maximum size that can be represented of quote currency.
    #[serde_as(as = "DisplayFromStr")]
    pub quote_max_size: f64,
    /// Minimum size that can be represented of base currency.
    #[serde_as(as = "DisplayFromStr")]
    pub base_min_size: f64,
    /// Maximum size that can be represented of base currency.
    #[serde_as(as = "DisplayFromStr")]
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
    #[serde_as(as = "DisplayFromStr")]
    pub price_increment: f64,
    /// Display name of the product.
    pub display_name: String,
    /// The sole venue id for the product. Defaults to CBE if the product is not specific to a single venue
    pub product_venue: ProductVenue,
    /// Approximate 24-hour trading volume in quote currency.
    #[serde_as(as = "DefaultOnError<DisplayFromStr>")]
    #[serde(default)]
    pub approximate_quote_24h_volume: f64,
    /// Future product details.
    pub future_product_details: Option<FutureDetails>,
}

/// Represents a Bid or an Ask entry for a product.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAsk {
    /// Current bid or ask price.
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    /// Current bid or ask size.
    #[serde_as(as = "DisplayFromStr")]
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
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Candle {
    /// Timestamp for bucket start time, in UNIX time.
    #[serde_as(as = "DisplayFromStr")]
    pub start: u64,
    /// Lowest price during the bucket interval.
    #[serde_as(as = "DisplayFromStr")]
    pub low: f64,
    /// Highest price during the bucket interval.
    #[serde_as(as = "DisplayFromStr")]
    pub high: f64,
    /// Opening price (first trade) in the bucket interval.
    #[serde_as(as = "DisplayFromStr")]
    pub open: f64,
    /// Closing price (last trade) in the bucket interval.
    #[serde_as(as = "DisplayFromStr")]
    pub close: f64,
    /// Volume of trading activity during the bucket interval.
    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,
}

impl From<CandleUpdate> for Candle {
    fn from(candle_update: CandleUpdate) -> Self {
        candle_update.data
    }
}

/// Represents a trade for a product.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Trade {
    /// The ID of the trade that was placed.
    pub trade_id: String,
    /// The trading pair.
    pub product_id: String,
    /// The price of the trade, in quote currency.
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    /// The size of the trade, in base currency.
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    /// The time of the trade.
    pub time: String,
    /// Possible values: [UNKNOWN_ORDER_SIDE, BUY, SELL]
    pub side: OrderSide,
    /// The exchange where the trade was placed.
    pub exchange: String,
}

/// Represents a ticker for a product.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticker {
    /// List of trades for the product.
    pub trades: Vec<Trade>,
    /// The best bid for the `product_id`, in quote currency.
    #[serde_as(as = "DisplayFromStr")]
    pub best_bid: f64,
    /// The best ask for the `product_id`, in quote currency.
    #[serde_as(as = "DisplayFromStr")]
    pub best_ask: f64,
}

/// Represents parameters that are optional for List Products API request.
#[derive(Serialize, Default, Debug)]
pub struct ProductListQuery {
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
    /// Whether or not to populate `view_only` with the tradability status of the product. This is only enabled for SPOT products.
    pub get_tradability_status: Option<bool>,
}

impl Query for ProductListQuery {
    fn check(&self) -> CbResult<()> {
        if let Some(limit) = self.limit {
            if limit == 0 {
                return Err(CbError::BadQuery(
                    "limit must be greater than 0".to_string(),
                ));
            }
        } else if let Some(offset) = self.offset {
            if offset == 0 {
                return Err(CbError::BadQuery(
                    "offset must be greater than 0".to_string(),
                ));
            }
        } else if let Some(product_type) = &self.product_type {
            if *product_type == ProductType::Unknown {
                return Err(CbError::BadQuery(
                    "product_type cannot be unknown".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional("limit", self.limit.as_ref())
            .push_optional("offset", self.offset.as_ref())
            .push_optional("product_type", self.product_type.as_ref())
            .push_optional_vec("product_ids", self.product_ids.as_ref())
            .push_optional("get_all_products", self.get_all_products.as_ref())
            .push_optional(
                "get_tradability_status",
                self.get_tradability_status.as_ref(),
            )
            .build()
    }
}

impl ProductListQuery {
    /// Creates a new `ProductListQuery` object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of products to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Number of products to offset before returning.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Type of products to return. Valid options: SPOT or FUTURE.
    pub fn product_type(mut self, product_type: ProductType) -> Self {
        self.product_type = Some(product_type);
        self
    }

    /// List of product IDs to return.
    pub fn product_ids(mut self, product_ids: &[String]) -> Self {
        self.product_ids = Some(product_ids.to_vec());
        self
    }

    /// If true, return all products of all product types (including expired futures contracts.
    pub fn get_all_products(mut self, get_all_products: bool) -> Self {
        self.get_all_products = Some(get_all_products);
        self
    }

    /// Whether or not to populate `view_only` with the tradability status of the product. This is only enabled for SPOT products.
    pub fn get_tradability_status(mut self, get_tradability_status: bool) -> Self {
        self.get_tradability_status = Some(get_tradability_status);
        self
    }
}

/// Represents parameters for Ticker Product API request.
#[derive(Serialize, Debug)]
pub struct ProductTickerQuery {
    /// Number of trades to return.
    pub limit: u32,
    /// The UNIX timestamp indicating the start of the time interval.
    pub start: Option<String>,
    /// The UNIX timestamp indicating the end of the time interval.
    pub end: Option<String>,
}

impl Query for ProductTickerQuery {
    fn check(&self) -> CbResult<()> {
        if self.limit == 0 {
            return Err(CbError::BadQuery(
                "limit must be greater than 0".to_string(),
            ));
        } else if let (Some(start), Some(end)) = (&self.start, &self.end) {
            if start >= end {
                return Err(CbError::BadQuery("start must be less than end".to_string()));
            }
        }
        Ok(())
    }

    /// Converts the object into HTTP request parameters.
    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push("limit", self.limit)
            .push_optional("start", self.start.as_ref())
            .push_optional("end", self.end.as_ref())
            .build()
    }
}

impl Default for ProductTickerQuery {
    fn default() -> Self {
        Self {
            limit: 100,
            start: None,
            end: None,
        }
    }
}

impl ProductTickerQuery {
    /// Creates a new `ProductTickerQuery` object with default values.
    ///
    /// # Arguments
    ///
    /// * `limit` - Number of trades to return.
    pub fn new(limit: u32) -> Self {
        Self {
            limit,
            ..Default::default()
        }
    }

    /// Number of trades to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    /// The UNIX timestamp indicating the start of the time interval.
    pub fn start(mut self, start: &str) -> Self {
        self.start = Some(start.to_string());
        self
    }

    /// The UNIX timestamp indicating the end of the time interval.
    pub fn end(mut self, end: &str) -> Self {
        self.end = Some(end.to_string());
        self
    }
}

/// Represents parameters for Ticker Product API request.
#[derive(Serialize, Debug, Default)]
pub struct ProductBidAskQuery {
    /// The list of trading pairs (e.g. 'BTC-USD').
    pub product_ids: Vec<String>,
}

impl Query for ProductBidAskQuery {
    fn check(&self) -> CbResult<()> {
        Ok(())
    }

    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push_optional_vec("product_ids", Some(self.product_ids.as_ref()))
            .build()
    }
}

impl ProductBidAskQuery {
    /// Creates a new `ProductBidAskQuery` object with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// The list of trading pairs (e.g. 'BTC-USD').
    pub fn product_ids(mut self, product_ids: &[String]) -> Self {
        self.product_ids = product_ids.to_vec();
        self
    }
}

/// Represents parameters for Ticker Product API request.
#[derive(Serialize, Debug, Default)]
pub struct ProductBookQuery {
    /// The trading pair (e.g. 'BTC-USD').
    pub product_id: String,
    /// The number of bid/asks to be returned.
    pub limit: Option<u32>,
    /// The minimum price intervals at which buy and sell orders are grouped or combined in the order book.
    pub aggregation_price_increment: Option<f64>,
}

impl Query for ProductBookQuery {
    fn check(&self) -> CbResult<()> {
        if self.product_id.is_empty() {
            return Err(CbError::BadQuery("product_id is required".to_string()));
        } else if let Some(limit) = self.limit {
            if limit == 0 {
                return Err(CbError::BadQuery(
                    "limit must be greater than 0".to_string(),
                ));
            }
        } else if let Some(aggregation_price_increment) = self.aggregation_price_increment {
            if aggregation_price_increment <= 0.0 {
                return Err(CbError::BadQuery(
                    "aggregation_price_increment must be greater than 0".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push("product_id", &self.product_id)
            .push_optional("limit", self.limit.as_ref())
            .push_optional(
                "aggregation_price_increment",
                self.aggregation_price_increment.as_ref(),
            )
            .build()
    }
}

impl ProductBookQuery {
    /// Creates a new `ProductBookQuery` object with default values.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The trading pair (e.g. 'BTC-USD').
    pub fn new(product_id: &str) -> Self {
        Self {
            product_id: product_id.to_string(),
            ..Default::default()
        }
    }

    /// The trading pair (e.g. 'BTC-USD').
    pub fn product_id(mut self, product_id: &str) -> Self {
        self.product_id = product_id.to_string();
        self
    }

    /// The number of bid/asks to be returned.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// The minimum price intervals at which buy and sell orders are grouped or combined in the order book.
    pub fn aggregation_price_increment(mut self, aggregation_price_increment: f64) -> Self {
        self.aggregation_price_increment = Some(aggregation_price_increment);
        self
    }
}

/// Represents parameters for Candles Product API request.
///
/// # Required Parameters
///
/// * `start` - The start time of the time range.
/// * `end` - The end time of the time range.
/// * `granularity` - The granularity of the candles.
#[derive(Serialize, Debug, Clone)]
pub struct ProductCandleQuery {
    /// The start time of the time range.
    pub start: u64,
    /// The end time of the time range.
    pub end: u64,
    /// The granularity of the candles.
    pub granularity: Granularity,
    /// The number of candles to return. Maximum is 350.
    pub limit: u32,
}

impl Query for ProductCandleQuery {
    fn check(&self) -> CbResult<()> {
        if self.limit == 0 {
            return Err(CbError::BadQuery(
                "limit must be greater than 0".to_string(),
            ));
        } else if self.start >= self.end {
            return Err(CbError::BadQuery("start must be less than end".to_string()));
        } else if self.granularity == Granularity::Unknown {
            return Err(CbError::BadQuery(
                "granularity cannot be unknown or unset".to_string(),
            ));
        }
        Ok(())
    }

    fn to_query(&self) -> String {
        QueryBuilder::new()
            .push("start", self.start)
            .push("end", self.end)
            .push("granularity", &self.granularity)
            .push("limit", self.limit)
            .build()
    }
}

impl Default for ProductCandleQuery {
    fn default() -> Self {
        Self {
            start: time::now() - u64::from(Granularity::to_secs(&Granularity::OneDay)),
            end: time::now(),
            granularity: Granularity::FiveMinute,
            limit: CANDLE_MAXIMUM,
        }
    }
}

impl ProductCandleQuery {
    /// Creates a new `ProductCandleQuery` object with default values.
    ///
    /// # Arguments
    ///
    /// * `start` - The start time of the time range.
    /// * `end` - The end time of the time range.
    /// * `granularity` - The granularity of the candles.
    pub fn new(start: u64, end: u64, granularity: Granularity) -> Self {
        Self {
            start,
            end,
            granularity,
            limit: CANDLE_MAXIMUM,
        }
    }

    /// The start time of the time range.
    /// Note: This is a required field.
    pub fn start(mut self, start: u64) -> Self {
        self.start = start;
        self
    }

    /// The end time of the time range.
    /// Note: This is a required field.
    pub fn end(mut self, end: u64) -> Self {
        self.end = end;
        self
    }

    /// The granularity of the candles.
    /// Note: This is a required field.
    pub fn granularity(mut self, granularity: Granularity) -> Self {
        self.granularity = granularity;
        self
    }

    /// The number of candles to return. Maximum is 350.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }
}

/// Represents a list of Products received from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct ProductsWrapper {
    /// Array of objects, each representing one product.
    pub(crate) products: Vec<Product>,
    // Number of products that were returned.
    // NOTE: Disabled because `.len()` exists on the vector.
    // num_products: i32,
}

impl From<ProductsWrapper> for Vec<Product> {
    fn from(wrapper: ProductsWrapper) -> Self {
        wrapper.products
    }
}

/// Represents a candle response from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct CandlesWrapper {
    /// Array of candles for the product.
    pub(crate) candles: Vec<Candle>,
}

impl From<CandlesWrapper> for Vec<Candle> {
    fn from(wrapper: CandlesWrapper) -> Self {
        wrapper.candles
    }
}

/// Represents a best bid and ask response from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct ProductBooksWrapper {
    /// Array of product books.
    pub(crate) pricebooks: Vec<ProductBook>,
}

impl From<ProductBooksWrapper> for Vec<ProductBook> {
    fn from(wrapper: ProductBooksWrapper) -> Self {
        wrapper.pricebooks
    }
}

/// Represents a product book response from the API.
#[derive(Deserialize, Debug)]
pub(crate) struct ProductBookWrapper {
    /// Price book for the product.
    pub(crate) pricebook: ProductBook,
}

impl From<ProductBookWrapper> for ProductBook {
    fn from(wrapper: ProductBookWrapper) -> Self {
        wrapper.pricebook
    }
}
