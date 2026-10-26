use asknothingx2_http::{Body, HeaderMapExt};
use serde::Serialize;

use crate::{
    AuthorizationCode, ClientId, ClientSecret,
    request::{IntoRequest, TOKEN_URL},
    types::GrantType,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeCode<'a> {
    grant_type: GrantType,
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    code: AuthorizationCode,
    state: String,
}

impl<'a> ExchangeCode<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        code: AuthorizationCode,
        state: String,
    ) -> Self {
        Self {
            grant_type: GrantType::AuthorizationCode,
            client_id,
            client_secret,
            code,
            state,
        }
    }
}

impl IntoRequest for ExchangeCode<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::ExchangeCode;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        let body = serde_json::to_vec(&self)?;
        let mut req = http::Request::post(TOKEN_URL).body(Body::from(body))?;
        req.headers_mut().content_type_json();
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use crate::{AuthorizationCode, ClientId, ClientSecret};

    use super::ExchangeCode;

    #[test]
    fn request_body() {
        let client_id = ClientId::from("client_id");
        let client_secret = ClientSecret::from("client_secret");
        let code = AuthorizationCode::from("test_code");
        let state = "test_state".to_string();

        let req = ExchangeCode::new(&client_id, &client_secret, code, state);
        let body = serde_json::to_value(&req).unwrap();

        assert_eq!(body["grantType"], "authorization_code");
        assert_eq!(body["clientId"], "client_id");
        assert_eq!(body["clientSecret"], "client_secret");
        assert_eq!(body["code"], "test_code");
        assert_eq!(body["state"], "test_state");
        assert!(body.get("tokenUrl").is_none());
    }
}
