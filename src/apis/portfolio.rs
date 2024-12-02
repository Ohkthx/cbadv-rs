//! # Coinbase Advanced Portfolio API
//!
//! `portfolio` gives access to the Portfolio API and the various endpoints associated with it.
//! This allows for the management of individual portfolios.

use crate::constants::portfolios::{MOVE_FUNDS_ENDPOINT, RESOURCE_ENDPOINT};
use crate::errors::CbError;
use crate::http_agent::SecureHttpAgent;
use crate::models::portfolio::{
    Portfolio, PortfolioBreakdown, PortfolioBreakdownQuery, PortfolioBreakdownWrapper,
    PortfolioListQuery, PortfolioModifyRequest, PortfolioMoveFundsRequest, PortfolioWrapper,
    PortfoliosWrapper,
};
use crate::traits::{HttpAgent, NoQuery};
use crate::types::CbResult;

/// Provides access to the Portfolio API for the service.
pub struct PortfolioApi {
    /// Object used to sign requests made to the API.
    agent: Option<SecureHttpAgent>,
}

impl PortfolioApi {
    /// Creates a new instance of the Portfolio API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: Option<SecureHttpAgent>) -> Self {
        Self { agent }
    }

    /// Obtains various portfolios from the API.
    ///
    /// # Arguments
    ///
    /// * `query` - The query parameters to filter the results.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/portfolios
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_getportfolios>
    pub async fn get_all(&mut self, query: &PortfolioListQuery) -> CbResult<Vec<Portfolio>> {
        let agent = get_auth!(self.agent, "get all portfolios");
        let response = agent.get(RESOURCE_ENDPOINT, query).await?;
        let data: PortfoliosWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
    }

    /// Creates a new portfolio.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to create a new portfolio.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/portfolios
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_createportfolio>
    pub async fn create(&mut self, request: &PortfolioModifyRequest) -> CbResult<Portfolio> {
        let agent = get_auth!(self.agent, "create portfolio");
        let response = agent.post(RESOURCE_ENDPOINT, &NoQuery, request).await?;
        let data: PortfolioWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
    }

    /// Edits an existing portfolio.
    ///
    /// # Arguments
    ///
    /// * `portfolio_uuid` - The UUID of the portfolio to edit.
    /// * `request` - The request to edit the portfolio.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/portfolios
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_editportfolio>
    pub async fn edit(
        &mut self,
        portfolio_uuid: &str,
        request: &PortfolioModifyRequest,
    ) -> CbResult<Portfolio> {
        let agent = get_auth!(self.agent, "edit portfolio");
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, portfolio_uuid);
        let response = agent.put(&resource, &NoQuery, request).await?;
        let data: PortfolioWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
    }

    /// Edits an existing portfolio.
    ///
    /// # Arguments
    ///
    /// * `portfolio_uuid` - The UUID of the portfolio to delete.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/portfolios
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_editportfolio>
    pub async fn delete(&mut self, portfolio_uuid: &str) -> CbResult<()> {
        let agent = get_auth!(self.agent, "delete portfolio");
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, portfolio_uuid);
        agent.delete(&resource, &NoQuery).await?;
        Ok(())
    }

    /// Move funds from a source portfolio to a target portfolio.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to move funds.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/portfolios/move_funds
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_moveportfoliofunds>
    pub async fn move_funds(&mut self, request: &PortfolioMoveFundsRequest) -> CbResult<()> {
        let agent = get_auth!(self.agent, "move funds");
        agent.post(MOVE_FUNDS_ENDPOINT, &NoQuery, request).await?;
        Ok(())
    }

    /// Obtains a breakdown of a specific portfolio.
    ///
    /// # Arguments
    ///
    /// * `portfolio_uuid` - The UUID of the portfolio to obtain a breakdown for.
    /// * `query` - The query parameters to filter the results.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/portfolios
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_getportfoliobreakdown>
    pub async fn get(
        &mut self,
        portfolio_uuid: &str,
        query: &PortfolioBreakdownQuery,
    ) -> CbResult<PortfolioBreakdown> {
        let agent = get_auth!(self.agent, "get portfolio breakdown");
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, portfolio_uuid);
        let response = agent.get(&resource, query).await?;
        let data: PortfolioBreakdownWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
    }
}
