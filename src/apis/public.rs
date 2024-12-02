//! # Coinbase Advanced Public API
//!
//! `public` gives access to the Public API and the various endpoints associated with it.
//! Some of the features include getting the API current time in ISO format.

use crate::constants::products::CANDLE_MAXIMUM;
use crate::constants::public::{PRODUCT_BOOK_ENDPOINT, RESOURCE_ENDPOINT, SERVERTIME_ENDPOINT};
use crate::errors::CbError;
use crate::http_agent::PublicHttpAgent;
use crate::models::product::{
    Candle, CandlesWrapper, Product, ProductBook, ProductBookWrapper, ProductListQuery,
    ProductTickerQuery, ProductsWrapper, Ticker,
};
use crate::models::product::{ProductBookQuery, ProductCandleQuery};
use crate::models::public::ServerTime;
use crate::time::{self, Granularity};
use crate::traits::{HttpAgent, NoQuery, Query};
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
    pub async fn time(&mut self) -> CbResult<ServerTime> {
        let response = self.agent.get(SERVERTIME_ENDPOINT, &NoQuery).await?;
        let data: ServerTime = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
    }

    /// Obtains the product book (bids and asks) for the product ID provided.
    ///
    /// # Arguments
    ///
    /// * `query` - Query used to obtain the product book.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/market/product_book
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_getpublicproductbook>
    pub async fn product_book(&mut self, query: &ProductBookQuery) -> CbResult<ProductBook> {
        let response = self.agent.get(PRODUCT_BOOK_ENDPOINT, query).await?;
        let data: ProductBookWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
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
        let response = self.agent.get(&resource, &NoQuery).await?;
        let data: Product = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
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
    pub async fn products(&mut self, query: &ProductListQuery) -> CbResult<Vec<Product>> {
        let response = self.agent.get(RESOURCE_ENDPOINT, query).await?;
        let data: ProductsWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
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
    pub async fn candles(
        &mut self,
        product_id: &str,
        query: &ProductCandleQuery,
    ) -> CbResult<Vec<Candle>> {
        let resource = format!("{}/{}/candles", RESOURCE_ENDPOINT, product_id);
        let response = self.agent.get(&resource, query).await?;
        let data: CandlesWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
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
        query: &ProductCandleQuery,
    ) -> CbResult<Vec<Candle>> {
        query.check()?;

        // Extract query parameters.
        let end_time = query.end;
        let granularity = query.granularity.clone();
        let interval_seconds = Granularity::to_secs(&granularity) as u64;
        let maximum_candles = CANDLE_MAXIMUM as u64;

        // Initialize the span.
        let mut current_start = query.start;
        let mut all_candles: Vec<Candle> = Vec::new();

        while current_start < end_time {
            // Calculate the end time for the current batch.
            let current_end = std::cmp::min(
                time::after(current_start, interval_seconds * maximum_candles),
                end_time,
            );

            // Create a new span for the current batch and fetch candles.
            let query = ProductCandleQuery {
                start: current_start,
                end: current_end,
                granularity: granularity.clone(),
                limit: CANDLE_MAXIMUM,
            };

            let mut candles = self.candles(product_id, &query).await?;
            all_candles.append(&mut candles);

            // Update the start time for the next batch.
            current_start = current_end;
        }

        Ok(all_candles)
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
    pub async fn ticker(
        &mut self,
        product_id: &str,
        query: &ProductTickerQuery,
    ) -> CbResult<Ticker> {
        let resource = format!("{}/{}/ticker", RESOURCE_ENDPOINT, product_id);
        let response = self.agent.get(&resource, query).await?;
        let data: Ticker = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data)
    }
}
