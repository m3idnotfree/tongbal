use serde::{Deserialize, Serialize};

use crate::new_types::ChannelId;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribedEvent {
    pub event_type: EventType,
    pub channel_id: ChannelId,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventType {
    Chat,
    Donation,
    Subscription,
}
