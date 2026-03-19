use serde::{Deserialize, Serialize};
use url::Url;

use crate::new_types::CategoryId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub category_type: CategoryType,
    pub category_id: CategoryId,
    pub category_value: String,
    pub poster_image_url: Option<Url>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CategoryType {
    Game,
    Sports,
    Etc,
}
