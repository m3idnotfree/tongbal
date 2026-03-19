use serde::{Deserialize, Serialize};

use crate::{Tier, new_types::ChannelId};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub channel_id: ChannelId,
    pub subscriber_channel_id: ChannelId,
    pub subscriber_nickname: String,
    pub tier_no: Tier,
    pub tier_name: String,
    pub month: u64,
}
