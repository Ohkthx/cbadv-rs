//! # Coinbase Advanced Portfolio API
//!
//! `portfolio` gives access to the Portfolio API and the various endpoints associated with it.
//! This allows for the management of individual portfolios.

use crate::constants::portfolios::{MOVE_FUNDS_ENDPOINT, RESOURCE_ENDPOINT};
use crate::errors::CbAdvError;
use crate::http_agent::{HttpAgent, SecureHttpAgent};
use crate::models::portfolio::{ListPortfoliosQuery, ListedPortfolios, Portfolio};
use crate::portfolio::{
    CreatePortfolio, EditPortfolio, MoveFunds, PortfolioBreakdown, PortfolioBreakdownQuery,
    PortfolioBreakdownResponse, PortfolioResponse,
};
use crate::shared::Balance;
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Portfolio API for the service.
pub struct PortfolioApi {
    /// Object used to sign requests made to the API.
    agent: SecureHttpAgent,
}

impl PortfolioApi {
    /// Creates a new instance of the Portfolio API. This grants access to product information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: SecureHttpAgent) -> Self {
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
    pub async fn get_all(&mut self, query: &ListPortfoliosQuery) -> CbResult<Vec<Portfolio>> {
        match self.agent.get(RESOURCE_ENDPOINT, query).await {
            Ok(value) => match value.json::<ListedPortfolios>().await {
                Ok(resp) => Ok(resp.portfolios),
                Err(_) => Err(CbAdvError::BadParse("portfolios vector".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Creates a new portfolio.
    ///
    /// # Arguments
    ///
    /// * `portfolio_name` - The name of the portfolio to create.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/portfolios
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_createportfolio>
    pub async fn create(&mut self, portfolio_name: &str) -> CbResult<Portfolio> {
        let body = CreatePortfolio {
            name: portfolio_name.to_string(),
        };

        match self.agent.post(RESOURCE_ENDPOINT, &NoQuery, body).await {
            Ok(value) => match value.json::<PortfolioResponse>().await {
                Ok(resp) => Ok(resp.portfolio),
                Err(_) => Err(CbAdvError::BadParse("portfolio".to_string())),
            },
            Err(error) => Err(error),
        }
    }

    /// Edits an existing portfolio.
    ///
    /// # Arguments
    ///
    /// * `portfolio_uuid` - The UUID of the portfolio to edit.
    /// * `new_name` - The new name of the portfolio.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/portfolios
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_editportfolio>
    pub async fn edit(&mut self, portfolio_uuid: &str, new_name: &str) -> CbResult<Portfolio> {
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, portfolio_uuid);
        let body = EditPortfolio {
            name: new_name.to_string(),
        };

        match self.agent.put(&resource, &NoQuery, body).await {
            Ok(value) => match value.json::<PortfolioResponse>().await {
                Ok(resp) => Ok(resp.portfolio),
                Err(_) => Err(CbAdvError::BadParse("portfolio".to_string())),
            },
            Err(error) => Err(error),
        }
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
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, portfolio_uuid);
        match self.agent.delete(&resource, &NoQuery).await {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Move funds from a source portfolio to a target portfolio.
    ///
    /// # Arguments
    ///
    /// * `funds` - The amount of funds to move.
    /// * `source_portfolio_uuid` - The UUID of the source portfolio.
    /// * `target_portfolio_uuid` - The UUID of the target portfolio.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/portfolios/move_funds
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_moveportfoliofunds>
    pub async fn move_funds(
        &mut self,
        funds: &Balance,
        source_portfolio_uuid: &str,
        target_portfolio_uuid: &str,
    ) -> CbResult<()> {
        let body = MoveFunds {
            funds: funds.clone(),
            source_portfolio_uuid: source_portfolio_uuid.to_string(),
            target_portfolio_uuid: target_portfolio_uuid.to_string(),
        };

        match self.agent.post(MOVE_FUNDS_ENDPOINT, &NoQuery, body).await {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    /// Obtains a breakdown of a specific portfolio.
    ///
    /// # Arguments
    ///
    /// * `portfolio_uuid` - The UUID of the portfolio to obtain a breakdown for.
    /// * `currency` - The currency to obtain the breakdown in.
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
        currency: Option<String>,
    ) -> CbResult<PortfolioBreakdown> {
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, portfolio_uuid);
        let query = PortfolioBreakdownQuery { currency };
        match self.agent.get(&resource, &query).await {
            Ok(value) => match value.json::<PortfolioBreakdownResponse>().await {
                Ok(resp) => Ok(resp.breakdown),
                Err(_) => Err(CbAdvError::BadParse("portfolio breakdown".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
