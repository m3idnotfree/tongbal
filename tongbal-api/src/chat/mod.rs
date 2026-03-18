mod builder;
mod response;
mod types;

pub use builder::{SetChatNotice, SetChatSettings};
pub use response::{ChatSettingResponse, MessageResponse};
pub use types::{ChatCondition, ChatGroup, ChatSlowModeSec, MinFollowerMinute};

use std::future::Future;

use types::{MessageBody, NoticeBody};

use crate::{
    APIResponse, BaseClient, Error, UserClient,
    types::constants::{CHATS, SEND, SETTINGS},
};

pub trait ChatAPI: BaseClient {
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/chat#undefined>
    fn send_message(
        &self,
        message: &str,
    ) -> impl Future<Output = Result<APIResponse<MessageResponse>, Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/chat#undefined-1>
    fn set_chat_notice<'a>(&'a self) -> SetChatNotice<'a>;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/chat#undefined-2>
    fn get_chat_settings(
        &self,
    ) -> impl Future<Output = Result<APIResponse<ChatSettingResponse>, Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/chat#undefined-3>
    fn set_chat_settings<'a>(&'a self) -> SetChatSettings<'a>;
}

impl ChatAPI for UserClient {
    async fn send_message(&self, message: &str) -> Result<APIResponse<MessageResponse>, Error> {
        let mut url = self.base_url();
        url.path_segments_mut().unwrap().extend([CHATS, SEND]);
        crate::client::json(self.http_client().post(url).json(&MessageBody { message })).await
    }

    fn set_chat_notice<'a>(&'a self) -> SetChatNotice<'a> {
        SetChatNotice::new(self)
    }

    async fn get_chat_settings(&self) -> Result<APIResponse<ChatSettingResponse>, Error> {
        let mut url = self.base_url();
        url.path_segments_mut().unwrap().extend([CHATS, SETTINGS]);
        crate::client::json(self.http_client().get(url)).await
    }

    fn set_chat_settings<'a>(&'a self) -> SetChatSettings<'a> {
        SetChatSettings::new(self)
    }
}
