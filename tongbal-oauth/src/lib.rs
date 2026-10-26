//! OAuth 2.0 authorization client for the Chzzk API.
//!
//! Handles the authorization code flow.
//!
//! ## Quick Start
//!
//! ```no_run
//! use std::str::FromStr;
//! use tongbal_oauth::{Client, ClientId, ClientSecret, RedirectUrl};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let oauth = Client::new(
//!         ClientId::from("client_id"),
//!         ClientSecret::from("client_secret"),
//!         RedirectUrl::from_str("http://localhost:3000/auth/callback")?,
//!     );
//!
//!     let auth_url = oauth.auth_url();
//!     println!("Visit: {}", auth_url);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Handling OAuth Callbacks
//!
//! ```no_run
//! use tongbal_oauth::{AuthCallback, Client, Error};
//!
//! async fn handle_callback(
//!     oauth: &Client,
//!     callback: AuthCallback,
//! ) -> Result<(), Error> {
//!     let token = oauth
//!         .exchange_code(callback.code, callback.state)
//!         .await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Token Management
//!
//! ### Refresh
//!
//! ```rust
//! # use tongbal_oauth::{Client, Error, RefreshToken};
//! # async fn run(oauth: Client, refresh_token: RefreshToken) -> Result<(), Error> {
//! let new_token = oauth.refresh_token(refresh_token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Revocation
//!
//! Both [`AccessToken`] and [`RefreshToken`] can be revoked.
//!
//! ```rust
//! # use tongbal_oauth::{AccessToken, Client, Error, RefreshToken};
//! # async fn run(oauth: Client, access_token: AccessToken, refresh_token: RefreshToken) -> Result<(), Error> {
//! oauth.revoke_token(access_token).await?;
//! oauth.revoke_token(refresh_token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## HTTP Client
//!
//! A suitable HTTP client for authorization is used by default.
#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
mod error;
mod request;
mod tokens;
mod types;

pub use client::{Client, ClientBuilder};
pub use error::{Error, ErrorKind};
pub use tokens::Token;
pub use types::AuthCallback;

pub use asknothingx2_util::oauth::{AuthorizationCode, RedirectUrl, RefreshToken};
pub use tongbal_types::{AccessToken, ClientId, ClientSecret, Response};

pub mod csrf {
    pub use asknothingx2_util::oauth::signed_token::{
        TokenConfig as CsrfConfig, TokenError, current_timestamp, extract_datetime,
        extract_timestamp, generate, generate_at_time, generate_secret_key, is_expired, token_age,
        verify, verify_at_time, verify_with_config,
    };
}

static HTTP_CLIENT: std::sync::LazyLock<reqwest::Client> = std::sync::LazyLock::new(|| {
    asknothingx2_http::config::preset::authentication(
        reqwest::Client::builder(),
        http::HeaderValue::from_static(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        )),
    )
    .build()
    .expect("the default HTTP client configuration should be valid")
});
