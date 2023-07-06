//! # Coinbase Advanced Product API
//!
//! `product` gives access to the Product API and the various endpoints associated with it.
//! This allows you to obtain product information such as: Ticker (Market Trades), Product and
//! Currency information, Product Book, and Best Bids and Asks for multiple products.

use crate::signer::Signer;
use crate::time;
use crate::utils::{CBAdvError, Result};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::fmt;

/// Represents a Product received from the Websocket API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductUpdate {
    pub product_type: String,
    pub id: String,
    pub base_currency: String,
    pub quote_currency: String,
    #[serde_as(as = "DisplayFromStr")]
    pub base_increment: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub quote_increment: f64,
    pub display_name: String,
    pub status: String,
    pub status_message: String,
    #[serde_as(as = "DisplayFromStr")]
    pub min_market_funds: f64,
}

/// Represents a Market Trade received from the Websocket API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketTradesUpdate {
    pub trade_id: String,
    pub product_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    pub side: String,
    pub time: String,
}

/// Represents a Product received from the REST API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub product_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price_percentage_change_24h: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub volume_24h: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub volume_percentage_change_24h: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub base_increment: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub quote_increment: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub quote_min_size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub quote_max_size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub base_min_size: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub base_max_size: f64,

    pub base_name: String,
    pub quote_name: String,
    pub watched: bool,
    pub is_disabled: bool,
    pub new: bool,
    pub status: String,
    pub cancel_only: bool,
    pub limit_only: bool,
    pub post_only: bool,
    pub trading_disabled: bool,
    pub auction_mode: bool,
    pub product_type: String,
    pub quote_currency_id: String,
    pub base_currency_id: String,
    pub fcm_trading_session_details: Option<String>,
    pub mid_market_price: String,
    pub alias: String,
    pub alias_to: Vec<String>,
    pub base_display_symbol: String,
    pub quote_display_symbol: String,
    pub view_only: bool,
    #[serde_as(as = "DisplayFromStr")]
    pub price_increment: f64,
}

/// Represents a Bid or an Ask entry for a product.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct BidAsk {
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
}

/// Represents a product book for a product.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductBook {
    pub product_id: String,
    pub time: String,
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
}

/// Represents a candle for a product.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Candle {
    #[serde_as(as = "DisplayFromStr")]
    pub start: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub low: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub high: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub open: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub close: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub volume: f64,
}

/// Represents a trade for a product.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub trade_id: String,
    pub product_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub size: f64,
    pub time: String,
    pub side: String,
    // NOTE: (20230705) API gives an empty string not a number.
    pub bid: String,
    // NOTE: (20230705) API gives an empty string not a number.
    pub ask: String,
}

/// Represents a Ticker update received from the Websocket API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct TickerUpdate {
    pub r#type: String,
    pub product_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub price: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub volume_24_h: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub low_24_h: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub high_24_h: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub low_52_w: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub high_52_w: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub price_percent_chg_24_h: f64,
}

/// Represents a ticker for a product.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    pub trades: Vec<Trade>,
    #[serde_as(as = "DisplayFromStr")]
    pub best_bid: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub best_ask: f64,
}

/// Represents a list of Products received from the API.
#[derive(Serialize, Deserialize, Debug)]
struct ListProductsResponse {
    pub products: Vec<Product>,
    pub num_products: i32,
}

/// Represents a candle response from the API.
#[derive(Serialize, Deserialize, Debug)]
struct CandleResponse {
    pub candles: Vec<Candle>,
}

/// Represents a best bid and ask response from the API.
#[derive(Serialize, Deserialize, Debug)]
struct BidAskResponse {
    pub pricebooks: Vec<ProductBook>,
}

/// Represents a product book response from the API.
#[derive(Serialize, Deserialize, Debug)]
struct ProductBookResponse {
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
    pub fn new(signer: Signer) -> Self {
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

    /// Obtains product ticker from the API.
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
