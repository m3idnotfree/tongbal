mod new_types;

pub use new_types::{ChannelId, MessageId, SessionKey};

use serde::{Deserialize, Serialize};

pub(crate) mod constants {
    pub const AUTH: &str = "auth";
    pub const CATEGORIES: &str = "categories";
    pub const CHANNELIDS: &str = "channelIds";
    pub const CHANNELS: &str = "channels";
    pub const CHAT: &str = "chat";
    pub const CHATS: &str = "chats";
    pub const CLIENT: &str = "client";
    pub const DONATION: &str = "donation";
    pub const EVENTS: &str = "events";
    pub const FOLLOWERS: &str = "followers";
    pub const LIVES: &str = "lives";
    pub const ME: &str = "me";
    pub const NEXT: &str = "next";
    pub const NOTICE: &str = "notice";
    pub const PAGE: &str = "page";
    pub const QUERY: &str = "query";
    pub const RESTRICT_CHANNELS: &str = "restrict-channels";
    pub const SEARCH: &str = "search";
    pub const SEND: &str = "send";
    pub const SESSIONS: &str = "sessions";
    pub const SESSION_KEY: &str = "sessionKey";
    pub const SETTING: &str = "setting";
    pub const SETTINGS: &str = "settings";
    pub const SIZE: &str = "size";
    pub const SORT: &str = "sort";
    pub const STREAMS: &str = "streams";
    pub const SUBSCRIBE: &str = "subscribe";
    pub const SUBSCRIBERS: &str = "subscribers";
    pub const SUBSCRIPTION: &str = "subscription";
    pub const UNSUBSCRIBE: &str = "unsubscribe";
    pub const USERS: &str = "users";
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub category_type: CategoryType,
    pub category_id: String,
    pub category_value: String,
    pub poster_image_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CategoryType {
    Game,
    Sports,
    Etc,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    pub next: Option<String>,
}
