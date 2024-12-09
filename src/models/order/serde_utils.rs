//! # Coinbase Advanced Order API
//!
//! `order/serde_utils` is the module containing the serde utility functions for the `OrderType` enum.

use std::fmt;

use serde::de::{self, Deserialize as DeDeserialize, Deserializer, Visitor};

use super::OrderType;

impl<'de> DeDeserialize<'de> for OrderType {
    fn deserialize<D>(deserializer: D) -> Result<OrderType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OrderTypeVisitor)
    }
}

struct OrderTypeVisitor;

impl Visitor<'_> for OrderTypeVisitor {
    type Value = OrderType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing an OrderType")
    }

    fn visit_str<E>(self, value: &str) -> Result<OrderType, E>
    where
        E: de::Error,
    {
        match value.to_uppercase().as_str() {
            "UNKNOWN_ORDER_TYPE" => Ok(OrderType::Unknown),
            "MARKET" => Ok(OrderType::Market),
            "LIMIT" => Ok(OrderType::Limit),
            "STOP" => Ok(OrderType::Stop),
            "STOP_LIMIT" => Ok(OrderType::StopLimit),
            "BRACKET" => Ok(OrderType::Bracket),
            _ => Err(de::Error::unknown_variant(
                value,
                &[
                    "UnknownOrderType",
                    "Market",
                    "Limit",
                    "Stop",
                    "StopLimit",
                    "Bracket",
                ],
            )),
        }
    }
}
