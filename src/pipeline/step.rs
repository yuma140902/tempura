use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Step {
    Load {
        path: PathBuf,
        name: String,
        loader: EnumLoader,
    },
    Transform {
        input: String,
        output: String,
        transformer: EnumTransformer,
    },
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[serde(tag = "type")]
pub enum EnumLoader {
    Template,
    Json,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum EnumTransformer {
    TemplateRenderer { template_name: String },
}
