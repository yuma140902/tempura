use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum StringMatcher {
    Regex {
        #[serde(with = "serde_regex")]
        regex: Regex,
    },
    Not {
        matcher: Box<StringMatcher>,
    },
    All {
        matchers: Vec<StringMatcher>,
    },
    Any {
        matchers: Vec<StringMatcher>,
    },
}

impl StringMatcher {
    pub fn is_match(&self, string: &str) -> bool {
        match self {
            Self::Regex { regex } => regex.is_match(string),
            Self::Not { matcher } => !matcher.is_match(string),
            Self::All { matchers } => matchers.iter().all(|matcher| matcher.is_match(string)),
            Self::Any { matchers } => matchers.iter().any(|matcher| matcher.is_match(string)),
        }
    }
}
