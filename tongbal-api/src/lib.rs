//! See <https://chzzk.gitbook.io/chzzk>
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod category;
pub mod channel;
pub mod chat;
pub mod live;
pub mod restriction;
pub mod session;
pub mod types;
pub mod user;

mod client;
mod error;

pub use client::{AppClient, BaseClient, UserClient};
pub use error::Error;

pub use tongbal_types::{AccessToken, ClientId, ClientSecret, Response};
