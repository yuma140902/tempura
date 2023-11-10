use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::transformer::TemplateRenderer;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Step {
    Load {
        path: PathBuf,
        key: String,
        with: EnumLoader,
    },
    Transform {
        input: String,
        output: String,
        with: EnumTransformer,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[serde(tag = "loader")]
pub enum EnumLoader {
    TextWithFrontmatter,
    Template,
    Json,
    Blob,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "transformer")]
pub enum EnumTransformer {
    TemplateRenderer(TemplateRenderer),
}
