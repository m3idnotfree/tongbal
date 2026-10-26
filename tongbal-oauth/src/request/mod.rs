mod exchange_code;
mod refresh;
mod revoke;

pub use exchange_code::ExchangeCode;
pub use refresh::RefreshRequest;
pub use revoke::RevokeRequest;

pub(crate) const AUTH_URL: &str = "https://chzzk.naver.com/account-interlock";
const TOKEN_URL: &str = "https://openapi.chzzk.naver.com/auth/v1/token";
const REVOKE_URL: &str = "https://openapi.chzzk.naver.com/auth/v1/token/revoke";
