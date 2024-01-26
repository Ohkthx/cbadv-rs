//! # Coinbase Advanced Product API
//!
//! `product` gives access to the Product API and the various endpoints associated with it.
//! This allows you to obtain product information such as: Ticker (Market Trades), Product and
//! Currency information, Product Book, and Best Bids and Asks for multiple products.

use crate::constants::products::{
    BID_ASK_ENDPOINT, CANDLE_MAXIMUM, PRODUCT_BOOK_ENDPOINT, RESOURCE_ENDPOINT,
};
use crate::errors::CbAdvError;
use crate::models::product::{
    BidAskResponse, Candle, CandleResponse, ListProductsQuery, ListProductsResponse, Product,
    ProductBook, ProductBookResponse, Ticker, TickerQuery,
};
use crate::product::{BidAskQuery, ProductBookQuery};
use crate::signer::Signer;
use crate::time;
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Product API for the service.
pub struct ProductApi {
    /// Object used to sign requests made to the API.
    signer: Signer,
}

impl ProductApi {
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
    pub async fn best_bid_ask(&mut self, product_ids: Vec<String>) -> CbResult<Vec<ProductBook>> {
        let query = BidAskQuery { product_ids };

        match self.signer.get(BID_ASK_ENDPOINT, &query).await {
            Ok(value) => match value.json::<BidAskResponse>().await {
                Ok(bidasks) => Ok(bidasks.pricebooks),
                Err(_) => Err(CbAdvError::BadParse("bid asks object".to_string())),
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
    pub async fn product_book(
        &mut self,
        product_id: &str,
        limit: Option<u32>,
    ) -> CbResult<ProductBook> {
        let query = ProductBookQuery {
            product_id: product_id.to_string(),
            limit,
        };

        match self.signer.get(PRODUCT_BOOK_ENDPOINT, &query).await {
            Ok(value) => match value.json::<ProductBookResponse>().await {
                Ok(book) => Ok(book.pricebook),
                Err(_) => Err(CbAdvError::BadParse("product book object".to_string())),
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
    pub async fn get(&mut self, product_id: &str) -> CbResult<Product> {
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, product_id);
        match self.signer.get(&resource, &NoQuery).await {
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
    pub async fn get_bulk(&mut self, query: &ListProductsQuery) -> CbResult<Vec<Product>> {
        match self.signer.get(RESOURCE_ENDPOINT, query).await {
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
        match self.signer.get(&resource, query).await {
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
            match self.signer.get(&resource, &span).await {
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
        match self.signer.get(&resource, query).await {
            Ok(value) => match value.json::<Ticker>().await {
                Ok(resp) => Ok(resp),
                Err(_) => Err(CbAdvError::BadParse("ticker object".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
