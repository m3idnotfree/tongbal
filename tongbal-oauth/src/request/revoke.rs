use asknothingx2_http::{Body, HeaderMapExt};
use serde::Serialize;

use crate::{
    ClientId, ClientSecret,
    request::{IntoRequest, REVOKE_URL},
    types::TokenTypeHint,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevokeRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    token: &'a str,
    token_type_hint: TokenTypeHint,
}

impl<'a> RevokeRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        token: &'a str,
        token_type_hint: TokenTypeHint,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            token,
            token_type_hint,
        }
    }
}

impl IntoRequest for RevokeRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::RevokeToken;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        let body = serde_json::to_vec(&self)?;
        let mut req = http::Request::post(REVOKE_URL).body(Body::from(body))?;
        req.headers_mut().content_type_json();
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use crate::{AccessToken, ClientId, ClientSecret, RefreshToken, types::TokenTypeHint};

    use super::RevokeRequest;

    #[test]
    fn request_body_with_access_token() {
        let client_id = ClientId::from("client_id");
        let client_secret = ClientSecret::from("client_secret");
        let token = AccessToken::from("test_access_token");

        let req = RevokeRequest::new(
            &client_id,
            &client_secret,
            &token,
            TokenTypeHint::AccessToken,
        );
        let body = serde_json::to_value(&req).unwrap();

        assert_eq!(body["clientId"], "client_id");
        assert_eq!(body["clientSecret"], "client_secret");
        assert_eq!(body["token"], "test_access_token");
        assert_eq!(body["tokenTypeHint"], "access_token");
    }

    #[test]
    fn request_body_with_refresh_token() {
        let client_id = ClientId::from("client_id");
        let client_secret = ClientSecret::from("client_secret");
        let token = RefreshToken::from("test_refresh_token");

        let req = RevokeRequest::new(
            &client_id,
            &client_secret,
            &token,
            TokenTypeHint::RefreshToken,
        );
        let body = serde_json::to_value(&req).unwrap();

        assert_eq!(body["clientId"], "client_id");
        assert_eq!(body["clientSecret"], "client_secret");
        assert_eq!(body["token"], "test_refresh_token");
        assert_eq!(body["tokenTypeHint"], "refresh_token");
    }
}
