//! # API Endpoints that are interact with via `RestClient`.
//!
//! This module contains all the API Endpoints that are used to interact with Coinbase Advanced.

mod account;
mod convert;
mod data;
mod fee;
mod order;
mod payment;
mod portfolio;
mod product;
mod public;

pub use account::AccountApi;
pub use convert::ConvertApi;
pub use data::DataApi;
pub use fee::FeeApi;
pub use order::OrderApi;
pub use payment::PaymentApi;
pub use portfolio::PortfolioApi;
pub use product::ProductApi;
pub use public::PublicApi;
