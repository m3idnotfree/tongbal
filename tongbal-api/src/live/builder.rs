use serde::Serialize;

use crate::{
    BaseClient, Client, Error,
    types::{
        CategoryType,
        constants::{LIVES, SETTING},
    },
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetStreamSetting<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_live_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<CategoryType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
}

impl<'a> SetStreamSetting<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            client,
            default_live_title: None,
            category: None,
            category_id: None,
            tags: Vec::new(),
        }
    }

    pub fn default_live_title(mut self, title: impl Into<String>) -> Self {
        self.default_live_title = Some(title.into());
        self
    }

    pub fn category(mut self, category: CategoryType) -> Self {
        self.category = Some(category);
        self
    }

    pub fn category_id(mut self, category_id: impl Into<String>) -> Self {
        self.category_id = Some(category_id.into());
        self
    }

    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn add_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags.extend(tags.into_iter().map(|t| t.into()));
        self
    }

    pub async fn send(self) -> Result<(), Error> {
        let mut url = self.client.base_url();
        url.path_segments_mut().unwrap().extend([LIVES, SETTING]);
        crate::client::no_content(self.client.http_client().patch(url).json(&self)).await?;
        Ok(())
    }
}
