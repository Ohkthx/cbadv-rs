//! Traits used to allow interfacing with advanced functionality for end-users.

use reqwest::Response;
use serde::Serialize;

use crate::types::CbResult;

/// Used to pass query/paramters for a URL.
pub(crate) trait Query {
    /// Checks that the query is valid and the required fields are present.
    fn check(&self) -> CbResult<()>;
    /// Used to convert a struct into query/paramters for a URL.
    fn to_query(&self) -> String;
}

/// Used to pass a request body to an endpoint.
pub(crate) trait Request {
    /// Checks that the request is valid and the required fields are present.
    fn check(&self) -> CbResult<()>;
}

/// Represents an empty query.
pub(crate) struct NoQuery;
impl Query for NoQuery {
    fn check(&self) -> CbResult<()> {
        Ok(())
    }

    fn to_query(&self) -> String {
        String::new()
    }
}

/// Trait for the `HttpAgent` that is responsible for making HTTP requests and managing the token bucket.
pub(crate) trait HttpAgent {
    /// Performs a HTTP GET Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    async fn get(&mut self, resource: &str, query: &impl Query) -> CbResult<Response>;

    /// Performs a HTTP POST Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    /// * `body` - An object to send to the URL via POST request.
    async fn post<'a, T>(
        &mut self,
        resource: &str,
        query: &impl Query,
        body: &'a T,
    ) -> CbResult<Response>
    where
        T: Request + Serialize + 'a;

    /// Performs a HTTP PUT Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    /// * `body` - An object to send to the URL via POST request.
    async fn put<'a, T>(
        &mut self,
        resource: &str,
        query: &impl Query,
        body: &'a T,
    ) -> CbResult<Response>
    where
        T: Request + Serialize + 'a;

    /// Performs a HTTP DELETE Request.
    ///
    /// # Arguments
    ///
    /// * `resource` - A string representing the resource that is being accessed.
    /// * `query` - A string containing options / parameters for the URL.
    async fn delete(&mut self, resource: &str, query: &impl Query) -> CbResult<Response>;
}
