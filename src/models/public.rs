//! # Coinbase Advanced Public API
//!
//! `public` gives access to the Public API and the various endpoints associated with it.
//! Some of the features include getting the API current time in ISO format.

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Get the current time from the Coinbase Advanced API.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerTime {
    /// An ISO-8601 representation of the timestamp.
    pub iso: String,
    /// A second-precision representation of the timestamp.
    #[serde(rename(deserialize = "epochSeconds"))]
    #[serde_as(as = "DisplayFromStr")]
    pub epoch_seconds: u64,
    /// A millisecond-precision representation of the timestamp.
    #[serde(rename(deserialize = "epochMillis"))]
    #[serde_as(as = "DisplayFromStr")]
    pub epoch_millis: u64,
}
