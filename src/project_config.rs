use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::pipeline::{Entry, EntryType, EnumLoader, EnumTransformer, Pipeline, Step};

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
                        match_regex: Regex::new(".*[.]md").unwrap(),
                        type_: EntryType::TextWithFrontmatter,
                    },
                    steps: vec![
                        Step::Load {
                            path: "src/templates/default.html.hbs".into(),
                            name: "default_template".to_string(),
                            loader: EnumLoader::Template,
                        },
                        Step::Transform {
                            transformer: EnumTransformer::TemplateRenderer {
                                template_name: "default".to_string(),
                            },
                        },
                    ],
                    output_extension: Some("html".to_string()),
                },
                Pipeline {
                    entry: Entry {
                        match_regex: Regex::new(".*").unwrap(),
                        type_: EntryType::Blob,
                    },
                    steps: vec![],
                    output_extension: None,
                },
            ],
            output_base_directory: "public".to_string(),
        }
    }
}
