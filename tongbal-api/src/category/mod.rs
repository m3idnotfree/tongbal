mod response;

pub use response::CategoryResponse;

use std::future::Future;

use crate::{
    APIResponse, AppClient, BaseClient, Error,
    types::constants::{CATEGORIES, QUERY, SEARCH, SIZE},
};

pub trait CategoryAPI: BaseClient {
    /// See <https://chzzk.gitbook.io/chzzk/chzzk-api/category>
    fn search_category(
        &self,
        query: &str,
        size: Option<usize>,
    ) -> impl Future<Output = Result<APIResponse<CategoryResponse>, Error>> + Send;
}

impl CategoryAPI for AppClient {
    async fn search_category(
        &self,
        query: &str,
        size: Option<usize>,
    ) -> Result<APIResponse<CategoryResponse>, Error> {
        let mut url = self.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CATEGORIES, SEARCH]);

        url.query_pairs_mut().append_pair(QUERY, query);
        if let Some(size) = size {
            url.query_pairs_mut().append_pair(SIZE, &size.to_string());
        }

        crate::client::json(self.http_client().get(url)).await
    }
}
