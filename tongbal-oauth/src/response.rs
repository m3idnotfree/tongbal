use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct APIResponse<T> {
    pub code: u16,
    pub message: Option<String>,
    pub content: T,
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: Option<String>,
}
