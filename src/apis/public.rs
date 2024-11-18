//! # Coinbase Advanced Public API
//!
//! `public` gives access to the Public API and the various endpoints associated with it.
//! Some of the features include getting the API current time in ISO format.

use crate::constants::products::CANDLE_MAXIMUM;
use crate::constants::public::{PRODUCT_BOOK_ENDPOINT, RESOURCE_ENDPOINT, SERVERTIME_ENDPOINT};
use crate::errors::CbAdvError;
use crate::http_agent::{HttpAgent, PublicHttpAgent};
use crate::models::product::{
    Candle, CandleResponse, ListProductsQuery, ListProductsResponse, Product, ProductBook,
    ProductBookResponse, Ticker, TickerQuery,
};
use crate::models::public::ServerTime;
use crate::product::ProductBookQuery;
use crate::time;
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Public API for the service.
pub struct PublicApi {
    /// Object used to sign requests made to the API.
    agent: PublicHttpAgent,
}

impl PublicApi {
    /// Creates a new instance of the Public API. This grants access to public information that requires no authentication.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: PublicHttpAgent) -> Self {
        Self { agent }
    }

    /// Get the current time from the Coinbase Advanced API.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/time
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_getservertime>
    pub async fn server_time(&mut self) -> CbResult<ServerTime> {
        match self.agent.get(SERVERTIME_ENDPOINT, &NoQuery).await {
            Ok(value) => match value.json::<ServerTime>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CbAdvError::BadParse("public servertime object".to_string())),
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
    /// https://api.coinbase.com/api/v3/brokerage/market/product_book
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_getpublicproductbook>
    pub async fn product_book(
        &mut self,
        product_id: &str,
        limit: Option<u32>,
    ) -> CbResult<ProductBook> {
        let query = ProductBookQuery {
            product_id: product_id.to_string(),
            limit,
        };

        match self.agent.get(PRODUCT_BOOK_ENDPOINT, &query).await {
            Ok(value) => match value.json::<ProductBookResponse>().await {
                Ok(book) => Ok(book.pricebook),
                Err(err) => Err(CbAdvError::BadParse(format!(
                    "product book object: {err:?}"
                ))),
                // Err(_) => Err(CbAdvError::BadParse("product book object".to_string())),
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
    pub async fn product(&mut self, product_id: &str) -> CbResult<Product> {
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, product_id);
        match self.agent.get(&resource, &NoQuery).await {
            Ok(value) => match value.json::<Product>().await {
                Ok(product) => Ok(product),
                Err(_) => Err(CbAdvError::BadParse("product object".to_string())),
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
    pub async fn products(&mut self, query: &ListProductsQuery) -> CbResult<Vec<Product>> {
        match self.agent.get(RESOURCE_ENDPOINT, query).await {
            Ok(value) => match value.json::<ListProductsResponse>().await {
                Ok(resp) => Ok(resp.products),
                Err(_) => Err(CbAdvError::BadParse("products vector".to_string())),
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
    pub async fn candles(&mut self, product_id: &str, query: &time::Span) -> CbResult<Vec<Candle>> {
        let resource = format!("{}/{}/candles", RESOURCE_ENDPOINT, product_id);
        match self.agent.get(&resource, query).await {
            Ok(value) => match value.json::<CandleResponse>().await {
                Ok(resp) => Ok(resp.candles),
                Err(_) => Err(CbAdvError::BadParse("candle object".to_string())),
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
    pub async fn candles_ext(
        &mut self,
        product_id: &str,
        query: &time::Span,
    ) -> CbResult<Vec<Candle>> {
        let resource = format!("{}/{}/candles", RESOURCE_ENDPOINT, product_id);

        // Make a copy of the query.
        let end = query.end;
        let interval = query.granularity as u64;
        let maximum = CANDLE_MAXIMUM;
        let granularity = &time::Granularity::from_secs(query.granularity);

        // Create new span.
        let mut span = time::Span::new(query.start, end, granularity);
        span.end = time::after(query.start, interval * maximum);
        if span.end > end {
            span.end = end;
        }

        let mut candles: Vec<Candle> = vec![];
        while span.count() > 0 {
            match self.agent.get(&resource, &span).await {
                Ok(value) => match value.json::<CandleResponse>().await {
                    Ok(resp) => candles.extend(resp.candles),
                    Err(_) => return Err(CbAdvError::BadParse("candle object".to_string())),
                },
                Err(error) => return Err(error),
            }

            // Update to get additional candles.
            span.start = span.end;
            span.end = time::after(span.start, interval * CANDLE_MAXIMUM);
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
    pub async fn ticker(&mut self, product_id: &str, query: &TickerQuery) -> CbResult<Ticker> {
        let resource = format!("{}/{}/ticker", RESOURCE_ENDPOINT, product_id);
        match self.agent.get(&resource, query).await {
            Ok(value) => match value.json::<Ticker>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CbAdvError::BadParse("ticker object".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
