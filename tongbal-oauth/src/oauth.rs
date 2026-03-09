use std::str::FromStr;

use asknothingx2_util::api::IntoRequestBuilder;
use reqwest::Client;
use url::Url;

use crate::{
    APIResponse, AuthUrl, AuthorizationCode, ClientId, ClientSecret, Error, RedirectUrl,
    RefreshToken, RevocationUrl, Token, TokenUrl,
    csrf::{self, CsrfConfig, generate, generate_secret_key},
    request::{ExchangeCode, RefreshRequest, RevokeRequest},
    response::ErrorResponse,
    types::{RevokeableToken, constants},
};

const AUTH_URL: &str = "https://chzzk.naver.com/account-interlock";
const TOKEN_URL: &str = "https://openapi.chzzk.naver.com/auth/v1/token";
const REVOKE_URL: &str = "https://openapi.chzzk.naver.com/auth/v1/token/revoke";

#[derive(Clone)]
pub struct TongbalOauth {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUrl,
    auth_url: AuthUrl,
    token_url: TokenUrl,
    revoke_url: RevocationUrl,
    secret_key: [u8; 32],
    csrf_config: CsrfConfig,
    client: Client,
}

impl TongbalOauth {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            auth_url: AuthUrl::from_str(AUTH_URL).unwrap(),
            token_url: TokenUrl::from_str(TOKEN_URL).unwrap(),
            revoke_url: RevocationUrl::from_str(REVOKE_URL).unwrap(),
            secret_key: generate_secret_key(),
            csrf_config: CsrfConfig::default(),
            client: crate::client::get().clone(),
        }
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    #[deprecated(note = "accessing client_secret is discouraged for security reasons")]
    pub fn client_secret(&self) -> &ClientSecret {
        &self.client_secret
    }

    pub fn with_auth_url(mut self, auth_url: AuthUrl) -> Self {
        self.auth_url = auth_url;
        self
    }

    pub fn with_token_url(mut self, token_url: TokenUrl) -> Self {
        self.token_url = token_url;
        self
    }

    pub fn with_rovoke_url(mut self, revoke_url: RevocationUrl) -> Self {
        self.revoke_url = revoke_url;
        self
    }

    pub fn with_secret_key(mut self, secret_key: [u8; 32]) -> Self {
        self.secret_key = secret_key;
        self
    }

    pub fn with_csrf_config(mut self, config: CsrfConfig) -> Self {
        self.csrf_config = config;
        self
    }

    /// Override the HTTP client
    ///
    /// Note: This only affects this OAuth instance, not the global client.
    /// For global configuration, use [client::setup()](crate::client::setup()) instead.
    pub fn with_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }

    pub fn set_client_secret(&mut self, client_secret: ClientSecret) {
        self.client_secret = client_secret;
    }

    pub fn set_redirect_uri(&mut self, redirect_uri: RedirectUrl) {
        self.redirect_uri = redirect_uri;
    }

    pub fn set_secret_key(&mut self, secret_key: [u8; 32]) {
        self.secret_key = secret_key;
    }

    pub fn set_csrf_config(&mut self, config: CsrfConfig) {
        self.csrf_config = config;
    }

    pub fn auth_url(&self) -> Url {
        let state = generate(&self.secret_key, Some(&self.client_id));

        let mut url = self.auth_url.to_url();
        url.query_pairs_mut().extend_pairs([
            (constants::CLIENT_ID, self.client_id.as_ref()),
            ("redirectUri", self.redirect_uri.as_ref()),
            ("state", &state),
        ]);

        url
    }

    pub fn auth_url_with_state(&self) -> (Url, String) {
        let state = generate(&self.secret_key, Some(&self.client_id));

        let mut url = self.auth_url.to_url();
        url.query_pairs_mut().extend_pairs([
            (constants::CLIENT_ID, self.client_id.as_ref()),
            ("redirectUri", self.redirect_uri.as_ref()),
            ("state", &state),
        ]);

        (url, state)
    }

    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
        state: String,
    ) -> Result<APIResponse<Token>, Error> {
        if csrf::verify_with_config(
            &self.secret_key,
            &state,
            Some(&self.client_id),
            &self.csrf_config,
        )
        .is_err()
        {
            return Err(Error::CsrfTokenMismatch);
        }

        let req = ExchangeCode::new(
            &self.token_url,
            &self.client_id,
            &self.client_secret,
            code,
            state,
        );

        self.send(req).await?.json().await.map_err(Error::from)
    }

    pub async fn refresh_token(
        &self,
        refresh_token: RefreshToken,
    ) -> Result<APIResponse<Token>, Error> {
        let req = RefreshRequest::new(
            &self.token_url,
            &self.client_id,
            &self.client_secret,
            refresh_token,
        );

        self.send(req).await?.json().await.map_err(Error::from)
    }

    pub async fn revoke_token<T: RevokeableToken>(&self, token: T) -> Result<(), Error> {
        let req = RevokeRequest::new(
            &self.revoke_url,
            &self.client_id,
            &self.client_secret,
            token,
        );

        self.send(req).await?;
        Ok(())
    }

    async fn send<T>(&self, request: T) -> Result<reqwest::Response, T::Error>
    where
        T: IntoRequestBuilder<Error = Error>,
    {
        let resp = request
            .into_request_builder(&self.client)?
            .send()
            .await
            .map_err(Error::from)?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let v = resp.bytes().await?;

            let (code, message) = serde_json::from_slice::<ErrorResponse>(&v)
                .map(|f| (f.code, f.message.unwrap_or_default()))
                .unwrap_or_else(|_| (status, String::from_utf8_lossy(&v).into_owned()));

            return Err(Error::OAuthError { code, message });
        }

        Ok(resp)
    }
}
