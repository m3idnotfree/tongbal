use serde::{Deserialize, Serialize};
use url::Url;

use crate::{channel::Tier, types::ChannelId};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub channel_id: ChannelId,
    pub channel_name: String,
    pub channel_image_url: Option<Url>,
    pub follower_count: u64,
    pub verified_mark: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Moderator {
    pub manager_channel_id: ChannelId,
    pub manager_channel_name: String,
    pub user_role: UserRole,
    pub created_date: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    StreamingChannelOwner,
    StreamingChannelManager,
    StreamingChatManager,
    StreamingSettlementManager,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Follower {
    pub channel_id: ChannelId,
    pub channel_name: String,
    pub created_date: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscriber {
    pub channel_id: ChannelId,
    pub channel_name: String,
    pub month: u64,
    pub tier_no: Tier,
    pub created_date: String,
}

pub enum Sort {
    RECENT,
    LONGER,
}

impl Sort {
    pub fn as_str(&self) -> &str {
        match self {
            Self::RECENT => "RECENT",
            Self::LONGER => "LONGER",
        }
    }
}
