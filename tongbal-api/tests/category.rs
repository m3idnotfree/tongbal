mod common;

use common::AppHttpMock;
use tongbal_api::category::CategoryAPI;

#[tokio::test]
async fn search_category() {
    let suite = AppHttpMock::new().await;
    suite.search_category().await;

    let result = suite.client().search_category("all", Some(5)).await;

    assert!(result.is_ok());
}
