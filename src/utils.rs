//! # Utilities and supporting functions.
//!
//! `utils` is a collection of helpful tools that may be required throughout the rest of the API.

use std::fmt;

/// Used to return objects from the API to the end-user.
pub type Result<T> = std::result::Result<T, CBAdvError>;

/// Types of errors that can occur.
#[derive(Debug)]
pub enum CBAdvError {
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

impl fmt::Display for CBAdvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CBAdvError::Unknown(value) => write!(f, "unknown error occured: {}", value),
            CBAdvError::BadParse(value) => write!(f, "could not parse: {}", value),
            CBAdvError::NothingToDo(value) => write!(f, "nothing to do: {}", value),
            CBAdvError::NotFound(value) => write!(f, "could not find: {}", value),
            CBAdvError::BadStatus(value) => write!(f, "non-zero status occurred: {}", value),
            CBAdvError::BadConnection(value) => write!(f, "could not connect: {}", value),
        }
    }
}
