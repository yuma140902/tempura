use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::transformer::{JsonPathQuery, JsonPathQueryAll, Reloader, TemplateRenderer};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Step {
    Load {
        path: PathBuf,
        key: String,
        with: EnumLoader,
    },
    Transform {
        input: InputKey,
        output: String,
        with: EnumTransformer,
    },
    DumpStore,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[serde(tag = "loader")]
pub enum EnumLoader {
    TextWithFrontmatter,
    Text,
    Template,
    Json,
    Blob,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "transformer")]
pub enum EnumTransformer {
    TemplateRenderer(TemplateRenderer),
    JsonPath(JsonPathQuery),
    JsonPathAll(JsonPathQueryAll),
    Reloader(Reloader),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InputKey {
    Single(String),
    List(Vec<String>),
    Map(HashMap<String, String>),
}
