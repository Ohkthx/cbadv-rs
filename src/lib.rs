//! # cbadv, Coinbase Advanced API
//!
//! `cbadv` provides the tools required to access the new Coinbase Advanced API. Coinbased Advanced
//! API is the successor of the Coinbase Pro API which is now depreciated.
//!
//! This crate is still a work-in-progress with additional optimizations and quality checking to
//! come in future updates. Use it as your own discretion and be mindful that bugs most likely
//! exist. I hold no responsibility for any issues that occur by using this software and welcome
//! contributions.

pub mod account;
pub mod config;
pub mod fee;
pub mod order;
pub mod product;
pub mod rest;
mod signer;
pub mod time;
pub mod utils;
pub mod websocket;
