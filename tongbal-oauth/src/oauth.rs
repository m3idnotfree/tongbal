use asknothingx2_http::{ClientService, client::Reqwest};
use reqwest::Client;
use url::Url;

use crate::{
    AuthorizationCode, ClientId, ClientSecret, Error, RedirectUrl, RefreshToken, Response, Token,
    csrf::{self, CsrfConfig, generate, generate_secret_key},
    error,
    request::{AUTH_URL, ExchangeCode, IntoRequest, RefreshRequest, RevokeRequest},
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
    svc: ClientService,
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
            svc: ClientService::new(Reqwest::new(crate::client::get().clone())),
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

        self.json(ExchangeCode::new(
            &self.client_id,
            &self.client_secret,
            code,
            state,
        ))
        .await
    }

    pub async fn refresh_token(
        &self,
        refresh_token: RefreshToken,
    ) -> Result<Response<Token>, Error> {
        self.json(RefreshRequest::new(
            &self.client_id,
            &self.client_secret,
            refresh_token,
        ))
        .await
    }

    pub async fn revoke_token<T: RevokeableToken>(&self, token: T) -> Result<(), Error> {
        self.send(RevokeRequest::new(
            &self.client_id,
            &self.client_secret,
            token,
        ))
        .await?;

        Ok(())
    }

    async fn json<T, R>(&self, request: T) -> Result<R, Error>
    where
        T: IntoRequest,
        R: serde::de::DeserializeOwned,
    {
        let resp = self.send(request).await?;

        let body = resp
            .into_body()
            .collect_bytes()
            .await
            .map_err(|e| error::body(T::OPERATION, e.into_inner()))?;

        let mut de = serde_json::Deserializer::from_slice(&body);

        let value =
            serde_path_to_error::deserialize(&mut de).map_err(|e| error::parse(T::OPERATION, e))?;

        de.end().map_err(|e| error::parse(T::OPERATION, e))?;

        Ok(value)
    }

    async fn send<T: IntoRequest>(&self, request: T) -> Result<asknothingx2_http::Response, Error> {
        let req = request
            .into_request()
            .map_err(|e| error::build(T::OPERATION, e))?;

        let resp = self
            .svc
            .clone()
            .execute(req)
            .await
            .map_err(|e| error::request(T::OPERATION, e.into_inner()))?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.into_body().collect_bytes().await.unwrap_or_default();

            return Err(error::api(T::OPERATION, status, &body));
        }

        Ok(resp)
    }
}
