//! Global HTTP Client Management for OAuth Request
//!
//! See also [`Preset documentation`](https://docs.rs/asknothingx2-util/latest/asknothingx2_util/api/preset/struct.Preset.html)
//!
//! This module manages a single, shared HTTP client used by all TongbalOauth instances
//! in your application. This design provides several benefits:
//!
//! - **Connection pooling**: Reuses HTTP connections across OAuth operations
//! - **Consistent configuration**: Same timeouts, headers, etc. for all requests
//! - **Memory efficiency**: One client instead of many
//! - **Thread safety**: Can be safely used from multiple threads
//!
//! # Default Behavior (No Setup Required)
//!
//! If you don't call [`setup()`], a default client is created automatically with:
//!
//! |                          |                    |
//! |--------------------------|--------------------|
#![doc = concat!("| `User-Agent` | `", env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"), "` |")]
//! | Request timeout          | 60s                |
//! | Connect timeout          | 10s                |
//! | Max connections per host | 30                 |
//! | Idle timeout             | 90s                |
//! | TLS                      | 1.2+               |
//! | HTTPS                    | enforced           |
//! | HTTP/2                   | required           |
//! | Redirects                | up to 5            |
//! | Cookies                  | disabled           |
//! | Referer                  | disabled           |
//! | `Accept`                 | `application/json` |
//! | `Content-Type`           | `application/json` |
//! | `Cache-Control`          | `no-cache`         |
//!
//! # Custom Configuration
//! ```
//! use std::time::Duration;
//!
//! use asknothingx2_util::api::preset;
//! use tongbal_oauth::{client, TongbalOauth};
//!
//! # async fn run() -> Result<(), tongbal_oauth::Error> {
//! let mut preset = preset::authentication("MyApp/1.0");
//! preset
//!     .timeouts(Duration::from_secs(60), Duration::from_secs(30))
//!     .connections(10, Duration::from_secs(90));
//!
//! preset
//!     .default_headers_mut()
//!     .accept_json()
//!     .content_type_json();
//!
//! // Configure once at startup
//! client::setup(preset.build()?)?;
//! # Ok(())
//! # }
//! ```

use std::sync::OnceLock;

use asknothingx2_util::api::preset;
use reqwest::Client;

use crate::Error;

static CLIENT: OnceLock<Client> = OnceLock::new();

/// Configure the global HTTP client used for all OAuth requests
///
/// This should be called once at application startup if you need custom
/// timeouts, proxies, or other HTTP client configuration.
pub fn setup(client: reqwest::Client) -> Result<(), Error> {
    if CLIENT.get().is_some() {
        return Err(Error::ClientSetup);
    }

    CLIENT.set(client).map_err(|_| Error::ClientSetup)?;

    Ok(())
}

/// Get the global HTTP client (creates default if not configured)
pub fn get() -> &'static Client {
    CLIENT.get_or_init(|| {
        let mut client = preset::authentication(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ));

        client
            .default_headers_mut()
            .accept_json()
            .content_type_json();

        client
            .build()
            .expect("build-in default configurtaion is always valid")
    })
}
