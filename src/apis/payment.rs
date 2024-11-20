//! # Coinbase Advanced Payment API
//!
//! `payment` gives access to the Payment API and the various endpoints associated with it.

use crate::constants::payments::RESOURCE_ENDPOINT;
use crate::errors::CbAdvError;
use crate::http_agent::{HttpAgent, SecureHttpAgent};
use crate::models::payment::{GetPaymentMethod, ListPaymentMethods, PaymentMethod};
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
        match self.agent.get(RESOURCE_ENDPOINT, &NoQuery).await {
            Ok(value) => match value.json::<ListPaymentMethods>().await {
                Ok(resp) => Ok(resp.payment_methods),
                Err(err) => Err(CbAdvError::BadParse(format!(
                    "payment methods object: {}",
                    err
                ))),
            },
            Err(error) => Err(error),
        }
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
        match self.agent.get(&resource, &NoQuery).await {
            Ok(value) => match value.json::<GetPaymentMethod>().await {
                Ok(resp) => Ok(resp.payment_method),
                Err(_) => Err(CbAdvError::BadParse("payment method object".to_string())),
            },
            Err(error) => Err(error),
        }
    }
}
