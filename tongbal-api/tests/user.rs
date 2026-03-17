mod common;

use common::UserHttpMock;
use tongbal_api::user::UserAPI;

#[tokio::test]
async fn get_me() {
    let suite = UserHttpMock::new().await;
    suite.get_me().await;

    let result = suite.client().get_me().await;
    assert!(result.is_ok());
}
