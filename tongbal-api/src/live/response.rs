use serde::{Deserialize, Serialize};

use crate::types::{Category, Page};

use super::Live;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveListResponse {
    pub data: Vec<Live>,
    pub page: Page,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamKey {
    pub stream_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSetting {
    pub default_live_title: String,
    pub category: Option<Category>,
    pub tags: Vec<String>,
}
