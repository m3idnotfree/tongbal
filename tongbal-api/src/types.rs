use serde::{Deserialize, Serialize};

pub(crate) mod constants {
    pub const CATEGORIES: &str = "categories";
    pub const SEARCH: &str = "search";
    pub const QUERY: &str = "query";
    pub const SIZE: &str = "size";
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub category_type: CategoryType,
    pub category_id: String,
    pub category_value: String,
    pub poster_image_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CategoryType {
    Game,
    Sports,
    Etc,
}
