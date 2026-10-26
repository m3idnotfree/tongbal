use asknothingx2_http::{Body, HeaderMapExt};
use serde::Serialize;

use crate::{
    ClientId, ClientSecret, RefreshToken,
    request::{IntoRequest, TOKEN_URL},
    types::GrantType,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest<'a> {
    grant_type: GrantType,
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    refresh_token: RefreshToken,
}

impl<'a> RefreshRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        refresh_token: RefreshToken,
    ) -> Self {
        Self {
            grant_type: GrantType::RefreshToken,
            client_id,
            client_secret,
            refresh_token,
        }
    }
}

impl IntoRequest for RefreshRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::RefreshToken;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        let body = serde_json::to_vec(&self)?;
        let mut req = http::Request::post(TOKEN_URL).body(Body::from(body))?;
        req.headers_mut().content_type_json();
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClientId, ClientSecret, RefreshToken};

    use super::RefreshRequest;

    #[test]
    fn request_body() {
        let client_id = ClientId::from("client_id");
        let client_secret = ClientSecret::from("client_secret");
        let refresh_token = RefreshToken::from("refresh_token");

        let req = RefreshRequest::new(&client_id, &client_secret, refresh_token);
        let body = serde_json::to_value(&req).unwrap();

        assert_eq!(body["grantType"], "refresh_token");
        assert_eq!(body["clientId"], "client_id");
        assert_eq!(body["clientSecret"], "client_secret");
        assert_eq!(body["refreshToken"], "refresh_token");
        assert!(body.get("tokenUrl").is_none());
    }
}
