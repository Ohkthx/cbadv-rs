//! Contains all errors produced by the crate.

use std::error::Error;
use std::fmt;

/// Types of errors that can occur.
#[derive(Debug)]
pub enum CbError {
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
    /// JWT generation error.
    BadJwt(String),
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
    /// Authentication error.
    AuthenticationError(String),
    /// An invalid query.
    BadQuery(String),
    /// An invalid request.
    BadRequest(String),
}

impl fmt::Display for CbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CbError::Unknown(value) => write!(f, "unknown error occurred: {value}"),
            CbError::BadJwt(value) => write!(f, "could not create JWT: {value}"),
            CbError::BadSignature(value) => write!(f, "could not create signature: {value}"),
            CbError::BadSerialization(value) => {
                write!(f, "could not serialize the message body: {value}")
            }
            CbError::BadPrivateKey(value) => write!(f, "invalid private key: {value}"),
            CbError::BadParse(value) => write!(f, "could not parse: {value}"),
            CbError::NothingToDo(value) => write!(f, "nothing to do: {value}"),
            CbError::NotFound(value) => write!(f, "could not find: {value}"),
            CbError::BadStatus { code, body } => {
                write!(f, "HTTP error {}: {}", code.as_u16(), body)
            }
            CbError::BadConnection(value) => write!(f, "could not connect: {value}"),
            CbError::RequestError(value) => write!(f, "HTTP request error: {value}"),
            CbError::UrlParseError(value) => write!(f, "URL parse error: {value}"),
            CbError::JsonError(value) => write!(f, "JSON deserialization error: {value}"),
            CbError::AuthenticationError(value) => write!(f, "authentication error: {value}"),
            CbError::BadQuery(value) => write!(f, "invalid query: {value}"),
            CbError::BadRequest(value) => write!(f, "invalid request: {value}"),
        }
    }
}

impl Error for CbError {}
