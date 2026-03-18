use serde::{Deserialize, Serialize};
use url::Url;

use super::Session;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSessionResponse {
    pub url: Url,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionResponse {
    pub data: Vec<Session>,
}
