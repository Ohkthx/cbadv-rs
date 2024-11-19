//! # Coinbase Advanced Account API
//!
//! `shared` gives access to utilities that will be reused throughout the API and user.

use serde::{Deserialize, Serialize};

use crate::utils::deserialize_numeric;

/// Represents a Balance for either Available or Held funds.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
    /// Value for the currency available or held.
    #[serde(deserialize_with = "deserialize_numeric")]
    pub value: f64,
    /// Denomination of the currency.
    pub currency: String,
}

impl Balance {
    /// Creates a new Balance object that represents the value and currency.
    pub fn new(value: f64, currency: String) -> Self {
        Self { value, currency }
    }
}
