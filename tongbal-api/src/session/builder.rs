use crate::{
    BaseClient, Error, Response,
    types::constants::{PAGE, SESSIONS, SIZE},
};

use super::SessionResponse;

#[derive(Debug)]
pub struct GetSession<'a, T> {
    client: &'a T,
    client_path: Option<&'static str>,
    size: Option<u64>,
    page: Option<&'a str>,
}

impl<'a, T> GetSession<'a, T>
where
    T: BaseClient,
{
    pub(crate) fn new(client: &'a T, client_path: Option<&'static str>) -> Self {
        Self {
            client,
            client_path,
            size: None,
            page: None,
        }
    }

    pub fn size(mut self, value: u64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn page(mut self, value: &'a str) -> Self {
        self.page = Some(value);
        self
    }

    pub async fn send(self) -> Result<Response<SessionResponse>, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(SESSIONS);
        if let Some(path) = self.client_path {
            url.path_segments_mut().unwrap().push(path);
        }

        if let Some(size) = self.size {
            url.query_pairs_mut().append_pair(SIZE, &size.to_string());
        }
        if let Some(page) = self.page {
            url.query_pairs_mut().append_pair(PAGE, page);
        }

        crate::client::json(self.client.http_client().get(url)).await
    }
}
