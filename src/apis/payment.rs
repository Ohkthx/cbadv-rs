//! # Coinbase Advanced Payment API
//!
//! `payment` gives access to the Payment API and the various endpoints associated with it.

use crate::constants::payments::RESOURCE_ENDPOINT;
use crate::errors::CbError;
use crate::http_agent::SecureHttpAgent;
use crate::models::payment::{PaymentMethod, PaymentMethodWrapper, PaymentMethodsWrapper};
use crate::traits::{HttpAgent, NoQuery};
use crate::types::CbResult;

/// Provides access to the Payment API for the service.
pub struct PaymentApi {
    /// Object used to sign requests made to the API.
    agent: Option<SecureHttpAgent>,
}

impl PaymentApi {
    /// Creates a new instance of the Payment API. This grants access to payment information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: Option<SecureHttpAgent>) -> Self {
        Self { agent }
    }

    /// Obtains a list of payment methods for the current user from the API.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/payment_methods
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_getpaymentmethods>
    pub async fn get_all(&mut self) -> CbResult<Vec<PaymentMethod>> {
        let agent = get_auth!(self.agent, "get all payment methods");
        let response = agent.get(RESOURCE_ENDPOINT, &NoQuery).await?;
        let data: PaymentMethodsWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
    }

    /// Obtains a single payment method by its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `payment_method_id` - The unique identifier for the payment method.
    ///
    /// # Endpoint / Reference
    ///
    #[allow(rustdoc::bare_urls)]
    /// https://api.coinbase.com/api/v3/brokerage/payment_methods
    ///
    /// <https://docs.cdp.coinbase.com/advanced-trade/reference/retailbrokerageapi_getpaymentmethod>
    pub async fn get(&mut self, payment_method_id: &str) -> CbResult<PaymentMethod> {
        let agent = get_auth!(self.agent, "get payment method");
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, payment_method_id);
        let response = agent.get(&resource, &NoQuery).await?;
        let data: PaymentMethodWrapper = response
            .json()
            .await
            .map_err(|e| CbError::JsonError(e.to_string()))?;
        Ok(data.into())
    }
}
