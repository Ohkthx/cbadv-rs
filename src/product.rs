//! # Coinbase Advanced Product API
//!
//! `product` gives access to the Product API and the various endpoints associated with it.
//! This allows you to obtain product information such as: Ticker (Market Trades), Product and
//! Currency information, Product Book, and Best Bids and Asks for multiple products.

use crate::signer::Signer;
use crate::time;
use crate::utils::{CBAdvError, Result};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, DisplayFromStr};
use std::fmt;

/// Maximum amount returned.
const CANDLE_MAXIMUM: usize = 300;

/// Represents a Product received from the Websocket API.
#[serde_as]
#[derive(Deserialize, Debug)]
pub struct ProductUpdate {
    /// Type of the product.
    pub product_type: String,
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
#[derive(Deserialize, Debug)]
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
    pub side: String,
    /// Time for the market trade.
    pub time: String,
}

/// Session details for the product.
#[derive(Deserialize, Debug)]
pub struct SessionDetails {
    /// Whether or not the session is currently open.
    pub is_session_open: bool,
    /// Time the session opened.
    pub open_time: String,
    /// Time the session closed.
    pub close_time: String,
}

/// Perpetual details for the product.
#[derive(Deserialize, Debug)]
pub struct PerpetualDetails {
    pub open_interest: String,
    pub funding_rate: String,
    pub funding_time: String,
}

/// Future details for the product.
#[serde_as]
#[derive(Deserialize, Debug)]
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
#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Product {
    /// The trading pair.
    pub product_id: String,
    /// The current price for the product, in quote currency.
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    /// The amount the price of the product has changed, in percent, in the last 24 hours.
    #[serde_as(as = "DefaultOnError")]
    pub price_percentage_change_24h: f64,
    /// The trading volume for the product in the last 24 hours.
    #[serde_as(as = "DefaultOnError")]
    pub volume_24h: f64,
    /// The percentage amount the volume of the product has changed in the last 24 hours.
    #[serde_as(as = "DefaultOnError")]
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
    pub product_type: String,
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
    /// Future product details.
    pub future_product_details: Option<FutureDetails>,
}

/// Represents a Bid or an Ask entry for a product.
#[serde_as]
#[derive(Deserialize, Debug)]
pub struct BidAsk {
    /// Current bid or ask price.
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    /// Current bid or ask size.
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
}

/// Represents a product book for a product.
#[derive(Deserialize, Debug)]
pub struct ProductBook {
    /// The trading pair.
    pub product_id: String,
    /// Time of the product book.
    pub time: String,
    /// Array of current bids.
    pub bids: Vec<BidAsk>,
    /// Array of current asks.
    pub asks: Vec<BidAsk>,
}

/// Represents a candle for a product.
#[serde_as]
#[derive(Deserialize, Debug)]
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

/// Represents a trade for a product.
#[serde_as]
#[derive(Deserialize, Debug)]
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
    pub side: String,
    /// The best bid for the `product_id`, in quote currency.
    /// NOTE: (20230705) API gives an empty string not a number.
    pub bid: String,
    /// The best ask for the `product_id`, in quote currency.
    /// NOTE: (20230705) API gives an empty string not a number.
    pub ask: String,
}

/// Represents a Ticker update received from the Websocket API.
#[serde_as]
#[derive(Deserialize, Debug)]
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

/// Represents a ticker for a product.
#[serde_as]
#[derive(Deserialize, Debug)]
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

/// Represents a list of Products received from the API.
#[derive(Deserialize, Debug)]
struct ListProductsResponse {
    /// Array of objects, each representing one product.
    pub products: Vec<Product>,
    // Number of products that were returned.
    // NOTE: Disabled because `.len()` exists on the vector.
    // pub num_products: i32,
}

/// Represents a candle response from the API.
#[derive(Deserialize, Debug)]
struct CandleResponse {
    /// Array of candles for the product.
    pub candles: Vec<Candle>,
}

/// Represents a best bid and ask response from the API.
#[derive(Deserialize, Debug)]
struct BidAskResponse {
    /// Array of product books.
    pub pricebooks: Vec<ProductBook>,
}

/// Represents a product book response from the API.
#[derive(Deserialize, Debug)]
struct ProductBookResponse {
    /// Price book for the product.
    pub pricebook: ProductBook,
}

/// Represents parameters that are optional for List Products API request.
#[derive(Serialize, Default, Debug)]
pub struct ListProductsQuery {
    /// A limit describing how many products to return.
    pub limit: Option<u32>,
    /// Number of products to offset before returning.
    pub offset: Option<u32>,
    /// Type of products to return. Valid options: SPOT or FUTURE
    pub product_type: Option<String>,
    /// List of product IDs to return.
    pub product_ids: Option<Vec<String>>,
}

impl fmt::Display for ListProductsQuery {
    /// Converts the object into HTTP request parameters.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut query: String = "".to_string();

        query = match &self.limit {
            Some(v) => format!("{}&limit={}", query, v),
            _ => query,
        };

        query = match &self.offset {
            Some(v) => format!("{}&offset={}", query, v),
            _ => query,
        };

        query = match &self.product_type {
            Some(v) => format!("{}&product_type={}", query, v),
            _ => query,
        };

        query = match &self.product_ids {
            Some(v) => {
                let ids: String = v.iter().map(|p| format!("&product_ids={p}")).collect();
                format!("{}{}", query, ids)
            }
            _ => query,
        };

        match query.is_empty() {
            true => write!(f, ""),
            false => write!(f, "{}", query[1..].to_string()),
        }
    }
}

/// Represents parameters for Ticker Product API request.
#[derive(Serialize, Debug)]
pub struct TickerQuery {
    /// Number of trades to return.
    pub limit: u32,
}

impl fmt::Display for TickerQuery {
    /// Converts the object into HTTP request parameters.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "limit={}", self.limit)
    }
}

/// Provides access to the Product API for the service.
pub struct ProductAPI {
    /// Object used to sign requests made to the API.
    signer: Signer,
}

impl ProductAPI {
    /// Resource for the API.
    const RESOURCE: &str = "/api/v3/brokerage/products";

    /// Creates a new instance of the Product API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `signer` - A Signer that include the API Key & Secret along with a client to make
    /// requests.
    pub(crate) fn new(signer: Signer) -> Self {
        Self { signer }
    }

    /// Obtains best bids and asks for a vector of product IDs..
    ///
    /// # Arguments
    ///
    /// * `product_ids` - A vector of strings the represents the product IDs of product books to
    /// obtain.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/best_bid_ask
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getbestbidask>
    pub async fn best_bid_ask(&self, product_ids: Vec<String>) -> Result<Vec<ProductBook>> {
        let resource = "/api/v3/brokerage/best_bid_ask";
        let query = format!("product_ids={}", product_ids.join("&product_ids="));

        match self.signer.get(resource, &query).await {
            Ok(value) => match value.json::<BidAskResponse>().await {
                Ok(bidasks) => Ok(bidasks.pricebooks),
                Err(_) => Err(CBAdvError::BadParse("bid asks object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains the product book (bids and asks) for the product ID provided.
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string the represents the product's ID.
    /// * `limit` - An integer the represents the amount to obtain, defaults to 250.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/product_book
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getproductbook>
    pub async fn product_book(&self, product_id: &str, limit: Option<u16>) -> Result<ProductBook> {
        let resource = "/api/v3/brokerage/product_book";
        let query = format!("product_id={}&limit={}", product_id, limit.unwrap_or(250));

        match self.signer.get(resource, &query).await {
            Ok(value) => match value.json::<ProductBookResponse>().await {
                Ok(book) => Ok(book.pricebook),
                Err(_) => Err(CBAdvError::BadParse("product book object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains a single product based on the Product ID (ex. "BTC-USD").
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string the represents the product's ID.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/products/{product_id}
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getproduct>
    pub async fn get(&self, product_id: &str) -> Result<Product> {
        let resource = format!("{}/{}", Self::RESOURCE, product_id);
        match self.signer.get(&resource, "").await {
            Ok(value) => match value.json::<Product>().await {
                Ok(product) => Ok(product),
                Err(_) => Err(CBAdvError::BadParse("product object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains bulk products from the API.
    ///
    /// # Arguments
    ///
    /// * `query` - Query used to obtain products.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/products
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getproducts>
    pub async fn get_bulk(&self, query: &ListProductsQuery) -> Result<Vec<Product>> {
        match self.signer.get(Self::RESOURCE, &query.to_string()).await {
            Ok(value) => match value.json::<ListProductsResponse>().await {
                Ok(resp) => Ok(resp.products),
                Err(_) => Err(CBAdvError::BadParse("products vector".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains candles for a specific product.
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string the represents the product's ID.
    /// * `query` - Span of time to obtain.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/products/{product_id}/candles
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getcandles>
    pub async fn candles(&self, product_id: &str, query: &time::Span) -> Result<Vec<Candle>> {
        let resource = format!("{}/{}/candles", Self::RESOURCE, product_id);
        match self.signer.get(&resource, &query.to_string()).await {
            Ok(value) => match value.json::<CandleResponse>().await {
                Ok(resp) => Ok(resp.candles),
                Err(_) => Err(CBAdvError::BadParse("candle object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains candles for a specific product extended. This will exceed the 300 limit threshold
    /// and try to obtain the amount specified.
    ///
    /// NOTE: NOT A STANDARD API FUNCTION. QoL function that may require additional API requests than
    /// normal.
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string the represents the product's ID.
    /// * `query` - Span of time to obtain.
    pub async fn candles_ext(&self, product_id: &str, query: &time::Span) -> Result<Vec<Candle>> {
        let resource = format!("{}/{}/candles", Self::RESOURCE, product_id);

        // Make a copy of the query.
        let end = query.end;
        let interval = query.granularity as u64;
        let maximum = CANDLE_MAXIMUM as u64;
        let granularity = &time::Granularity::from_secs(query.granularity);

        // Create new span.
        let mut span = time::Span::new(query.start, end, granularity);
        span.end = time::after(query.start, interval * maximum);
        if span.end > end {
            span.end = end;
        }

        let mut candles: Vec<Candle> = vec![];
        while span.count() > 0 {
            match self.signer.get(&resource, &span.to_string()).await {
                Ok(value) => match value.json::<CandleResponse>().await {
                    Ok(resp) => candles.extend(resp.candles),
                    Err(_) => return Err(CBAdvError::BadParse("candle object".to_string())),
                },
                Err(error) => return Err(error),
            }

            // Update to get additional candles.
            span.start = span.end;
            span.end = time::after(span.start, interval as u64 * (CANDLE_MAXIMUM as u64));
            if span.end > end {
                span.end = end;
            }

            // Stop condition.
            if span.start > span.end {
                span.start = span.end;
            }
        }

        Ok(candles)
    }

    /// Obtains product ticker from the API.
    ///
    /// # Arguments
    ///
    /// * `product_id` - A string the represents the product's ID.
    /// * `query` - Amount of products to get.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/products/{product_id}/ticker
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getmarkettrades>
    pub async fn ticker(&self, product_id: &str, query: &TickerQuery) -> Result<Ticker> {
        let resource = format!("{}/{}/ticker", Self::RESOURCE, product_id);
        match self.signer.get(&resource, &query.to_string()).await {
            Ok(value) => match value.json::<Ticker>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse("ticker object".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
