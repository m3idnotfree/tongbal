use serde::{Deserialize, Serialize};

use crate::types::{CategoryType, ChannelId};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Live {
    pub live_id: u64,
    pub live_title: String,
    pub live_thumbnail_image_url: Option<String>,
    pub concurrent_user_count: u64,
    pub open_date: String,
    pub adult: bool,
    pub tags: Vec<String>,
    pub category_type: CategoryType,
    pub live_category: String,
    pub live_category_value: String,
    pub channel_id: ChannelId,
    pub channel_name: String,
    pub channel_image_url: Option<String>,
}
