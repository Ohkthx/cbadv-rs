//! # Coinbase Advanced Util API
//!
//! `util` gives access to the Util API and the various endpoints associated with it.
//! Some of the features include getting the API current time in ISO format.

use serde::{Deserialize, Serialize};

use crate::utils::deserialize_numeric;

/// Get the current time from the Coinbase Advanced API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnixTime {
    /// An ISO-8601 representation of the timestamp.
    pub iso: String,
    /// A second-precision representation of the timestamp.
    #[serde(rename(deserialize = "epochSeconds"))]
    #[serde(deserialize_with = "deserialize_numeric")]
    pub epoch_seconds: u64,
    /// A millisecond-precision representation of the timestamp.
    #[serde(rename(deserialize = "epochMillis"))]
    #[serde(deserialize_with = "deserialize_numeric")]
    pub epoch_millis: u64,
}
