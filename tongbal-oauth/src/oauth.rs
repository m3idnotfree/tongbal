use reqwest::Client;
use url::Url;

use crate::{
    AuthorizationCode, ClientId, ClientSecret, Error, RedirectUrl, RefreshToken, Response, Token,
    csrf::{self, CsrfConfig, generate, generate_secret_key},
    error,
    request::{AUTH_URL, ExchangeCode, IntoRequest, RefreshRequest, RevokeRequest},
    response::ErrorResponse,
    types::{RevokeableToken, constants},
};

#[derive(Clone)]
pub struct TongbalOauth {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUrl,
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

        let mut url = Url::parse(AUTH_URL).expect("AUTH_URL is a valid URL");
        url.query_pairs_mut().extend_pairs([
            (constants::CLIENT_ID, self.client_id.as_ref()),
            ("redirectUri", self.redirect_uri.as_ref()),
            ("state", &state),
        ]);

        url
    }

    pub fn auth_url_with_state(&self) -> (Url, String) {
        let state = generate(&self.secret_key, Some(&self.client_id));

        let mut url = Url::parse(AUTH_URL).expect("AUTH_URL is a valid URL");
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
    ) -> Result<Response<Token>, Error> {
        csrf::verify_with_config(
            &self.secret_key,
            &state,
            Some(&self.client_id),
            &self.csrf_config,
        )
        .map_err(|e| error::csrf(error::Operation::ExchangeCode, e))?;

        let req = ExchangeCode::new(&self.client_id, &self.client_secret, code, state);

        self.send(req)
            .await?
            .json()
            .await
            .map_err(|e| error::parse(error::Operation::ExchangeCode, e))
    }

    pub async fn refresh_token(
        &self,
        refresh_token: RefreshToken,
    ) -> Result<Response<Token>, Error> {
        let req = RefreshRequest::new(&self.client_id, &self.client_secret, refresh_token);

        self.send(req)
            .await?
            .json()
            .await
            .map_err(|e| error::parse(error::Operation::RefreshToken, e))
    }

    pub async fn revoke_token<T: RevokeableToken>(&self, token: T) -> Result<(), Error> {
        let req = RevokeRequest::new(&self.client_id, &self.client_secret, token);

        self.send(req).await?;
        Ok(())
    }

    async fn send<T>(&self, request: T) -> Result<reqwest::Response, Error>
    where
        T: IntoRequest,
    {
        let req = request
            .into_request()
            .map_err(|e| error::build(T::OPERATION, e))?;
        let req = reqwest::Request::try_from(req).map_err(|e| error::request(T::OPERATION, e))?;

        let resp = self
            .client
            .execute(req)
            .await
            .map_err(|e| error::request(T::OPERATION, e))?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let v = resp.bytes().await.unwrap_or_default();

            let (code, message) = serde_json::from_slice::<ErrorResponse>(&v)
                .map(|f| (f.code, f.message.unwrap_or_default()))
                .unwrap_or_else(|_| (status, String::from_utf8_lossy(&v).into_owned()));

            return Err(error::api(T::OPERATION, code, message));
        }

        Ok(resp)
    }
}
