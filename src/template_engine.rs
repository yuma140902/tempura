use handlebars::Handlebars;
use tracing::{debug, error, warn};

use crate::handlebars_helpers;

/// TemplateEngine is a wrapper for Handlebars and processes templates and [`Value`](crate::Value)s.
pub struct TemplateEngine(Handlebars<'static>);

impl TemplateEngine {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();

        handlebars.register_helper("md2html", Box::new(handlebars_helpers::md2html));
        handlebars.register_helper("resolve", Box::new(handlebars_helpers::resolve));

        Self(handlebars)
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEngine {
    pub fn register_template_from_string(
        &mut self,
        name: &str,
        string: String,
    ) -> Result<(), Box<handlebars::TemplateError>> {
        self.0
            .register_template_string(name, string)
            .map_err(Box::new)
    }

    pub fn register_template(&mut self, name: &str, tpl: handlebars::Template) {
        self.0.register_template(name, tpl)
    }

    pub fn render(
        &self,
        template_name: &str,
        current_directory: &Option<String>,
        value: &serde_json::Value,
    ) -> Result<String, handlebars::RenderError> {
        if let Some(current_directory) = current_directory {
            if let serde_json::Value::Object(inner) = value {
                let mut map = serde_json::Map::new();
                map.extend(inner.clone());
                debug!("current_directory = {}", current_directory);
                map.insert(
                    "___current_directory".to_string(),
                    serde_json::Value::String(current_directory.to_string()),
                );
                let value = serde_json::Value::Object(map);
                return self.0.render(template_name, &value);
            } else {
                error!("value is not object. `resolve` helper will not work in this template");
            }
        } else {
            warn!("current_directory is not set. `resolve` helper will not work in this template");
        }
        self.0.render(template_name, value)
    }
}
