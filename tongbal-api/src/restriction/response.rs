use serde::{Deserialize, Serialize};

use crate::types::Page;

use super::RestrictionChannel;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestrictionChannelResponse {
    pub data: Vec<RestrictionChannel>,
    pub page: Page,
}
