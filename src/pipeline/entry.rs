use serde::{Deserialize, Serialize};

use crate::StringMatcher;

use super::EnumLoader;

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    #[serde(rename = "match")]
    pub match_regex: StringMatcher,
    #[serde(rename = "with")]
    pub type_: EnumLoader,
}
