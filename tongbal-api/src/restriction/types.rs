use serde::{Deserialize, Serialize};

use crate::types::ChannelId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestrictionChannel {
    pub restricted_channel_id: ChannelId,
    pub restricted_channel_name: String,
    pub created_date: String,
    pub release_date: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestrictionBody<'a> {
    pub target_channel_id: &'a ChannelId,
}
