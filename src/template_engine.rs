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
