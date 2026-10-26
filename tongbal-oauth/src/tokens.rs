use serde::{Deserialize, Serialize};

use crate::{AccessToken, RefreshToken};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
}
