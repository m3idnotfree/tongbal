use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIResponse<T> {
    pub code: u16,
    pub message: Option<String>,
    pub content: T,
}
