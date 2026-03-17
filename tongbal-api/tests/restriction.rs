mod common;

use common::UserHttpMock;
use tongbal_api::{restriction::RestrictionAPI, types::ChannelId};

#[tokio::test]
async fn add_restraction() {
    let suite = UserHttpMock::new().await;
    suite.add_restraction().await;

    let result = suite
        .client()
        .add_restraction(&ChannelId::from("909501f048b44cf0d5c1d28XXXXXXXX"))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn remove_restriction() {
    let suite = UserHttpMock::new().await;
    suite.remove_restriction().await;

    let result = suite
        .client()
        .remove_restriction(&ChannelId::from("909501f048b44cf0d5c1d28XXXXXXXX"))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn get_restrictions() {
    let suite = UserHttpMock::new().await;
    suite.get_restrictions().await;

    let result = suite.client().get_restrictions().send().await;
    assert!(result.is_ok());
}
