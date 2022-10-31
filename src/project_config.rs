use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub generator: Generator,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Generator {
    pub rules: Vec<GeneratorRule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneratorRule {
    #[serde(rename = "match")]
    pub match_: String,
    pub generator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_extension: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            generator: Generator {
                rules: vec![GeneratorRule {
                    match_: ".*[.]md".to_owned(),
                    generator: "handlebars".to_owned(),
                    export_base: None,
                    export_extension: Some("html".to_owned()),
                }],
            },
        }
    }
}
