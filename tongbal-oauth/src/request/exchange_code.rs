use asknothingx2_util::api::{IntoRequestBuilder, Method};
use serde::Serialize;

use crate::{
    AuthorizationCode, ClientId, ClientSecret, Error, request::TOKEN_URL, types::GrantType,
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

impl<'a> IntoRequestBuilder for ExchangeCode<'a> {
    type Error = Error;
    fn into_request_builder(
        self,
        client: &reqwest::Client,
    ) -> Result<reqwest::RequestBuilder, Self::Error> {
        Ok(client.request(Method::POST, TOKEN_URL).json(&self))
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
