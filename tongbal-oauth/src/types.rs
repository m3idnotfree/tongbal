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

pub trait RevokeableToken {
    fn as_token_str(&self) -> &str;
    fn type_hint(&self) -> &'static str;
}

impl RevokeableToken for AccessToken {
    fn as_token_str(&self) -> &str {
        self.as_ref()
    }

    fn type_hint(&self) -> &'static str {
        "access_token"
    }
}

impl RevokeableToken for RefreshToken {
    fn as_token_str(&self) -> &str {
        self.as_ref()
    }

    fn type_hint(&self) -> &'static str {
        "refresh_token"
    }
}
