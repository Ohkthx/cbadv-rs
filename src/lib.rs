//! # cbadv, Coinbase Advanced API
//!
//! `cbadv` provides the tools required to access the new Coinbase Advanced API. Coinbased Advanced
//! API is the successor of the Coinbase Pro API which is now depreciated.
//!
//! This crate is still a work-in-progress with additional optimizations and quality checking to
//! come in future updates. Use it as your own discretion and be mindful that bugs most likely
//! exist. I hold no responsibility for any issues that occur by using this software and welcome
//! contributions.
#![warn(clippy::pedantic)]
#![allow(
    clippy::return_self_not_must_use,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools
)]
#![cfg_attr(all(test, feature = "full"), deny(unreachable_pub))]
#![cfg_attr(all(test, feature = "full"), deny(warnings))]

#[cfg(feature = "config")]
pub mod config;

#[macro_use]
pub(crate) mod macros;

/// Re-export tokio for use in the library.
pub use tokio::{self, main as tokio_main};

pub(crate) mod http_agent;
pub(crate) mod jwt;
mod token_bucket;

pub(crate) mod constants;
pub mod errors;
pub mod time;
pub mod traits;
pub mod types;
pub(crate) mod utils;

pub mod apis;
pub mod models;

mod rest;
mod websocket;
pub use rest::{RestClient, RestClientBuilder};
pub use websocket::{WebSocketClient, WebSocketClientBuilder};
