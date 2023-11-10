use handlebars::Handlebars;

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
    ) -> Result<(), handlebars::TemplateError> {
        self.0.register_template_string(name, string)
    }

    pub fn register_template(&mut self, name: &str, tpl: handlebars::Template) {
        self.0.register_template(name, tpl)
    }

    pub fn render(
        &self,
        template_name: &str,
        value: &serde_json::Value,
    ) -> Result<String, handlebars::RenderError> {
        self.0.render(template_name, value)
    }
}
