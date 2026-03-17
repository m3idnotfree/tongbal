use crate::{
    APIResponse, BaseClient, Client, Error,
    types::constants::{NEXT, RESTRICT_CHANNELS, SIZE},
};

use super::RestrictionChannelResponse;

#[derive(Debug)]
pub struct GetRestriction<'a> {
    client: &'a Client,
    size: Option<u64>,
    next: Option<String>,
}

impl<'a> GetRestriction<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            size: None,
            next: None,
        }
    }

    pub fn size(mut self, value: u64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn next(mut self, value: impl Into<String>) -> Self {
        self.next = Some(value.into());
        self
    }

    pub async fn send(self) -> Result<APIResponse<RestrictionChannelResponse>, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut().unwrap().push(RESTRICT_CHANNELS);

        if let Some(size) = self.size {
            url.query_pairs_mut().append_pair(SIZE, &size.to_string());
        }
        if let Some(next) = self.next {
            url.query_pairs_mut().append_pair(NEXT, &next);
        }

        crate::client::json(self.client.http_client().get(url)).await
    }
}
