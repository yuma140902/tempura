use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    #[serde(rename = "match", with = "serde_regex")]
    pub match_regex: Regex,
    #[serde(rename = "type")]
    pub type_: EntryType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum EntryType {
    TextWithFrontmatter,
    Json,
    Blob,
}
