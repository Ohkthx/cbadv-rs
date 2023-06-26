use crate::cbadv::time;
use crate::cbadv::utils::{CBAdvError, Result, Signer};
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
            Some(v) => format!("{}&product_ids={}", params, v.join(",")),
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
        let resource = Self::RESOURCE.to_string();
        match self.signer.get(resource, params.to_params()).await {
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
        match self.signer.get(resource, params.to_params()).await {
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
        match self.signer.get(resource, params.to_params()).await {
            Ok(value) => match value.json::<Ticker>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CBAdvError::BadParse("ticker object".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
