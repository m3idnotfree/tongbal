mod builder;
mod response;
mod types;

pub use builder::{GetChannelFollower, GetChannelSubscriber};
pub use response::{ChannelResponse, FollowersResponse, ModeratorResponse, SubscribersResponse};
pub use types::{Channel, Follower, Moderator, Sort, Subscriber, Tier, UserRole};

use std::future::Future;

use crate::{
    APIResponse, AppClient, BaseClient, Client, Error,
    types::{
        ChannelId,
        constants::{CHANNELIDS, CHANNELS},
    },
};

pub trait UserChannelAPI: BaseClient {
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/channel#undefined-1>
    fn get_channel_moderators(
        &self,
    ) -> impl Future<Output = Result<APIResponse<ModeratorResponse>, Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/channel#undefined-2>
    fn get_channel_followers<'a>(&'a self) -> GetChannelFollower<'a>;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/channel#undefined-3>
    fn get_channel_subscribers<'a>(&'a self) -> GetChannelSubscriber<'a>;
}

pub trait AppChannelAPI: BaseClient {
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/channel#undefined>
    fn get_channels(
        &self,
        channel_ids: &[ChannelId],
    ) -> impl Future<Output = Result<APIResponse<ChannelResponse>, Error>> + Send;
}

impl AppChannelAPI for AppClient {
    async fn get_channels(
        &self,
        channel_ids: &[ChannelId],
    ) -> Result<APIResponse<ChannelResponse>, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(CHANNELS);

        url.query_pairs_mut()
            .extend_pairs(channel_ids.iter().map(|id| (CHANNELIDS, id)));

        crate::client::json(self.http_client().get(url)).await
    }
}

impl UserChannelAPI for Client {
    async fn get_channel_moderators(&self) -> Result<APIResponse<ModeratorResponse>, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNELS, "streaming-roles"]);

        crate::client::json(self.http_client().get(url)).await
    }

    fn get_channel_followers<'a>(&'a self) -> GetChannelFollower<'a> {
        GetChannelFollower::new(self)
    }

    fn get_channel_subscribers<'a>(&'a self) -> GetChannelSubscriber<'a> {
        GetChannelSubscriber::new(self)
    }
}
