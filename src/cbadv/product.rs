use crate::cbadv::time;
use crate::cbadv::utils::Signer;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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
    // pub fcm_trading_session_details: Option<String>,
    pub mid_market_price: String,
    // pub alias: String,
    // pub alias_to: String,
    pub base_display_symbol: String,
    pub quote_display_symbol: String,
    pub view_only: bool,
}

/// Represents a list of Products received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct ListProducts {
    pub products: Vec<Product>,
    pub num_products: i32,
}

/// Represents parameters that are optional for List Product API request.
#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub struct ListProductParams {
    pub limit: i32,
    pub offset: i32,
    pub product_type: String,
}

impl ListProductParams {
    pub fn to_params(&self) -> String {
        format!(
            "limit={}&offset={}&product_type={}",
            self.limit, self.offset, self.product_type
        )
    }
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

/// Represents a candle response from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct CandleResponse {
    pub candles: Vec<Candle>,
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

/// Represents parameters for Ticker Product API request.
#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub struct TickerParams {
    pub limit: i32,
}

impl TickerParams {
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
        let resource = format!("{}/{}", Self::RESOURCE.to_string(), product_id);
        match self.signer.get(resource, "".to_string()).await {
            Ok(value) => Ok(value.json().await?),
            Err(error) => {
                println!("Failed to get product: {}", error);
                Err(error)
            }
        }
    }

    /// Obtains all products from the API.
    ///
    /// # Endpoint / Reference
    ///
    /// https://api.coinbase.com/api/v3/brokerage/products
    /// https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getproducts
    pub async fn get_all(&self, params: ListProductParams) -> Result<Vec<Product>> {
        let resource = Self::RESOURCE.to_string();
        match self.signer.get(resource, params.to_params()).await {
            Ok(value) => match value.json::<ListProducts>().await {
                Ok(resp) => Ok(resp.products),
                Err(error) => Err(Box::new(error)),
            },
            Err(error) => {
                println!("Failed to get all products: {}", error);
                Err(error)
            }
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
        match self.signer.get(resource, params.to_params()).await {
            Ok(value) => match value.json::<CandleResponse>().await {
                Ok(resp) => Ok(resp.candles),
                Err(error) => Err(Box::new(error)),
            },
            Err(error) => {
                println!("Failed to get the candles: {}", error);
                Err(error)
            }
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
        match self.signer.get(resource, params.to_params()).await {
            Ok(value) => match value.json::<Ticker>().await {
                Ok(resp) => Ok(resp),
                Err(error) => Err(Box::new(error)),
            },
            Err(error) => {
                println!("Failed to get product ticker: {}", error);
                Err(error)
            }
        }
    }
}
