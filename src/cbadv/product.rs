use crate::cbadv::utils::Signer;
use reqwest::Method;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Represents a list of Products received from the API.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ListProducts {
    pub products: Vec<Product>,
    pub num_products: i32,
}

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
    pub async fn get(&self, product_id: String) -> Result<Product> {
        let res = self
            .signer
            .request(Method::GET, Self::RESOURCE, product_id)
            .await?
            .json()
            .await?;
        Ok(res)
    }

    /// Obtains all products from the API.
    pub async fn get_all(&self) -> Result<ListProducts> {
        let res = self
            .signer
            .request(Method::GET, Self::RESOURCE, "".to_string())
            .await?
            .json()
            .await?;
        Ok(res)
    }
}
