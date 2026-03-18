use std::sync::LazyLock;

use asknothingx2_util::{
    api::{HeaderMut, preset},
    oauth::{AccessToken, ClientId, ClientSecret},
};
use serde::de::DeserializeOwned;
use url::Url;

use crate::Error;

const CHZZK_API_URL: &str = "https://openapi.chzzk.naver.com/open/v1";

static BASE_URL: LazyLock<Url> = LazyLock::new(|| url::Url::parse(CHZZK_API_URL).unwrap());

pub trait BaseClient {
    fn http_client(&self) -> &reqwest::Client;
    fn base_url(&self) -> Url;
}

#[derive(Debug, Clone)]
pub struct UserClient {
    url: Url,
    client: reqwest::Client,
}

#[derive(Debug, Clone)]
pub struct AppClient {
    url: Url,
    client: reqwest::Client,
}

impl UserClient {
    pub fn new(access_token: AccessToken) -> Self {
        let mut client = preset::rest_api(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ));

        client
            .default_headers_mut()
            .bearer_token(access_token.secret())
            .content_type_json();

        Self {
            url: BASE_URL.clone(),
            client: client.build().unwrap(),
        }
    }

    pub fn with_client_builder(access_token: AccessToken, builder: reqwest::ClientBuilder) -> Self {
        use reqwest::header;
        let mut headers = header::HeaderMap::new();
        HeaderMut::new(&mut headers).bearer_token(access_token.secret());

        let client = builder.default_headers(headers);

        Self {
            url: BASE_URL.clone(),
            client: client.build().unwrap(),
        }
    }

    pub fn with_url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }
}

impl AppClient {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Self {
        let mut client = preset::rest_api(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ));

        client
            .default_headers_mut()
            .client_id(&client_id)
            .expect("client_id must be valid ASCII")
            .client_secret(&client_secret)
            .expect("client_secret must be valid ASCII")
            .content_type_json();

        Self {
            url: BASE_URL.clone(),
            client: client.build().unwrap(),
        }
    }

    pub fn with_client_builder(
        client_id: ClientId,
        client_secret: ClientSecret,
        builder: reqwest::ClientBuilder,
    ) -> Self {
        use reqwest::header;
        let mut headers = header::HeaderMap::new();
        HeaderMut::new(&mut headers)
            .client_id(&client_id)
            .expect("client_id must be valid ASCII")
            .client_secret(&client_secret)
            .expect("client_secret must be valid ASCII")
            .content_type_json();

        let client = builder.default_headers(headers);

        Self {
            url: BASE_URL.clone(),
            client: client.build().unwrap(),
        }
    }

    pub fn with_url(mut self, url: Url) -> Self {
        self.url = url;
        self
    }
}

impl BaseClient for UserClient {
    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    fn base_url(&self) -> Url {
        self.url.clone()
    }
}

impl BaseClient for AppClient {
    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    fn base_url(&self) -> Url {
        self.url.clone()
    }
}

pub async fn json<T: DeserializeOwned>(req: reqwest::RequestBuilder) -> Result<T, Error> {
    execute(req).await?.json().await.map_err(Error::from)
}

pub async fn no_content(req: reqwest::RequestBuilder) -> Result<(), Error> {
    execute(req).await?;
    Ok(())
}

pub async fn execute(req: reqwest::RequestBuilder) -> Result<reqwest::Response, Error> {
    let resp = req.send().await.map_err(Error::from)?;
    if resp.status().is_success() {
        Ok(resp)
    } else {
        let code = resp.status().as_u16();
        let v = resp.bytes().await?;
        let message = String::from_utf8_lossy(&v).into_owned();
        Err(Error::APIError { code, message })
    }
}
