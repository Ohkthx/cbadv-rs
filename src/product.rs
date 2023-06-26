use crate::time;
use crate::utils::{CBAdvError, Result, Signer};
use serde::{Deserialize, Serialize};

/// Represents a Product received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub product_id: String,
    pub price: String,
    pub price_percentage_change_24h: String,
    pub volume_24h: String,
    pub volume_percentage_change_24h: String,
    pub base_increment: String,
    pub quote_increment: String,
    pub quote_min_size: String,
    pub quote_max_size: String,
    pub base_min_size: String,
    pub base_max_size: String,
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
}

/// Represents a Bid or an Ask entry for a product.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct BidAsk {
    pub price: String,
    pub size: String,
}

/// Represents a product book for a product.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductBook {
    pub product_id: String,
    pub time: String,
    pub bids: Vec<BidAsk>,
    pub asks: Vec<BidAsk>,
}

/// Represents a candle for a product.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Candle {
    pub start: String,
    pub low: String,
    pub high: String,
    pub open: String,
    pub close: String,
    pub volume: String,
}

/// Represents a trade for a product.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub trade_id: String,
    pub product_id: String,
    pub price: String,
    pub size: String,
    pub time: String,
    pub side: String,
    pub bid: String,
    pub ask: String,
}

/// Represents a ticker for a product.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    pub trades: Vec<Trade>,
    pub best_bid: String,
    pub best_ask: String,
}

/// Represents a list of Products received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct ListProductsResponse {
    pub products: Vec<Product>,
    pub num_products: i32,
}

/// Represents a candle response from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct CandleResponse {
    pub candles: Vec<Candle>,
}

/// Represents a best bid and ask response from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct BidAskResponse {
    pub pricebooks: Vec<ProductBook>,
}

/// Represents a product book response from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct ProductBookResponse {
    pub pricebook: ProductBook,
}

/// Represents parameters that are optional for List Products API request.
#[allow(dead_code)]
#[derive(Serialize, Default, Debug)]
pub struct ListProductsParams {
    /// A limit describing how many products to return.
    pub limit: Option<i32>,
    /// Number of products to offset before returning.
    pub offset: Option<i32>,
    /// Type of products to return. Valid options: SPOT or FUTURE
    pub product_type: Option<String>,
    /// List of product IDs to return.
    pub product_ids: Option<Vec<String>>,
}

impl ListProductsParams {
    /// Converts the object into HTTP request parameters.
    pub fn to_params(&self) -> String {
        let mut params: String = "".to_string();

        params = match &self.limit {
            Some(v) => format!("{}&limit={}", params, v),
            _ => params,
        };

        params = match &self.offset {
            Some(v) => format!("{}&offset={}", params, v),
            _ => params,
        };

        params = match &self.product_type {
            Some(v) => format!("{}&product_type={}", params, v),
            _ => params,
        };

        params = match &self.product_ids {
            Some(v) => format!("{}&product_ids={}", params, v.join("&product_ids=")),
            _ => params,
        };

        match params.is_empty() {
            true => params,
            false => params[1..].to_string(),
        }
    }
}

/// Represents parameters for Ticker Product API request.
#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub struct TickerParams {
    /// Number of trades to return.
    pub limit: i32,
}

impl TickerParams {
    /// Converts the object into HTTP request parameters.
    pub fn to_params(&self) -> String {
        format!("limit={}", self.limit)
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
    /// https://api.coinbase.com/api/v3/brokerage/best_bid_ask
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getbestbidask
    pub async fn best_bid_ask(&self, product_ids: Vec<String>) -> Result<Vec<ProductBook>> {
        let resource = "/api/v3/brokerage/best_bid_ask";
        let params = format!("product_ids={}", product_ids.join("&product_ids="));

        match self.signer.get(resource, &params).await {
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
    /// https://api.coinbase.com/api/v3/brokerage/product_book
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getproductbook
    pub async fn product_book(
        &self,
        product_id: String,
        limit: Option<u16>,
    ) -> Result<ProductBook> {
        let resource = "/api/v3/brokerage/product_book";
        let params = format!("product_id={}&limit={}", product_id, limit.unwrap_or(250));

        match self.signer.get(resource, &params).await {
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
    /// https://api.coinbase.com/api/v3/brokerage/products/{product_id}
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getproduct
    pub async fn get(&self, product_id: String) -> Result<Product> {
        let resource = format!("{}/{}", Self::RESOURCE, product_id);
        match self.signer.get(&resource, "").await {
            Ok(value) => match value.json::<Product>().await {
                Ok(product) => Ok(product),
                Err(_) => Err(CBAdvError::BadParse("product object".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Obtains all products from the API.
    ///
    /// # Endpoint / Reference
    ///
    /// https://api.coinbase.com/api/v3/brokerage/products
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getproducts
    pub async fn get_all(&self, params: ListProductsParams) -> Result<Vec<Product>> {
        match self.signer.get(Self::RESOURCE, &params.to_params()).await {
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
    /// https://api.coinbase.com/api/v3/brokerage/products/{product_id}/candles
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getcandles
    pub async fn candles(&self, product_id: String, params: time::Span) -> Result<Vec<Candle>> {
        let resource = format!("{}/{}/candles", Self::RESOURCE, product_id);
        match self.signer.get(&resource, &params.to_params()).await {
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
    /// https://api.coinbase.com/api/v3/brokerage/products/{product_id}/ticker
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getmarkettrades
    pub async fn ticker(&self, product_id: String, params: TickerParams) -> Result<Ticker> {
        let resource = format!("{}/{}/ticker", Self::RESOURCE, product_id);
        match self.signer.get(&resource, &params.to_params()).await {
            Ok(value) => match value.json::<Ticker>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse("ticker object".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
