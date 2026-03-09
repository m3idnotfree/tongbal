use asknothingx2_util::api::{IntoRequestBuilder, Method};
use serde::Serialize;

use crate::{
    ClientId, ClientSecret, Error, RevocationUrl,
    types::{RevokeableToken, constants},
};

pub struct RevokeRequest<'a, T> {
    revoke_url: &'a RevocationUrl,
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    token: T,
}

impl<'a, T> RevokeRequest<'a, T>
where
    T: RevokeableToken,
{
    pub fn new(
        revoke_url: &'a RevocationUrl,
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        token: T,
    ) -> Self {
        Self {
            revoke_url,
            client_id,
            client_secret,
            token,
        }
    }
}

impl<'a, T: RevokeableToken> Serialize for RevokeRequest<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry(constants::CLIENT_ID, self.client_id)?;
        map.serialize_entry("clientSecret", self.client_secret)?;
        map.serialize_entry("token", self.token.as_token_str())?;
        map.serialize_entry("tokenTypeHint", self.token.type_hint())?;
        map.end()
    }
}

impl<'a, T> IntoRequestBuilder for RevokeRequest<'a, T>
where
    T: RevokeableToken,
{
    type Error = Error;
    fn into_request_builder(
        self,
        client: &reqwest::Client,
    ) -> Result<reqwest::RequestBuilder, Self::Error> {
        Ok(client
            .request(Method::POST, self.revoke_url.to_url())
            .json(&self))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{AccessToken, ClientId, ClientSecret, RefreshToken, RevocationUrl};

    use super::RevokeRequest;

    #[test]
    fn request_body_with_access_token() {
        let revoke_url =
            RevocationUrl::from_str("https://openapi.chzzk.naver.com/auth/v1/token/revoke")
                .unwrap();
        let client_id = ClientId::from("client_id");
        let client_secret = ClientSecret::from("client_secret");
        let token = AccessToken::from("test_access_token");

        let req = RevokeRequest::new(&revoke_url, &client_id, &client_secret, token);
        let body = serde_json::to_value(&req).unwrap();

        assert_eq!(body["clientId"], "client_id");
        assert_eq!(body["clientSecret"], "client_secret");
        assert_eq!(body["token"], "test_access_token");
        assert_eq!(body["tokenTypeHint"], "access_token");
        assert!(body.get("revokeUrl").is_none());
    }

    #[test]
    fn request_body_with_refresh_token() {
        let revoke_url =
            RevocationUrl::from_str("https://openapi.chzzk.naver.com/auth/v1/token/revoke")
                .unwrap();
        let client_id = ClientId::from("client_id");
        let client_secret = ClientSecret::from("client_secret");
        let token = RefreshToken::from("test_refresh_token");

        let req = RevokeRequest::new(&revoke_url, &client_id, &client_secret, token);
        let body = serde_json::to_value(&req).unwrap();

        assert_eq!(body["clientId"], "client_id");
        assert_eq!(body["clientSecret"], "client_secret");
        assert_eq!(body["token"], "test_refresh_token");
        assert_eq!(body["tokenTypeHint"], "refresh_token");
        assert!(body.get("revokeUrl").is_none());
    }
}
