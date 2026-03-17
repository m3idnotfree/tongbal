use serde::{Deserialize, Serialize};

use crate::types::MessageId;

#[derive(Serialize)]
pub struct MessageBody<'a> {
    pub message: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoticeBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<&'a MessageId>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChatCondition {
    None,
    RealName,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChatGroup {
    All,
    Follower,
    Manager,
    Subscriber,
}

#[derive(Debug)]
pub enum ChatSlowModeSec {
    Off,
    Three,
    Five,
    Ten,
    Thirty,
    Sixty,
    TwoMinutes,
    FiveMinutes,
}

impl Serialize for ChatSlowModeSec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::Off => 0,
            Self::Three => 3,
            Self::Five => 5,
            Self::Ten => 10,
            Self::Thirty => 30,
            Self::Sixty => 60,
            Self::TwoMinutes => 120,
            Self::FiveMinutes => 300,
        };
        serializer.serialize_u64(value)
    }
}

impl<'de> Deserialize<'de> for ChatSlowModeSec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Unexpected, Visitor};
        struct ChatSlowModeSecVisitor;

        impl<'de> Visitor<'de> for ChatSlowModeSecVisitor {
            type Value = ChatSlowModeSec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("0, 3, 5, 10, 30, 60, 120, or 300")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    0 => Ok(ChatSlowModeSec::Off),
                    3 => Ok(ChatSlowModeSec::Three),
                    5 => Ok(ChatSlowModeSec::Five),
                    10 => Ok(ChatSlowModeSec::Ten),
                    30 => Ok(ChatSlowModeSec::Thirty),
                    60 => Ok(ChatSlowModeSec::Sixty),
                    120 => Ok(ChatSlowModeSec::TwoMinutes),
                    300 => Ok(ChatSlowModeSec::FiveMinutes),
                    _ => Err(E::invalid_value(Unexpected::Unsigned(v), &self)),
                }
            }
        }

        deserializer.deserialize_u64(ChatSlowModeSecVisitor)
    }
}

#[derive(Debug)]
pub enum MinFollowerMinute {
    Zero,
    Five,
    Ten,
    Thirty,
    Sixty,
    OneDay,
    OneWeek,
    ThirtyDays,
    SixtyDays,
    NinetyDays,
    OneTwentyDays,
    OneFiftyDays,
    OneEightyDays,
}

impl Serialize for MinFollowerMinute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::Zero => 0,
            Self::Five => 5,
            Self::Ten => 10,
            Self::Thirty => 30,
            Self::Sixty => 60,
            Self::OneDay => 1440,
            Self::OneWeek => 10080,
            Self::ThirtyDays => 43200,
            Self::SixtyDays => 86400,
            Self::NinetyDays => 129600,
            Self::OneTwentyDays => 172800,
            Self::OneFiftyDays => 216000,
            Self::OneEightyDays => 259200,
        };
        serializer.serialize_u64(value)
    }
}

impl<'de> Deserialize<'de> for MinFollowerMinute {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Unexpected, Visitor};

        struct MinFollowerMinuteVisitor;
        impl<'de> Visitor<'de> for MinFollowerMinuteVisitor {
            type Value = MinFollowerMinute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("0, 5, 10, 30, 60, 1440, 10080, 43200, 86400, 129600, 172800, 216000, or 259200")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    0 => Ok(MinFollowerMinute::Zero),
                    5 => Ok(MinFollowerMinute::Five),
                    10 => Ok(MinFollowerMinute::Ten),
                    30 => Ok(MinFollowerMinute::Thirty),
                    60 => Ok(MinFollowerMinute::Sixty),
                    1440 => Ok(MinFollowerMinute::OneDay),
                    10080 => Ok(MinFollowerMinute::OneWeek),
                    43200 => Ok(MinFollowerMinute::ThirtyDays),
                    86400 => Ok(MinFollowerMinute::SixtyDays),
                    129600 => Ok(MinFollowerMinute::NinetyDays),
                    172800 => Ok(MinFollowerMinute::OneTwentyDays),
                    216000 => Ok(MinFollowerMinute::OneFiftyDays),
                    259200 => Ok(MinFollowerMinute::OneEightyDays),
                    _ => Err(E::invalid_value(Unexpected::Unsigned(v), &self)),
                }
            }
        }

        deserializer.deserialize_u64(MinFollowerMinuteVisitor)
    }
}
