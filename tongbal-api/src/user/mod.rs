mod types;

pub use types::User;

use std::future::Future;

use crate::{
    APIResponse, BaseClient, Client, Error,
    types::constants::{ME, USERS},
};

pub trait UserAPI: BaseClient {
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/user>
    fn get_me(&self) -> impl Future<Output = Result<APIResponse<User>, Error>> + Send;
}

impl UserAPI for Client {
    async fn get_me(&self) -> Result<APIResponse<User>, Error> {
        let mut url = self.base_url();
        url.path_segments_mut().unwrap().extend([USERS, ME]);
        crate::client::json(self.http_client().get(url)).await
    }
}
