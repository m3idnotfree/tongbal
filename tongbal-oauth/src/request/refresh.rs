use asknothingx2_util::api::{IntoRequestBuilder, Method};
use serde::Serialize;

use crate::{ClientId, ClientSecret, Error, RefreshToken, TokenUrl, types::GrantType};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest<'a> {
    #[serde(skip)]
    token_url: &'a TokenUrl,
    grant_type: GrantType,
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    refresh_token: RefreshToken,
}

impl<'a> RefreshRequest<'a> {
    pub fn new(
        token_url: &'a TokenUrl,
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        refresh_token: RefreshToken,
    ) -> Self {
        Self {
            token_url,
            grant_type: GrantType::RefreshToken,
            client_id,
            client_secret,
            refresh_token,
        }
    }
}

impl<'a> IntoRequestBuilder for RefreshRequest<'a> {
    type Error = Error;
    fn into_request_builder(
        self,
        client: &reqwest::Client,
    ) -> Result<reqwest::RequestBuilder, Self::Error> {
        Ok(client
            .request(Method::POST, self.token_url.to_url())
            .json(&self))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{ClientId, ClientSecret, RefreshToken, TokenUrl};

    use super::RefreshRequest;

    #[test]
    fn request_body() {
        let token_url =
            TokenUrl::from_str("https://openapi.chzzk.naver.com/auth/v1/token").unwrap();
        let client_id = ClientId::from("client_id");
        let client_secret = ClientSecret::from("client_secret");
        let refresh_token = RefreshToken::from("refresh_token");

        let req = RefreshRequest::new(&token_url, &client_id, &client_secret, refresh_token);
        let body = serde_json::to_value(&req).unwrap();

        assert_eq!(body["grantType"], "refresh_token");
        assert_eq!(body["clientId"], "client_id");
        assert_eq!(body["clientSecret"], "client_secret");
        assert_eq!(body["refreshToken"], "refresh_token");
        assert!(body.get("tokenUrl").is_none());
    }
}
