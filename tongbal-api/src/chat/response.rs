use serde::{Deserialize, Serialize};

use crate::types::MessageId;

use super::{ChatCondition, ChatGroup};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    pub message_id: MessageId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSettingResponse {
    pub chat_available_condition: ChatCondition,
    pub chat_available_group: ChatGroup,
    pub min_follower_minute: u64,
    pub allow_subscriber_in_follower_mode: bool,
}
