//! # Coinbase Advanced Product API
//!
//! `product` gives access to the Product API and the various endpoints associated with it.
//! This allows you to obtain product information such as: Ticker (Market Trades), Product and
//! Currency information, Product Book, and Best Bids and Asks for multiple products.

use crate::constants::products::{
    BID_ASK_ENDPOINT, CANDLE_MAXIMUM, PRODUCT_BOOK_ENDPOINT, RESOURCE_ENDPOINT,
};
use crate::errors::CbAdvError;
use crate::http_agent::{HttpAgent, SecureHttpAgent};
use crate::models::product::{
    Candle, CandlesWrapper, ListProductsQuery, Product, ProductBook, ProductBookWrapper,
    ProductBooksWrapper, ProductsWrapper, Ticker, TickerQuery,
};
use crate::product::{BidAskQuery, ProductBookQuery};
use crate::time;
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Product API for the service.
pub struct ProductApi {
    /// Object used to sign requests made to the API.
    agent: SecureHttpAgent,
}

impl ProductApi {
    /// Creates a new instance of the Product API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: SecureHttpAgent) -> Self {
        Self { agent }
    }

    /// Obtains best bids and asks for a vector of product IDs..
    ///
    /// # Arguments
    ///
    /// * `product_ids` - A vector of strings the represents the product IDs of product books to obtain.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/best_bid_ask
    ///
    /// <https://docs.cloud.coinbase.com/advanced-trade-api/reference/retailbrokerageapi_getbestbidask>
    pub async fn best_bid_ask(&mut self, product_ids: Vec<String>) -> CbResult<Vec<ProductBook>> {
        let query = BidAskQuery { product_ids };
        let response = self.agent.get(BID_ASK_ENDPOINT, &query).await?;
        let data: ProductBooksWrapper = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.pricebooks)
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
    pub async fn product_book(
        &mut self,
        product_id: &str,
        limit: Option<u32>,
    ) -> CbResult<ProductBook> {
        let query = ProductBookQuery {
            product_id: product_id.to_string(),
            limit,
        };

        let response = self.agent.get(PRODUCT_BOOK_ENDPOINT, &query).await?;
        let data: ProductBookWrapper = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.pricebook)
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
    pub async fn get(&mut self, product_id: &str) -> CbResult<Product> {
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, product_id);
        let response = self.agent.get(&resource, &NoQuery).await?;
        let data: Product = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
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
    pub async fn get_bulk(&mut self, query: &ListProductsQuery) -> CbResult<Vec<Product>> {
        let response = self.agent.get(RESOURCE_ENDPOINT, query).await?;
        let data: ProductsWrapper = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.products)
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
        let response = self.agent.get(&resource, query).await?;
        let data: CandlesWrapper = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.candles)
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
        // Extract query parameters.
        let end_time = query.end;
        let interval_seconds = query.granularity as u64;
        let maximum_candles = CANDLE_MAXIMUM;
        let granularity = time::Granularity::from_secs(query.granularity);

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
            let current_span = time::Span::new(current_start, current_end, &granularity);
            let mut candles = self.candles(product_id, &current_span).await?;
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
    pub async fn ticker(&mut self, product_id: &str, query: &TickerQuery) -> CbResult<Ticker> {
        let resource = format!("{}/{}/ticker", RESOURCE_ENDPOINT, product_id);
        let response = self.agent.get(&resource, query).await?;
        let data: Ticker = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data)
    }
}
