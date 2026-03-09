#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Build(#[from] asknothingx2_util::api::Error),
    #[error("OAuth error (code: {code}): {message}")]
    OAuthError { code: u16, message: String },
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("CSRF token validation failed - possible security issue")]
    CsrfTokenMismatch,
    #[error("HTTP client has already been initialized and cannot be reconfigured")]
    ClientSetup,
}
