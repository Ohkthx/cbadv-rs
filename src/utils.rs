//! # Utilities and supporting functions.
//!
//! `utils` is a collection of helpful tools that may be required throughout the rest of the API.

use std::fmt::{self, Write};
use std::str::{self, FromStr};

use num_traits::Zero;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer};

/// Enum representing different types of inputs.
#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(untagged)]
enum StringNumericOrNull {
    Numeric(serde_json::Value),
    String(String),
    Null,
}

/// Custom deserializer function for any numeric type.
pub(crate) fn deserialize_numeric<'de, N, D>(deserializer: D) -> Result<N, D::Error>
where
    N: FromStr + Zero,
    N::Err: fmt::Display,
    D: Deserializer<'de>,
{
    struct StringNumericOrNullVisitor<N>(std::marker::PhantomData<N>);

    impl<'de, N> Visitor<'de> for StringNumericOrNullVisitor<N>
    where
        N: FromStr + Zero,
        N::Err: fmt::Display,
    {
        type Value = N;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string, numeric type, null, or empty string")
        }

        fn visit_i64<E>(self, value: i64) -> Result<N, E>
        where
            E: de::Error,
        {
            N::from_str(&value.to_string()).map_err(de::Error::custom)
        }

        fn visit_u64<E>(self, value: u64) -> Result<N, E>
        where
            E: de::Error,
        {
            N::from_str(&value.to_string()).map_err(de::Error::custom)
        }

        fn visit_f64<E>(self, value: f64) -> Result<N, E>
        where
            E: de::Error,
        {
            N::from_str(&value.to_string()).map_err(de::Error::custom)
        }

        fn visit_str<E>(self, value: &str) -> Result<N, E>
        where
            E: de::Error,
        {
            if value.is_empty() {
                Ok(N::zero())
            } else {
                N::from_str(value).map_err(de::Error::custom)
            }
        }

        fn visit_unit<E>(self) -> Result<N, E>
        where
            E: de::Error,
        {
            Ok(N::zero())
        }
    }

    deserializer.deserialize_any(StringNumericOrNullVisitor(std::marker::PhantomData))
}

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
    ///
    /// # Examples
    ///
    /// ```
    /// let query_builder = QueryBuilder::new();
    /// ```
    pub(crate) fn new() -> Self {
        Self {
            query: String::new(),
        }
    }

    /// Adds a key-value pair to the query string.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the query parameter.
    /// * `value` - The value of the query parameter.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `QueryBuilder` for chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.push("key", "value");
    /// ```
    pub(crate) fn push<T: ToString>(mut self, key: &str, value: T) -> Self {
        if !self.query.is_empty() {
            self.query.push('&');
        } else {
            self.query.push('?');
        }

        write!(self.query, "{}={}", key, value.to_string()).unwrap();
        self
    }

    /// Adds a key-value pair to the query string, if the value is present (Some).
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the query parameter.
    /// * `value` - An optional value of the query parameter.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `QueryBuilder` for chaining.
    pub(crate) fn push_optional<T: AsRef<str>>(mut self, key: &str, value: &Option<T>) -> Self {
        if let Some(v) = value {
            self = self.push(key, v.as_ref());
        }
        self
    }
    /// Adds a key-value pair to the query string, if the value is present (Some).
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the query parameter.
    /// * `value` - An optional value of the query parameter.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `QueryBuilder` for chaining.
    pub(crate) fn push_u32_optional(mut self, key: &str, value: Option<u32>) -> Self {
        if let Some(v) = value {
            self = self.push(key, v.to_string());
        }
        self
    }

    /// Adds multiple query parameters from an array of tuples.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the query parameter.
    /// * `values` - An optional array of values to assign to the key.
    pub(crate) fn with_optional_vec<T: AsRef<str>>(
        mut self,
        key: &str,
        values: &Option<Vec<T>>,
    ) -> Self {
        if let Some(values) = values {
            for value in values {
                self = self.push(key.as_ref(), value.as_ref());
            }
        }
        self
    }

    /// Builds the final query string.
    ///
    /// # Returns
    ///
    /// A `String` containing the constructed query.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut query_builder = QueryBuilder::new();
    /// query_builder.push("key1", "value1").push("key2", "value2");
    /// let query = query_builder.build();
    /// ```
    pub(crate) fn build(self) -> String {
        self.query
    }
}
