//! Contains all errors produced by the crate.

use std::fmt;

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
