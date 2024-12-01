//! # Utilities and supporting functions.
//!
//! `utils` is a collection of helpful tools that may be required throughout the rest of the API.

use std::fmt::{Display, Write};

/// Prints out a debug message, wraps `println!` macro.
#[macro_export]
macro_rules! debugln {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        println!(concat!("[DEBUG] ", $fmt), $($($arg)*)?);
    };
}

/// Builds the URL Query to be sent to the API.
pub(crate) struct QueryBuilder {
    query: String,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryBuilder {
    /// Constructs a new `QueryBuilder`.
    pub(crate) fn new() -> Self {
        Self {
            query: String::new(),
        }
    }

    /// Adds a key-value pair to the query string.
    pub(crate) fn push<T: Display>(mut self, key: &str, value: T) -> Self {
        if !self.query.is_empty() {
            self.query.push('&');
        }

        write!(self.query, "{}={}", key, value).unwrap();
        self
    }

    /// Adds a key-value pair to the query string if the value is present.
    pub(crate) fn push_optional<T: Display>(self, key: &str, value: &Option<T>) -> Self {
        if let Some(v) = value {
            self.push(key, v)
        } else {
            self
        }
    }

    /// Adds multiple key-value pairs from an optional vector.
    pub(crate) fn push_optional_vec<T: Display>(
        mut self,
        key: &str,
        values: &Option<Vec<T>>,
    ) -> Self {
        if let Some(values) = values {
            for value in values {
                self = self.push(key, value);
            }
        }
        self
    }

    /// Builds and returns the final query string.
    pub(crate) fn build(self) -> String {
        self.query
    }
}
