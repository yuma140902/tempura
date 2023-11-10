//! This module defines a custom Helper for Tempura.
//! Helper is a term used in Handlebars to refer to a kind of function that can be called from a template.

use std::{collections::HashMap, path::PathBuf};

use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperResult, JsonRender, Output,
    RenderContext, RenderError,
};
use once_cell::sync::OnceCell;
use path_absolutize::Absolutize;
use tracing::debug;

use crate::directory;

handlebars_helper!(md2html: |markdown: String| {
    let options = pulldown_cmark::Options::empty();
    let parser = pulldown_cmark::Parser::new_ext(&markdown, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
});

/// - 入力: テンプレートの場所を基準とした相対パス
/// - 出力: 出力ディレクトリを基準とした相対パス
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
    let cwd = PathBuf::from(
        ctx.data()
            .get("___current_directory")
            .ok_or_else(|| RenderError::new("___current_directory not specified"))?
            .render()
            .as_str(),
    );
    let project_root = PROJECT_ROOT.get().unwrap();
    let table = RESOLVE_TABLE.get().unwrap();
    let output_directory = directory::get_output_directory(project_root);

    debug!("cwd = {}", cwd.display());
    let abs_cwd = cwd.absolutize_from(&project_root).map_err(|e| {
        RenderError::new(format!("failed to absolutize cwd {} : {e}", cwd.display()))
    })?;
    debug!("abs_cwd = {}", abs_cwd.display());
    let abs_input_path = target_input_path.absolutize_from(abs_cwd).map_err(|e| {
        RenderError::new(format!(
            "failed to absolutize input path {} : {e}",
            target_input_path.display()
        ))
    })?;
    debug!("abs_input_path = {}", abs_input_path.display());

    let abs_output_path = table.get(abs_input_path.as_ref()).ok_or_else(|| {
        RenderError::new(format!(
            "could not find path {} in table",
            abs_input_path.display()
        ))
    })?;
    debug!("abs_output_path = {}", abs_output_path.display());

    let rel_output_path = directory::get_relative_path(abs_output_path, &output_directory);
    debug!("rel_output_path = {}", rel_output_path.display());

    let output_str = rel_output_path.to_string_lossy().replace("\\", "/");
    write!(out, "{}", output_str)?;
    Ok(())
}

pub static PROJECT_ROOT: OnceCell<PathBuf> = OnceCell::new();
pub static RESOLVE_TABLE: OnceCell<HashMap<PathBuf, PathBuf>> = OnceCell::new();
