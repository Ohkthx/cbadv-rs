//! # Time, easy to use timestamps, spans of time, etc
//!
//! `time` plays an important role in authentication for API requests and obtaining data between
//! spans of time such as in the Product API for obtaining Candles.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::errors::CbError;
use crate::traits::Query;
use crate::types::CbResult;
use crate::utils::QueryBuilder;

/// One minute of time in seconds.
const ONE_MINUTE: u32 = 60;
const FIVE_MINUTE: u32 = ONE_MINUTE * 5;
const FIFTEEN_MINUTE: u32 = ONE_MINUTE * 15;
const THIRTY_MINUTE: u32 = ONE_MINUTE * 30;

/// One hour of time in seconds.
const ONE_HOUR: u32 = ONE_MINUTE * 60;
const TWO_HOUR: u32 = ONE_HOUR * 2;
const SIX_HOUR: u32 = ONE_HOUR * 6;

/// One day of time in seconds.
const ONE_DAY: u32 = ONE_HOUR * 24;

/// Span of time in seconds.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Granularity {
    #[serde(rename = "UNKNOWN_GRANULARITY")]
    Unknown,
    OneMinute,
    FiveMinute,
    FifteenMinute,
    ThirtyMinute,
    OneHour,
    TwoHour,
    SixHour,
    OneDay,
}

impl fmt::Display for Granularity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for Granularity {
    fn as_ref(&self) -> &str {
        match self {
            Granularity::OneMinute => "ONE_MINUTE",
            Granularity::FiveMinute => "FIVE_MINUTE",
            Granularity::FifteenMinute => "FIFTEEN_MINUTE",
            Granularity::ThirtyMinute => "THIRTY_MINUTE",
            Granularity::OneHour => "ONE_HOUR",
            Granularity::TwoHour => "TWO_HOUR",
            Granularity::SixHour => "SIX_HOUR",
            Granularity::OneDay => "ONE_DAY",
            Granularity::Unknown => "UNKNOWN_GRANULARITY",
        }
    }
}

impl Granularity {
    /// Converts a Granularity into seconds.
    pub fn to_secs(granularity: &Granularity) -> u32 {
        match granularity {
            Granularity::OneMinute => ONE_MINUTE,
            Granularity::FiveMinute => ONE_MINUTE * 5,
            Granularity::FifteenMinute => ONE_MINUTE * 15,
            Granularity::ThirtyMinute => ONE_MINUTE * 30,

            Granularity::OneHour => ONE_HOUR,
            Granularity::TwoHour => ONE_HOUR * 2,
            Granularity::SixHour => ONE_HOUR * 6,

            Granularity::OneDay => ONE_DAY,

            Granularity::Unknown => 0,
        }
    }

    /// Converts from seconds to Granularity.
    pub fn from_secs(granularity: u32) -> Granularity {
        match granularity {
            ONE_MINUTE => Granularity::OneMinute,
            FIVE_MINUTE => Granularity::FiveMinute,
            FIFTEEN_MINUTE => Granularity::FifteenMinute,
            THIRTY_MINUTE => Granularity::ThirtyMinute,

            ONE_HOUR => Granularity::OneHour,
            TWO_HOUR => Granularity::TwoHour,
            SIX_HOUR => Granularity::SixHour,

            ONE_DAY => Granularity::OneDay,

            // UnknownGranularity is defined in the API.
            _ => Granularity::Unknown,
        }
    }
}

/// Span of time, where `start` and `end` are in seconds.
#[derive(Serialize)]
pub struct Span {
    pub start: u64,
    pub end: u64,
    pub granularity: u32,
}

impl Span {
    /// Creates a new instance of Span that holds a start and end time along with how long the
    /// blocks of time should be.
    ///
    /// * NOTE: `start` should be oldest of the two timestamps. `end` should be most recent.
    ///         dinosaurs -> start -> end -> now
    ///
    /// # Arguments
    ///
    /// * `start` - An unsigned int that holds the start point of the span.
    /// * `end` - An unsigned int that holds the end point of the span.
    /// * `granularity` - A Granularity that represents blocks of time in seconds.
    pub fn new(start: u64, end: u64, granularity: &Granularity) -> Self {
        let granularity_sec = Granularity::to_secs(granularity);

        // Clean the time, they have to be the correct offset.
        // end = end - (end % granularity_sec as u64);
        // start = start - (start % granularity_sec as u64);

        Self {
            start,
            end,
            granularity: granularity_sec,
        }
    }

    /// Total amount of intervals within the span.
    ///
    /// # Panics
    ///
    /// Panics if the number of intervals is greater than `usize`.
    pub fn count(&self) -> usize {
        // Clean the time, they have to be the correct offset.
        let end = self.end - (self.end % u64::from(self.granularity));
        let start = self.start - (self.start % u64::from(self.granularity));
        usize::try_from((end - start) / u64::from(self.granularity)).unwrap()
    }
}

impl Query for Span {
    fn check(&self) -> CbResult<()> {
        if self.start >= self.end {
            return Err(CbError::BadQuery("start must be before end".to_string()));
        } else if self.granularity == 0 {
            return Err(CbError::BadQuery(
                "granularity must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }

    fn to_query(&self) -> String {
        let granularity = Granularity::from_secs(self.granularity);
        QueryBuilder::new()
            .push("start", self.start)
            .push("end", self.end)
            .push("granularity", granularity)
            .build()
    }
}

/// Obtains the current timestamp in UNIX format.
///
/// # Panics
///
/// Panics if the system time is before the UNIX epoch.
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Offset a timestamp by a number of seconds into the future.
pub fn after(timestamp: u64, seconds: u64) -> u64 {
    timestamp + seconds
}

/// Offset a timestamp by a number of seconds into the past.
pub fn before(timestamp: u64, seconds: u64) -> u64 {
    timestamp - seconds
}
