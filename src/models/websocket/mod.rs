//! # Coinbase Advanced Websocket Client
//!
//! `websocket` gives access to the websocket stream to receive updates in a streamlined fashion.
//! Many parts of the REST API suggest using websockets instead due to ratelimits and being quicker
//! for large amount of constantly changing data.

mod enums;
mod events;
mod message;
mod responses;
mod types;

pub use enums::*;
pub use events::*;
pub use message::*;
pub use responses::*;
pub use types::*;
