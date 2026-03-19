mod builder;
mod response;
mod types;

pub use builder::SetStreamSetting;
pub use response::{LiveListResponse, StreamKey, StreamSetting};
pub use types::Live;

use std::future::Future;

use crate::{
    AppClient, BaseClient, Error, Response, UserClient,
    types::constants::{LIVES, SETTING, SIZE, STREAMS},
};

pub trait AppLiveAPI: BaseClient {
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/live#undefined>
    fn get_live_list(
        &self,
        size: Option<u64>,
        next: Option<String>,
    ) -> impl Future<Output = Result<Response<LiveListResponse>, Error>> + Send;
}

pub trait UserLiveAPI: BaseClient {
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/live#undefined-1>
    fn get_stream_key(&self) -> impl Future<Output = Result<Response<StreamKey>, Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/live#undefined-2>
    fn get_stream_settings(
        &self,
    ) -> impl Future<Output = Result<Response<StreamSetting>, Error>> + Send;

    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/live#undefined-3>
    fn set_stream_settings<'a>(&'a self) -> SetStreamSetting<'a>;
}

impl AppLiveAPI for AppClient {
    async fn get_live_list(
        &self,
        size: Option<u64>,
        next: Option<String>,
    ) -> Result<Response<LiveListResponse>, Error> {
        let mut url = self.base_url();

        url.path_segments_mut().unwrap().push(LIVES);

        if let Some(size) = size {
            url.query_pairs_mut().append_pair(SIZE, &size.to_string());
        }
        if let Some(next) = next {
            url.query_pairs_mut().append_pair("next", &next);
        }

        crate::client::json(self.http_client().get(url)).await
    }
}

impl UserLiveAPI for UserClient {
    async fn get_stream_key(&self) -> Result<Response<StreamKey>, Error> {
        let mut url = self.base_url();
        url.path_segments_mut().unwrap().extend([STREAMS, "key"]);
        crate::client::json(self.http_client().get(url)).await
    }

    async fn get_stream_settings(&self) -> Result<Response<StreamSetting>, Error> {
        let mut url = self.base_url();
        url.path_segments_mut().unwrap().extend([LIVES, SETTING]);
        crate::client::json(self.http_client().get(url)).await
    }

    fn set_stream_settings<'a>(&'a self) -> SetStreamSetting<'a> {
        SetStreamSetting::new(self)
    }
}
