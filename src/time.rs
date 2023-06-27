//! # Time, easy to use timestamps, spans of time, etc
//!
//! `time` plays an important role in authentication for API requests and obtaining data between
//! spans of time such as in the Product API for obtaining Candles.

use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

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
#[allow(dead_code)]
pub enum Granularity {
    UnknownGranularity,
    OneMinute,
    FiveMinute,
    FifteenMinute,
    ThirtyMinute,
    OneHour,
    TwoHour,
    SixHour,
    OneDay,
}

impl Granularity {
    /// Converts a Granularity into seconds.
    pub fn to_seconds(granularity: Granularity) -> u32 {
        match granularity {
            Granularity::OneMinute => ONE_MINUTE,
            Granularity::FiveMinute => ONE_MINUTE * 5,
            Granularity::FifteenMinute => ONE_MINUTE * 15,
            Granularity::ThirtyMinute => ONE_MINUTE * 30,

            Granularity::OneHour => ONE_HOUR,
            Granularity::TwoHour => ONE_HOUR * 2,
            Granularity::SixHour => ONE_HOUR * 6,

            Granularity::OneDay => ONE_DAY,

            Granularity::UnknownGranularity => 0,
        }
    }

    /// Converts from seconds to Granularity.
    pub fn from_seconds(granularity: u32) -> Granularity {
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
            _ => Granularity::UnknownGranularity,
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
    pub fn new(start: u64, end: u64, granularity: Granularity) -> Self {
        let granularity_sec = Granularity::to_seconds(granularity);

        // Clean the time, they have to be the correct offset.
        // end = end - (end % granularity_sec as u64);
        // start = start - (start % granularity_sec as u64);

        Self {
            start,
            end,
            granularity: granularity_sec,
        }
    }

    /// Gets the time span in a parameter format.
    pub fn to_params(&self) -> String {
        // First convert Granularity to the string format.
        let granularity = Granularity::from_seconds(self.granularity);
        let granularity_str: &str = match granularity {
            Granularity::OneMinute => "ONE_MINUTE",
            Granularity::FiveMinute => "FIVE_MINUTE",
            Granularity::FifteenMinute => "FIFTEEN_MINUTE",
            Granularity::ThirtyMinute => "THIRTY_MINUTE",

            Granularity::OneHour => "ONE_HOUR",
            Granularity::TwoHour => "TWO_HOUR",
            Granularity::SixHour => "SIX_HOUR",

            Granularity::OneDay => "ONE_DAY",

            Granularity::UnknownGranularity => "UNKNOWN_GRANULARITY",
        };

        format!(
            "start={}&end={}&granularity={}",
            self.start, self.end, granularity_str
        )
    }
}

/// Obtains the current timestamp in UNIX format.
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
