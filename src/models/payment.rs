//! # Coinbase Advanced Payment API
//!
//! `payment` gives access to the Payment API and the various endpoints associated with it.

use serde::{Deserialize, Serialize};

/// A type of payment method available to the user for use.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethod {
    /// Unique identifier for the payment method.
    pub id: String,
    /// The payment method type.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Name for the payment method.
    pub name: String,
    /// Currency symbol for the payment method.
    pub currency: String,
    /// The verified status of the payment method.
    pub verified: bool,
    /// Whether or not this payment method can perform buys.
    pub allow_buy: bool,
    /// Whether or not this payment method can perform sells.
    pub allow_sell: bool,
    /// Whether or not this payment method can perform deposits.
    pub allow_deposit: bool,
    /// Whether or not this payment method can perform withdrawals.
    pub allow_withdraw: bool,
    /// Time at which this payment method was created.
    pub created_at: String,
    /// Time at which this payment method was updated.
    pub updated_at: Option<String>,
}

/// Response from the API that wraps a list of payment methods.
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct PaymentMethodsWrapper {
    /// List of payment methods available to the user.
    pub(crate) payment_methods: Vec<PaymentMethod>,
}

impl From<PaymentMethodsWrapper> for Vec<PaymentMethod> {
    fn from(wrapper: PaymentMethodsWrapper) -> Self {
        wrapper.payment_methods
    }
}

/// Response from the API that wraps a single payment method.
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct PaymentMethodWrapper {
    /// A payment method requested by the user.
    pub(crate) payment_method: PaymentMethod,
}

impl From<PaymentMethodWrapper> for PaymentMethod {
    fn from(wrapper: PaymentMethodWrapper) -> Self {
        wrapper.payment_method
    }
}
