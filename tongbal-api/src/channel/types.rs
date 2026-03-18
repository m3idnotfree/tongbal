use serde::{Deserialize, Serialize};
use url::Url;

use crate::types::ChannelId;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tier {
    One,
    Two,
}

impl Serialize for Tier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::One => 1,
            Self::Two => 2,
        };

        serializer.serialize_u8(value)
    }
}

impl<'de> Deserialize<'de> for Tier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Unexpected, Visitor};
        struct TierVisitor;

        impl<'de> Visitor<'de> for TierVisitor {
            type Value = Tier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("1 or 2")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    1 => Ok(Tier::One),
                    2 => Ok(Tier::Two),
                    _ => Err(E::invalid_value(Unexpected::Unsigned(v), &self)),
                }
            }
        }

        deserializer.deserialize_u64(TierVisitor)
    }
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
