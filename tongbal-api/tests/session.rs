mod common;

mod user {
    use tongbal_api::{session::SessionAPI, types::SessionKey};

    use super::common::UserHttpMock;

    #[tokio::test]
    async fn create_session() {
        let suite = UserHttpMock::new().await;
        suite.create_session().await;

        let result = suite.client().create_session().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_sessions() {
        let suite = UserHttpMock::new().await;
        suite.get_sessions().await;

        let result = suite.client().get_sessions().send().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn subscribe_chat() {
        let suite = UserHttpMock::new().await;
        suite.subscribe_chat().await;

        let result = suite
            .client()
            .subscribe_chat(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn unsubscribe_chat() {
        let suite = UserHttpMock::new().await;
        suite.unsubscribe_chat().await;

        let result = suite
            .client()
            .unsubscribe_chat(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn subscribe_donation() {
        let suite = UserHttpMock::new().await;
        suite.subscribe_donation().await;

        let result = suite
            .client()
            .subscribe_donation(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn unsubscribe_donation() {
        let suite = UserHttpMock::new().await;
        suite.unsubscribe_donation().await;

        let result = suite
            .client()
            .unsubscribe_donation(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn subscribe_subscription() {
        let suite = UserHttpMock::new().await;
        suite.subscribe_subscription().await;

        let result = suite
            .client()
            .subscribe_subscription(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn unsubscribe_subscription() {
        let suite = UserHttpMock::new().await;
        suite.unsubscribe_subscription().await;

        let result = suite
            .client()
            .unsubscribe_subscription(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }
}

mod app {
    use tongbal_api::{session::SessionAPI, types::SessionKey};

    use super::common::AppHttpMock;
    #[tokio::test]
    async fn create_session() {
        let suite = AppHttpMock::new().await;
        suite.create_session().await;

        let result = suite.client().create_session().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_sessions() {
        let suite = AppHttpMock::new().await;
        suite.get_sessions().await;

        let result = suite.client().get_sessions().send().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn subscribe_chat() {
        let suite = AppHttpMock::new().await;
        suite.subscribe_chat().await;

        let result = suite
            .client()
            .subscribe_chat(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn unsubscribe_chat() {
        let suite = AppHttpMock::new().await;
        suite.unsubscribe_chat().await;

        let result = suite
            .client()
            .unsubscribe_chat(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn subscribe_donation() {
        let suite = AppHttpMock::new().await;
        suite.subscribe_donation().await;

        let result = suite
            .client()
            .subscribe_donation(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn unsubscribe_donation() {
        let suite = AppHttpMock::new().await;
        suite.unsubscribe_donation().await;

        let result = suite
            .client()
            .unsubscribe_donation(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn subscribe_subscription() {
        let suite = AppHttpMock::new().await;
        suite.subscribe_subscription().await;

        let result = suite
            .client()
            .subscribe_subscription(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn unsubscribe_subscription() {
        let suite = AppHttpMock::new().await;
        suite.unsubscribe_subscription().await;

        let result = suite
            .client()
            .unsubscribe_subscription(&SessionKey::from("fefb6bbb-00c2-497c-afc2-XXXXXXXXXXXX"))
            .await;
        assert!(result.is_ok());
    }
}
