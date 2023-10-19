//! # Utilities and supporting functions.
//!
//! `utils` is a collection of helpful tools that may be required throughout the rest of the API.

use serde::{Deserialize, Deserializer};
use std::{fmt, result, str};

/// Used to return objects from the API to the end-user.
pub type Result<T> = result::Result<T, CbAdvError>;

/// Types of errors that can occur.
#[derive(Debug)]
pub enum CbAdvError {
    /// Unable to parse JSON successfully.
    BadParse(String),
    /// Non-200 status code received.
    BadStatus(String),
    /// Could not connect to the service.
    BadConnection(String),
    /// Nothing to do.
    NothingToDo(String),
    /// Unable to locate resource.
    NotFound(String),
    /// General unknown error.
    Unknown(String),
}

impl fmt::Display for CbAdvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CbAdvError::Unknown(value) => write!(f, "unknown error occured: {}", value),
            CbAdvError::BadParse(value) => write!(f, "could not parse: {}", value),
            CbAdvError::NothingToDo(value) => write!(f, "nothing to do: {}", value),
            CbAdvError::NotFound(value) => write!(f, "could not find: {}", value),
            CbAdvError::BadStatus(value) => write!(f, "non-zero status occurred: {}", value),
            CbAdvError::BadConnection(value) => write!(f, "could not connect: {}", value),
        }
    }
}

/// Used to check if the value is a number or string.
#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrNumeric {
    String(String),
    Numeric(f64),
}

/// Deserializes from a string or numeric type, using the default value if there is an error or is null.
pub(crate) fn from_str<'de, S, D>(deserializer: D) -> result::Result<S, D::Error>
where
    S: str::FromStr + Default,
    S::Err: fmt::Display,
    D: Deserializer<'de>,
{
    // Catches strings, null values, floats / doubles, and integers.
    // Null values default to 0.
    let s: String = match Deserialize::deserialize(deserializer) {
        Ok(value) => match value {
            StringOrNumeric::String(value) => value,
            StringOrNumeric::Numeric(value) => value.to_string(),
        },
        Err(_) => String::default(),
    };

    Ok(S::from_str(&s).unwrap_or_default())
}

/// Prints out a debug message, wraps `println!` macro.
#[macro_export]
macro_rules! debugln {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        println!(concat!("[DEBUG] ", $fmt), $($($arg)*)?);
    };
}
