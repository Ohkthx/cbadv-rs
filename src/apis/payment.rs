//! # Coinbase Advanced Payment API
//!
//! `payment` gives access to the Payment API and the various endpoints associated with it.

use crate::constants::payments::RESOURCE_ENDPOINT;
use crate::errors::CbAdvError;
use crate::http_agent::{HttpAgent, SecureHttpAgent};
use crate::models::payment::{PaymentMethod, PaymentMethodWrapper, PaymentMethodsWrapper};
use crate::traits::NoQuery;
use crate::types::CbResult;

/// Provides access to the Payment API for the service.
pub struct PaymentApi {
    /// Object used to sign requests made to the API.
    agent: SecureHttpAgent,
}

impl PaymentApi {
    /// Creates a new instance of the Payment API. This grants access to payment information.
    ///
    /// # Arguments
    ///
    /// * `agent` - A agent that include the API Key & Secret along with a client to make requests.
    pub(crate) fn new(agent: SecureHttpAgent) -> Self {
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
        let response = self.agent.get(RESOURCE_ENDPOINT, &NoQuery).await?;
        let data: PaymentMethodsWrapper = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.payment_methods)
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
        let resource = format!("{}/{}", RESOURCE_ENDPOINT, payment_method_id);
        let response = self.agent.get(&resource, &NoQuery).await?;
        let data: PaymentMethodWrapper = response
            .json()
            .await
            .map_err(|e| CbAdvError::JsonError(e.to_string()))?;
        Ok(data.payment_method)
    }
}
