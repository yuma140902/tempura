use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::pipeline::{Entry, EntryType, LoaderType, Pipeline, Step, TransformerType};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub pipelines: Vec<Pipeline>,
    pub output_base_directory: String,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            pipelines: vec![
                Pipeline {
                    entry: Entry {
                        match_regex: Regex::new(".*").unwrap(),
                        type_: EntryType::Blob,
                    },
                    steps: vec![],
                    output_extension: None,
                },
                Pipeline {
                    entry: Entry {
                        match_regex: Regex::new(".*[.]md").unwrap(),
                        type_: EntryType::TextWithFrontmatter,
                    },
                    steps: vec![
                        Step::Load {
                            path: "src/templates/default.html.hbs".into(),
                            loader: LoaderType::Template,
                        },
                        Step::Transform {
                            transformer: TransformerType::TemplateRenderer,
                        },
                    ],
                    output_extension: Some("html".to_string()),
                },
            ],
            output_base_directory: "public".to_string(),
        }
    }
}
