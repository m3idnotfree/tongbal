use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::ChannelId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Donation {
    pub donation_type: Kind,
    pub channel_id: ChannelId,
    pub donator_channel_id: ChannelId,
    pub donator_nickname: String,
    pub pay_amount: String,
    pub donation_text: String,
    pub emojis: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Kind {
    Chat,
    Video,
}
