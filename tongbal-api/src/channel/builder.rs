use crate::{
    APIResponse, BaseClient, Client, Error,
    types::constants::{CHANNELS, FOLLOWERS, PAGE, SIZE, SORT, SUBSCRIBERS},
};

use super::{FollowersResponse, Sort, SubscribersResponse};

pub struct GetChannelFollower<'a> {
    client: &'a Client,
    page: Option<u64>,
    size: Option<u64>,
}

impl<'a> GetChannelFollower<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            page: None,
            size: None,
        }
    }

    pub fn page(mut self, value: u64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn size(mut self, value: u64) -> Self {
        self.size = Some(value);
        self
    }

    pub async fn send(self) -> Result<APIResponse<FollowersResponse>, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNELS, FOLLOWERS]);

        if let Some(page) = self.page {
            url.query_pairs_mut().append_pair(PAGE, &page.to_string());
        }
        if let Some(size) = self.size {
            url.query_pairs_mut().append_pair(SIZE, &size.to_string());
        }

        crate::client::json(self.client.http_client().get(url)).await
    }
}

pub struct GetChannelSubscriber<'a> {
    client: &'a Client,
    page: Option<u64>,
    size: Option<u64>,
    sort: Option<Sort>,
}

impl<'a> GetChannelSubscriber<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            page: None,
            size: None,
            sort: None,
        }
    }

    pub fn page(mut self, value: u64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn size(mut self, value: u64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn sort(mut self, value: Sort) -> Self {
        self.sort = Some(value);
        self
    }

    pub async fn send(self) -> Result<APIResponse<SubscribersResponse>, Error> {
        let mut url = self.client.base_url();

        url.path_segments_mut()
            .unwrap()
            .extend([CHANNELS, SUBSCRIBERS]);

        if let Some(page) = self.page {
            url.query_pairs_mut().append_pair(PAGE, &page.to_string());
        }
        if let Some(size) = self.size {
            url.query_pairs_mut().append_pair(SIZE, &size.to_string());
        }
        if let Some(sort) = self.sort {
            url.query_pairs_mut().append_pair(SORT, sort.as_str());
        }

        crate::client::json(self.client.http_client().get(url)).await
    }
}
