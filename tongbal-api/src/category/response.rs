use serde::{Deserialize, Serialize};

use crate::types::Category;

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryResponse {
    pub data: Vec<Category>,
}
