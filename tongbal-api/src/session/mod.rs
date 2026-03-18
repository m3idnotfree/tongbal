mod builder;
mod response;
mod types;

pub use builder::GetSession;
pub use response::{CreateSessionResponse, SessionResponse};
pub use types::{EventType, Session, SubscribedEvent};

use std::future::Future;

use crate::{
    APIResponse, AppClient, BaseClient, Error, UserClient,
    types::{
        SessionKey,
        constants::{
            AUTH, CHAT, CLIENT, DONATION, EVENTS, SESSION_KEY, SESSIONS, SUBSCRIBE, SUBSCRIPTION,
            UNSUBSCRIBE,
        },
    },
};

pub trait SessionAPI: BaseClient {
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/session#undefined>
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/session#undefined-1>
    fn create_session(
        &self,
    ) -> impl Future<Output = Result<APIResponse<CreateSessionResponse>, Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/session#undefined-3>
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/session#undefined-4>
    fn get_sessions<'a>(&'a self) -> GetSession<'a, Self>
    where
        Self: Sized;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/session#undefined-5>
    fn subscribe_chat(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/session#undefined-6>
    fn unsubscribe_chat(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/session#undefined-7>
    fn subscribe_donation(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/session#undefined-8>
    fn unsubscribe_donation(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/session#undefined-9>
    fn subscribe_subscription(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/session#undefined-10>
    fn unsubscribe_subscription(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

impl SessionAPI for AppClient {
    async fn create_session(&self) -> Result<APIResponse<CreateSessionResponse>, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, AUTH, CLIENT]);

        crate::client::json(self.http_client().get(url)).await
    }

    fn get_sessions<'a>(&'a self) -> GetSession<'a, Self> {
        GetSession::new(self, Some(CLIENT))
    }

    async fn subscribe_chat(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, EVENTS, SUBSCRIBE, CHAT]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }

    async fn unsubscribe_chat(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, EVENTS, UNSUBSCRIBE, CHAT]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }

    async fn subscribe_donation(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, EVENTS, SUBSCRIBE, DONATION]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }

    async fn unsubscribe_donation(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, EVENTS, UNSUBSCRIBE, DONATION]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }

    async fn subscribe_subscription(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, EVENTS, SUBSCRIBE, SUBSCRIPTION]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }

    async fn unsubscribe_subscription(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[SESSIONS, EVENTS, UNSUBSCRIBE, SUBSCRIPTION]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }
}

impl SessionAPI for UserClient {
    async fn create_session(&self) -> Result<APIResponse<CreateSessionResponse>, Error> {
        let mut url = self.base_url();
        url.path_segments_mut().unwrap().extend([SESSIONS, AUTH]);
        crate::client::json(self.http_client().get(url)).await
    }

    fn get_sessions<'a>(&'a self) -> GetSession<'a, Self> {
        GetSession::new(self, None)
    }

    async fn subscribe_chat(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, EVENTS, SUBSCRIBE, CHAT]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }

    async fn unsubscribe_chat(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, EVENTS, UNSUBSCRIBE, CHAT]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }

    async fn subscribe_donation(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, EVENTS, SUBSCRIBE, DONATION]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }

    async fn unsubscribe_donation(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, EVENTS, UNSUBSCRIBE, DONATION]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }

    async fn subscribe_subscription(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([SESSIONS, EVENTS, SUBSCRIBE, SUBSCRIPTION]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }

    async fn unsubscribe_subscription(&self, session_key: &SessionKey) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend(&[SESSIONS, EVENTS, UNSUBSCRIBE, SUBSCRIPTION]);

        url.query_pairs_mut()
            .append_pair(SESSION_KEY, session_key.as_ref());

        crate::client::no_content(self.http_client().post(url)).await?;
        Ok(())
    }
}
