use std::fmt;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde_json::Value;

use super::{
    CandlesEvent, Channel, Event, FuturesSummaryBalanceEvent, HeartbeatsEvent, Level2Event,
    MarketTradesEvent, StatusEvent, SubscribeEvent, TickerEvent, UserEvent,
};

/// Message from the WebSocket containing event updates.
#[derive(Debug)]
pub struct Message {
    /// The channel the message is from.
    pub channel: Channel,
    /// The client ID for the message.
    pub client_id: String,
    /// The timestamp for the message.
    pub timestamp: String,
    /// The sequence number for the message
    pub sequence_num: u64,
    /// The events in the message.
    pub events: Vec<Event>,
}

/// Custom deserialization for Message.
impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Message, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MessageVisitor)
    }
}

/// Visitor struct for custom deserialization for Message.
struct MessageVisitor;

impl<'de> Visitor<'de> for MessageVisitor {
    type Value = Message;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a WebSocket message")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Message, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut channel: Option<Channel> = None;
        let mut client_id: Option<String> = None;
        let mut timestamp: Option<String> = None;
        let mut sequence_num: Option<u64> = None;
        let mut events_value: Option<Value> = None;

        // Extract common fields and store the raw events for later deserialization.
        while let Some(key) = map.next_key::<&str>()? {
            match key {
                "channel" => {
                    if channel.is_some() {
                        return Err(de::Error::duplicate_field("channel"));
                    }
                    channel = Some(map.next_value()?);
                }
                "client_id" => {
                    if client_id.is_some() {
                        return Err(de::Error::duplicate_field("client_id"));
                    }
                    client_id = Some(map.next_value()?);
                }
                "timestamp" => {
                    if timestamp.is_some() {
                        return Err(de::Error::duplicate_field("timestamp"));
                    }
                    timestamp = Some(map.next_value()?);
                }
                "sequence_num" => {
                    if sequence_num.is_some() {
                        return Err(de::Error::duplicate_field("sequence_num"));
                    }
                    sequence_num = Some(map.next_value()?);
                }
                "events" => {
                    if events_value.is_some() {
                        return Err(de::Error::duplicate_field("events"));
                    }
                    // Temporarily store events as serde_json::Value
                    events_value = Some(map.next_value()?);
                }
                _ => {
                    // Skip unknown fields or handle as needed.
                    let _ = map.next_value::<de::IgnoredAny>()?;
                }
            }
        }

        let channel = channel.ok_or_else(|| de::Error::missing_field("channel"))?;
        let client_id = client_id.ok_or_else(|| de::Error::missing_field("client_id"))?;
        let timestamp = timestamp.ok_or_else(|| de::Error::missing_field("timestamp"))?;
        let sequence_num = sequence_num.ok_or_else(|| de::Error::missing_field("sequence_num"))?;
        let events_value = events_value.ok_or_else(|| de::Error::missing_field("events"))?;

        // Deserialize events based on the channel.
        let events = deserialize_events(&channel, events_value).map_err(de::Error::custom)?;

        Ok(Message {
            channel,
            client_id,
            timestamp,
            sequence_num,
            events,
        })
    }
}

/// Helper function to deserialize events based on the channel.
fn deserialize_events(
    channel: &Channel,
    events_value: Value,
) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    match channel {
        Channel::Status => {
            let events: Vec<StatusEvent> = serde_json::from_value(events_value)?;
            Ok(events.into_iter().map(Event::Status).collect())
        }
        Channel::Candles => {
            let events: Vec<CandlesEvent> = serde_json::from_value(events_value)?;
            Ok(events.into_iter().map(Event::Candles).collect())
        }
        Channel::Ticker => {
            let events: Vec<TickerEvent> = serde_json::from_value(events_value)?;
            Ok(events.into_iter().map(Event::Ticker).collect())
        }
        Channel::TickerBatch => {
            let events: Vec<TickerEvent> = serde_json::from_value(events_value)?;
            Ok(events.into_iter().map(Event::TickerBatch).collect())
        }
        Channel::Level2 => {
            let events: Vec<Level2Event> = serde_json::from_value(events_value)?;
            Ok(events.into_iter().map(Event::Level2).collect())
        }
        Channel::User => {
            let events: Vec<UserEvent> = serde_json::from_value(events_value)?;
            Ok(events.into_iter().map(Event::User).collect())
        }
        Channel::MarketTrades => {
            let events: Vec<MarketTradesEvent> = serde_json::from_value(events_value)?;
            Ok(events.into_iter().map(Event::MarketTrades).collect())
        }
        Channel::Heartbeats => {
            let events: Vec<HeartbeatsEvent> = serde_json::from_value(events_value)?;
            Ok(events.into_iter().map(Event::Heartbeats).collect())
        }
        Channel::Subscriptions => {
            let events: Vec<SubscribeEvent> = serde_json::from_value(events_value)?;
            Ok(events.into_iter().map(Event::Subscribe).collect())
        }
        Channel::FuturesBalanceSummary => {
            let events: Vec<FuturesSummaryBalanceEvent> = serde_json::from_value(events_value)?;
            Ok(events
                .into_iter()
                .map(Event::FuturesBalanceSummary)
                .collect())
        }
    }
}
