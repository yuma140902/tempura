use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
    pub generator: Generator,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Generator {
    pub rules: Vec<GeneratorRule>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneratorRule {
    #[serde(rename = "match", with = "serde_regex")]
    pub match_: Regex,
    pub generator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            generator: Generator {
                rules: vec![GeneratorRule {
                    match_: Regex::new(".*[.]md").unwrap(),
                    generator: "handlebars".to_owned(),
                    export_base: None,
                    export_extension: Some("html".to_owned()),
                    template: Some("src/templates/page.html.hbs".to_owned()),
                }],
            },
        }
    }
}

impl Default for GeneratorRule {
    fn default() -> Self {
        Self {
            match_: Regex::new(".*").unwrap(),
            generator: "echo".to_owned(),
            export_base: None,
            export_extension: None,
            template: None,
        }
    }
}
