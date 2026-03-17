mod common;

use common::{AppHttpMock, UserHttpMock};
use tongbal_api::live::{AppLiveAPI, UserLiveAPI};

#[tokio::test]
async fn get_live_list() {
    let suite = AppHttpMock::new().await;
    suite.get_live_list().await;

    let result = suite.client().get_live_list(None, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn get_stream_key() {
    let suite = UserHttpMock::new().await;
    suite.get_stream_key().await;

    let result = suite.client().get_stream_key().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn get_stream_settings() {
    let suite = UserHttpMock::new().await;
    suite.get_stream_settings().await;

    let result = suite.client().get_stream_settings().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn set_stream_settings() {
    let suite = UserHttpMock::new().await;
    suite.set_stream_settings().await;

    let result = suite.client().set_stream_settings().send().await;
    assert!(result.is_ok());
}
