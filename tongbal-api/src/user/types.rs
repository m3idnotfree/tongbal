use serde::{Deserialize, Serialize};

use crate::types::ChannelId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub channel_id: ChannelId,
    pub channel_name: String,
}
