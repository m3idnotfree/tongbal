mod common;

use common::UserHttpMock;
use tongbal_api::chat::ChatAPI;

#[tokio::test]
async fn send_message() {
    let suite = UserHttpMock::new().await;
    suite.send_message().await;

    let result = suite.client().send_message("hi").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn set_chat_notice() {
    let suite = UserHttpMock::new().await;
    suite.set_chat_notice().await;

    let result = suite.client().set_chat_notice().send().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn get_chat_settings() {
    let suite = UserHttpMock::new().await;
    suite.get_chat_settings().await;

    let result = suite.client().get_chat_settings().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn set_chat_settings() {
    let suite = UserHttpMock::new().await;
    suite.set_chat_settings().await;

    let result = suite.client().set_chat_settings().send().await;

    assert!(result.is_ok());
}
