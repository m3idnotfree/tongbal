//! See <https://chzzk.gitbook.io/chzzk>
#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
mod error;
mod response;

pub use client::{AppClient, BaseClient, Client};
pub use error::Error;
pub use response::APIResponse;

pub use asknothingx2_util::oauth::{AccessToken, ClientId, ClientSecret};
