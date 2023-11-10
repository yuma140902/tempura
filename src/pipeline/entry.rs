use regex::Regex;
use serde::{Deserialize, Serialize};

use super::EnumLoader;

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    #[serde(rename = "match", with = "serde_regex")]
    pub match_regex: Regex,
    #[serde(rename = "with")]
    pub type_: EnumLoader,
}
