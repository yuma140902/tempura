use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Step {
    Load { path: PathBuf, loader: EnumLoader },
    Transform { transformer: EnumTransformer },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum EnumLoader {
    Template { template_name: String },
    Json { store_as: String },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum EnumTransformer {
    TemplateRenderer { template_name: String },
}
