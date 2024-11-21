//! Contains all errors produced by the crate.

use std::error::Error;
use std::fmt;

/// Types of errors that can occur.
#[derive(Debug)]
pub enum CbAdvError {
    /// Unable to parse JSON or Builders successfully.
    BadParse(String),
    /// Non-200 status code received.
    BadStatus {
        code: reqwest::StatusCode,
        body: String,
    },
    /// Could not connect to the service.
    BadConnection(String),
    /// Nothing to do.
    NothingToDo(String),
    /// Unable to locate resource.
    NotFound(String),
    /// Could not build the signature.
    BadSignature(String),
    /// Could not identify the API Secret key type.
    BadPrivateKey(String),
    /// Could not serialize the body of a message.
    BadSerialization(String),
    /// General unknown error.
    Unknown(String),
    /// HTTP request error.
    RequestError(String),
    /// URL parse error.
    UrlParseError(String),
    /// JSON deserialization error.
    JsonError(String),
}

impl fmt::Display for CbAdvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CbAdvError::Unknown(value) => write!(f, "unknown error occurred: {}", value),
            CbAdvError::BadSignature(value) => write!(f, "could not create signature: {}", value),
            CbAdvError::BadSerialization(value) => {
                write!(f, "could not serialize the message body: {}", value)
            }
            CbAdvError::BadPrivateKey(value) => write!(f, "invalid private key: {}", value),
            CbAdvError::BadParse(value) => write!(f, "could not parse: {}", value),
            CbAdvError::NothingToDo(value) => write!(f, "nothing to do: {}", value),
            CbAdvError::NotFound(value) => write!(f, "could not find: {}", value),
            CbAdvError::BadStatus { code, body } => {
                write!(f, "HTTP error {}: {}", code.as_u16(), body)
            }
            CbAdvError::BadConnection(value) => write!(f, "could not connect: {}", value),
            CbAdvError::RequestError(value) => write!(f, "HTTP request error: {}", value),
            CbAdvError::UrlParseError(value) => write!(f, "URL parse error: {}", value),
            CbAdvError::JsonError(value) => write!(f, "JSON deserialization error: {}", value),
        }
    }
}

impl Error for CbAdvError {}
