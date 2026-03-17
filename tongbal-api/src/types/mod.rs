mod ids;

pub use ids::{ChannelId, MessageId};

use serde::{Deserialize, Serialize};

pub(crate) mod constants {
    pub const CATEGORIES: &str = "categories";
    pub const LIVES: &str = "lives";
    pub const ME: &str = "me";
    pub const NOTICE: &str = "notice";
    pub const QUERY: &str = "query";
    pub const SEARCH: &str = "search";
    pub const SETTING: &str = "setting";
    pub const SETTINGS: &str = "settings";
    pub const SIZE: &str = "size";
    pub const STREAMS: &str = "streams";
    pub const USERS: &str = "users";
    pub const CHATS: &str = "chats";
    pub const SEND: &str = "send";
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
