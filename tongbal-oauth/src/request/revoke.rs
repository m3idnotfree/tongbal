use asknothingx2_http::{Body, HeaderMapExt};
use serde::Serialize;

use crate::{
    ClientId, ClientSecret,
    request::{IntoRequest, REVOKE_URL},
    types::{RevokeableToken, constants},
};

pub struct RevokeRequest<'a, T> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    token: T,
}

impl<'a, T> RevokeRequest<'a, T>
where
    T: RevokeableToken,
{
    pub fn new(client_id: &'a ClientId, client_secret: &'a ClientSecret, token: T) -> Self {
        Self {
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

impl<T> IntoRequest for RevokeRequest<'_, T>
where
    T: RevokeableToken,
{
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
    use crate::{AccessToken, ClientId, ClientSecret, RefreshToken};

    use super::RevokeRequest;

    #[test]
    fn request_body_with_access_token() {
        let client_id = ClientId::from("client_id");
        let client_secret = ClientSecret::from("client_secret");
        let token = AccessToken::from("test_access_token");

        let req = RevokeRequest::new(&client_id, &client_secret, token);
        let body = serde_json::to_value(&req).unwrap();

        assert_eq!(body["clientId"], "client_id");
        assert_eq!(body["clientSecret"], "client_secret");
        assert_eq!(body["token"], "test_access_token");
        assert_eq!(body["tokenTypeHint"], "access_token");
        assert!(body.get("revokeUrl").is_none());
    }

    #[test]
    fn request_body_with_refresh_token() {
        let client_id = ClientId::from("client_id");
        let client_secret = ClientSecret::from("client_secret");
        let token = RefreshToken::from("test_refresh_token");

        let req = RevokeRequest::new(&client_id, &client_secret, token);
        let body = serde_json::to_value(&req).unwrap();

        assert_eq!(body["clientId"], "client_id");
        assert_eq!(body["clientSecret"], "client_secret");
        assert_eq!(body["token"], "test_refresh_token");
        assert_eq!(body["tokenTypeHint"], "refresh_token");
        assert!(body.get("revokeUrl").is_none());
    }
}
