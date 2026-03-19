use serde::{Deserialize, Serialize};

use crate::{session::SubscribedEvent, types::SessionKey};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub session_key: SessionKey,
    pub connected_date: String,
    pub disconnected_date: String,
    pub subscribed_events: Vec<SubscribedEvent>,
}
