use serde::{Deserialize, Serialize};

use super::{Channel, Events};

/// Message from the WebSocket containing event updates.
#[derive(Serialize, Deserialize, Debug)]
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
    pub events: Events,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heartbeat_works() {
        let data = r#"
            {
                "channel":"heartbeats",
                "client_id":"",
                "timestamp":"2025-01-14T22:11:18.791273556Z",
                "sequence_num":17,
                "events":
                [
                    {
                        "current_time":"2025-01-14 22:11:18.787177997 +0000 UTC m=+25541.571430466",
                        "heartbeat_counter":25539
                    }
                ]
            }
        "#;

        let res: Result<Message, serde_json::Error> = serde_json::from_str(data);
        assert!(res.is_ok());
    }
}
