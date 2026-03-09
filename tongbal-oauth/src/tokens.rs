use chrono::Utc;
use serde::Deserialize;

use crate::{AccessToken, RefreshToken};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
    #[serde(default = "default_created_at")]
    pub created_at: i64,
}

impl Token {
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now >= (self.created_at + self.expires_in as i64)
    }
}

fn default_created_at() -> i64 {
    Utc::now().timestamp()
}
