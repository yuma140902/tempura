//! This module defines a custom Helper for Tempura.
//! Helper is a term used in Handlebars to refer to a kind of function that can be called from a template.

use std::path::PathBuf;

use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperResult, JsonRender, Output,
    RenderContext, RenderError,
};

use crate::directory;

handlebars_helper!(md2html: |markdown: String| {
    let options = pulldown_cmark::Options::empty();
    let parser = pulldown_cmark::Parser::new_ext(&markdown, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
});

// TODO:
pub fn resolve(
    h: &Helper<'_, '_>,
    _: &Handlebars<'_>,
    ctx: &Context,
    _rc: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    let target_input_path = PathBuf::from(
        h.param(0)
            .and_then(|v| v.value().as_str())
            .ok_or_else(|| RenderError::new("target_input_path not specified"))?,
    );
    let project_root = PathBuf::from(
        ctx.data()
            .get("project_root")
            .ok_or_else(|| RenderError::new("project_root not specified"))?
            .render()
            .as_str(),
    );
    let output_directory = directory::get_output_directory(project_root);
    let self_output_filepath = PathBuf::from(
        ctx.data()
            .get("output_file")
            .ok_or_else(|| RenderError::new("output_file not specified"))?
            .render()
            .as_str(),
    );
    let self_output_parent = self_output_filepath.parent().ok_or_else(|| {
        RenderError::new(format!(
            "could not get parent directory for output_file '{}'",
            self_output_filepath.display()
        ))
    })?;

    let target_output_path = output_directory.join(target_input_path); //TODO:
                                                                       //rule.export_extension
                                                                       //rule.export_base
    let relative_path = directory::get_relative_path(target_output_path, self_output_parent);
    write!(out, "{}", relative_path.display())?;
    Ok(())
}
