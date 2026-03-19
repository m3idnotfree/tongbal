use serde::{Deserialize, Serialize};

use crate::{new_types::SessionKey, subscribed_event::SubscribedEvent};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum System {
    Connected(Connected),
    Subscribed(SubscribedEvent),
    Unsubscribed(SubscribedEvent),
    Revoked(SubscribedEvent),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Connected {
    pub session_key: SessionKey,
}
