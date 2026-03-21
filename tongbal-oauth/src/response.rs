use serde::Deserialize;

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: Option<String>,
}
