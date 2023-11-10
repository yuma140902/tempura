use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    pipeline::{Entry, EnumLoader, EnumTransformer, InputKey, Pipeline, Step},
    transformer::TemplateRenderer,
};

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
                    name: "markdown to html".to_string(),
                    entry: Entry {
                        match_regex: Regex::new(".*[.]md").unwrap(),
                        type_: EnumLoader::TextWithFrontmatter,
                    },
                    steps: vec![
                        Step::Load {
                            path: "src/templates/default.html.hbs".into(),
                            key: "default_template".to_string(),
                            with: EnumLoader::Template,
                        },
                        Step::Transform {
                            input: InputKey::Single("entry".to_string()),
                            output: "template_result".to_string(),
                            with: EnumTransformer::TemplateRenderer(TemplateRenderer {
                                template_key: "default".to_string(),
                            }),
                        },
                    ],
                    output_extension: Some("html".to_string()),
                    output_key: "template_result".to_string(),
                },
                Pipeline {
                    name: "static resources".to_string(),
                    entry: Entry {
                        match_regex: Regex::new(".*").unwrap(),
                        type_: EnumLoader::Blob,
                    },
                    steps: vec![],
                    output_extension: None,
                    output_key: "entry".to_string(),
                },
            ],
            output_base_directory: "public".to_string(),
        }
    }
}
