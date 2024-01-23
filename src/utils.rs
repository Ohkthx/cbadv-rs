//! # Utilities and supporting functions.
//!
//! `utils` is a collection of helpful tools that may be required throughout the rest of the API.

use num_traits::Zero;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer};
use std::fmt::{self, Write};
use std::result;
use std::str::{self, FromStr};

/// Used to return objects from the API to the end-user.
pub type CbResult<T> = result::Result<T, CbAdvError>;

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
    /// Could not build the signature.
    BadSignature,
    /// Could not serialize the body of a message.
    BadSerialization,
    /// General unknown error.
    Unknown(String),
}

impl fmt::Display for CbAdvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CbAdvError::Unknown(value) => write!(f, "unknown error occured: {}", value),
            CbAdvError::BadSignature => write!(f, "could not sign the message to be sent"),
            CbAdvError::BadSerialization => write!(f, "could not serialize the message body"),
            CbAdvError::BadParse(value) => write!(f, "could not parse: {}", value),
            CbAdvError::NothingToDo(value) => write!(f, "nothing to do: {}", value),
            CbAdvError::NotFound(value) => write!(f, "could not find: {}", value),
            CbAdvError::BadStatus(value) => write!(f, "non-zero status occurred: {}", value),
            CbAdvError::BadConnection(value) => write!(f, "could not connect: {}", value),
        }
    }
}

/// Enum representing different types of inputs.
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
pub struct QueryBuilder {
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
    pub fn new() -> Self {
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
    pub fn push(&mut self, key: &str, value: &str) -> &mut Self {
        if !self.query.is_empty() {
            self.query.push('&');
        } else {
            self.query.push('?');
        }

        write!(self.query, "{}={}", key, value).unwrap();
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
    pub fn push_optional<'a, T: AsRef<str>>(
        &'a mut self,
        key: &str,
        value: &Option<T>,
    ) -> &'a mut Self {
        if let Some(v) = value {
            self.push(key, v.as_ref());
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
    pub fn push_u32_optional<'a>(&'a mut self, key: &str, value: Option<u32>) -> &'a mut Self {
        if let Some(v) = value {
            self.push(key, &v.to_string());
        }
        self
    }

    /// Adds multiple query parameters from an array of tuples.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the query parameter.
    /// * `values` - An optional array of values to assign to the key.
    pub fn with_optional_vec<'a, T: AsRef<str>>(
        &'a mut self,
        key: &str,
        values: &Option<Vec<T>>,
    ) -> &'a mut Self {
        if let Some(values) = values {
            for value in values {
                self.push(key.as_ref(), value.as_ref());
            }
        }
        self
    }

    /// Adds multiple query parameters from an array of tuples.
    ///
    /// # Arguments
    ///
    /// * `params` - Array of tuples where each tuple is a (key, value) pair.
    pub fn with_params(&mut self, params: &[(impl AsRef<str>, impl AsRef<str>)]) -> &mut Self {
        for (key, value) in params {
            self.push(key.as_ref(), value.as_ref());
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
    pub fn build(self) -> String {
        self.query
    }
}
