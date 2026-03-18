mod builder;
mod response;
mod types;

pub use builder::GetRestriction;
pub use response::RestrictionChannelResponse;
pub use types::RestrictionChannel;

use std::future::Future;

use types::RestrictionBody;

use crate::{
    BaseClient, Error, UserClient,
    types::{ChannelId, constants::RESTRICT_CHANNELS},
};

pub trait RestrictionAPI: BaseClient {
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/restriction#undefined>
    fn add_restraction(&self, target: &ChannelId)
    -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/restriction#undefined-1>
    fn remove_restriction(
        &self,
        target: &ChannelId,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/restriction#undefined-2>
    fn get_restrictions<'a>(&'a self) -> GetRestriction<'a>;
}

impl RestrictionAPI for UserClient {
    async fn add_restraction(&self, target: &ChannelId) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(RESTRICT_CHANNELS);

        self.http_client()
            .post(url)
            .json(&RestrictionBody {
                target_channel_id: target,
            })
            .send()
            .await?;

        Ok(())
    }

    async fn remove_restriction(&self, target: &ChannelId) -> Result<(), Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(RESTRICT_CHANNELS);

        self.http_client()
            .delete(url)
            .json(&RestrictionBody {
                target_channel_id: target,
            })
            .send()
            .await?;

        Ok(())
    }

    fn get_restrictions<'a>(&'a self) -> GetRestriction<'a> {
        GetRestriction::new(self)
    }
}
