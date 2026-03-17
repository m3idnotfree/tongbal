mod common;

use common::{AppHttpMock, UserHttpMock};
use tongbal_api::channel::{AppChannelAPI, UserChannelAPI};

#[tokio::test]
async fn get_channel_moderators() {
    let suite = UserHttpMock::new().await;
    suite.get_channel_moderators().await;

    let result = suite.client().get_channel_moderators().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_channel_followers() {
    let suite = UserHttpMock::new().await;
    suite.get_channel_followers().await;

    let result = suite.client().get_channel_followers().send().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_channel_subscribers() {
    let suite = UserHttpMock::new().await;
    suite.get_channel_subscribers().await;

    let result = suite.client().get_channel_subscribers().send().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_channels() {
    let suite = AppHttpMock::new().await;
    suite.get_channels().await;

    let result = suite.client().get_channels(&[]).await;

    assert!(result.is_ok());
}
