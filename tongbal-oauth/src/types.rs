use serde::{Deserialize, Serialize};

use crate::{AccessToken, AuthorizationCode, RefreshToken};

pub mod constants {
    pub const CLIENT_ID: &str = "clientId";
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthCallback {
    pub code: AuthorizationCode,
    pub state: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
}

impl GrantType {
    fn as_str(&self) -> &str {
        match self {
            Self::AuthorizationCode => "authorization_code",
            Self::RefreshToken => "refresh_token",
        }
    }
}

impl AsRef<str> for GrantType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenTypeHint {
    AccessToken,
    RefreshToken,
}

pub trait RevokeableToken: AsRef<str> {
    const TYPE_HINT: TokenTypeHint;
}

impl RevokeableToken for AccessToken {
    const TYPE_HINT: TokenTypeHint = TokenTypeHint::AccessToken;
}

impl RevokeableToken for RefreshToken {
    const TYPE_HINT: TokenTypeHint = TokenTypeHint::RefreshToken;
}
