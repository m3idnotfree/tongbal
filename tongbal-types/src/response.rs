use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response<T> {
    pub code: u16,
    pub message: Option<String>,
    pub content: T,
}
