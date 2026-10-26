use asknothingx2_http::{ClientService, HttpService, client::Reqwest};
use tower_layer::{Identity, Layer, Stack};
use url::Url;

use crate::{
    AuthorizationCode, ClientId, ClientSecret, Error, RedirectUrl, RefreshToken, Token,
    csrf::{self, generate, generate_secret_key},
    error,
    request::{AUTH_URL, ExchangeCode, IntoRequest, RefreshRequest, RevokeRequest},
    types::{RevokeableToken, constants},
};

pub struct ClientBuilder<L = Identity> {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUrl,
    csrf_key: Option<[u8; 32]>,
    csrf_config: Option<csrf::Config>,
    layers: L,
}

impl ClientBuilder<Identity> {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUrl,
    ) -> Self {
        ClientBuilder {
            client_id,
            client_secret,
            redirect_uri,
            csrf_key: None,
            csrf_config: None,
            layers: Identity::new(),
        }
    }
}

impl<L> ClientBuilder<L> {
    pub fn csrf_key(mut self, csrf_key: [u8; 32]) -> Self {
        self.csrf_key = Some(csrf_key);
        self
    }

    pub fn csrf_config(mut self, csrf_config: csrf::Config) -> Self {
        self.csrf_config = Some(csrf_config);
        self
    }

    pub fn layer<T>(self, layer: T) -> ClientBuilder<Stack<T, L>> {
        ClientBuilder {
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri: self.redirect_uri,
            csrf_config: self.csrf_config,
            csrf_key: self.csrf_key,
            layers: Stack::new(layer, self.layers),
        }
    }

    pub fn build(self) -> Client
    where
        L: Layer<Reqwest, Service: HttpService>,
    {
        self.http_client(crate::HTTP_CLIENT.clone())
    }

    pub fn http_client(self, http_client: reqwest::Client) -> Client
    where
        L: Layer<Reqwest, Service: HttpService>,
    {
        self.service(Reqwest::new(http_client))
    }

    pub fn service<S>(self, service: S) -> Client
    where
        L: Layer<S, Service: HttpService>,
    {
        Client {
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri: self.redirect_uri,
            csrf_key: self.csrf_key.unwrap_or_else(generate_secret_key),
            csrf_config: self.csrf_config.unwrap_or_default(),
            svc: ClientService::new(self.layers.layer(service)),
        }
    }
}

#[derive(Clone)]
pub struct Client {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: RedirectUrl,
    csrf_key: [u8; 32],
    csrf_config: csrf::Config,
    svc: ClientService,
}

impl Client {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUrl,
    ) -> Self {
        ClientBuilder::new(client_id, client_secret, redirect_uri).build()
    }

    pub fn builder(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: RedirectUrl,
    ) -> ClientBuilder {
        ClientBuilder::new(client_id, client_secret, redirect_uri)
    }

    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    #[deprecated(note = "accessing client_secret is discouraged for security reasons")]
    pub fn client_secret(&self) -> &ClientSecret {
        &self.client_secret
    }

    pub fn redirect_uri(&self) -> &RedirectUrl {
        &self.redirect_uri
    }

    pub fn set_client_secret(&mut self, client_secret: ClientSecret) {
        self.client_secret = client_secret;
    }

    pub fn set_csrf_key(&mut self, csrf_key: [u8; 32]) {
        self.csrf_key = csrf_key;
    }

    pub fn set_csrf_config(&mut self, config: csrf::Config) {
        self.csrf_config = config;
    }

    pub fn auth_url(&self) -> (Url, String) {
        let state = generate(&self.csrf_key, Some(&self.client_id));

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
        code: &AuthorizationCode,
        state: &str,
    ) -> Result<Token, Error> {
        csrf::verify_with_config(
            &self.csrf_key,
            state,
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

    pub async fn refresh_token(&self, refresh_token: &RefreshToken) -> Result<Token, Error> {
        self.json(RefreshRequest::new(
            &self.client_id,
            &self.client_secret,
            refresh_token,
        ))
        .await
    }

    pub async fn revoke_token<T: RevokeableToken>(&self, token: &T) -> Result<(), Error> {
        self.send(RevokeRequest::new(
            &self.client_id,
            &self.client_secret,
            token.as_ref(),
            T::TYPE_HINT,
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

        let value: tongbal_types::Response<R> =
            serde_path_to_error::deserialize(&mut de).map_err(|e| error::parse(T::OPERATION, e))?;

        de.end().map_err(|e| error::parse(T::OPERATION, e))?;

        Ok(value.content)
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
