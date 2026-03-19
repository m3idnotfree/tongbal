use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::new_types::ChannelId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub channel_id: ChannelId,
    pub sender_channel_id: ChannelId,
    pub profile: Profile,
    pub user_role_code: UserRoleCode,
    pub content: String,
    pub emojis: HashMap<String, String>,
    pub message_time: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub nickname: String,
    pub badges: Vec<serde_json::Value>,
    pub verified_mark: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRoleCode {
    Streamer,
    CommonUser,
    StreamingChannelManager,
    StreamingChatManager,
}
