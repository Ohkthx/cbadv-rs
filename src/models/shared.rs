//! # Coinbase Advanced Account API
//!
//! `shared` gives access to utilities that will be reused throughout the API and user.

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a Balance for either Available or Held funds.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
    /// Value for the currency available or held.
    #[serde_as(as = "DisplayFromStr")]
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
