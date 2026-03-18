use serde::Serialize;

use crate::{
    BaseClient, Error, UserClient,
    types::{
        MessageId,
        constants::{CHATS, NOTICE, SETTINGS},
    },
};

use super::{ChatCondition, ChatGroup, ChatSlowModeSec, MinFollowerMinute, NoticeBody};

#[derive(Debug)]
pub struct SetChatNotice<'a> {
    client: &'a UserClient,
    message: Option<&'a str>,
    message_id: Option<&'a MessageId>,
}

impl<'a> SetChatNotice<'a> {
    pub(crate) fn new(client: &'a UserClient) -> Self {
        Self {
            client,
            message: None,
            message_id: None,
        }
    }

    pub fn message(mut self, value: &'a str) -> Self {
        self.message = Some(value);
        self
    }

    pub fn message_id(mut self, value: &'a MessageId) -> Self {
        self.message_id = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();
        url.path_segments_mut().unwrap().extend([CHATS, NOTICE]);
        crate::client::no_content(self.client.http_client().post(url).json(&NoticeBody {
            message: self.message,
            message_id: self.message_id,
        }))
        .await?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetChatSettings<'a> {
    #[serde(skip)]
    client: &'a UserClient,
    chat_available_condition: Option<ChatCondition>,
    chat_available_group: Option<ChatGroup>,
    min_follower_minute: Option<MinFollowerMinute>,
    allow_subscriber_in_follower_mode: Option<bool>,
    chat_slow_mode_sec: Option<ChatSlowModeSec>,
    chat_emoji_mode: Option<bool>,
}

impl<'a> SetChatSettings<'a> {
    pub(crate) fn new(client: &'a UserClient) -> Self {
        Self {
            client,
            chat_available_condition: None,
            chat_available_group: None,
            min_follower_minute: None,
            allow_subscriber_in_follower_mode: None,
            chat_slow_mode_sec: None,
            chat_emoji_mode: None,
        }
    }

    pub fn chat_available_condition(mut self, value: ChatCondition) -> Self {
        self.chat_available_condition = Some(value);
        self
    }

    pub fn chat_available_group(mut self, value: ChatGroup) -> Self {
        self.chat_available_group = Some(value);
        self
    }

    pub fn min_follower_minute(mut self, value: MinFollowerMinute) -> Self {
        self.min_follower_minute = Some(value);
        self
    }

    pub fn allow_subscriber_in_follower_mode(mut self, value: bool) -> Self {
        self.allow_subscriber_in_follower_mode = Some(value);
        self
    }

    pub fn chat_slow_mode_sec(mut self, value: ChatSlowModeSec) -> Self {
        self.chat_slow_mode_sec = Some(value);
        self
    }

    pub fn chat_emoji_mode(mut self, value: bool) -> Self {
        self.chat_emoji_mode = Some(value);
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();
        url.path_segments_mut().unwrap().extend([CHATS, SETTINGS]);
        crate::client::no_content(self.client.http_client().put(url).json(&self)).await?;
        Ok(())
    }
}
