use serde::{Deserialize, Serialize};

use crate::types::{ChannelId, SessionKey};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub session_key: SessionKey,
    pub connected_date: String,
    pub disconnected_date: String,
    pub subscribed_events: Vec<SubscribedEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribedEvent {
    pub event_type: EventType,
    pub channel_id: ChannelId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventType {
    Chat,
    Donation,
    Subscription,
}
