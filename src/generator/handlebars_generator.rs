use std::{borrow::Borrow, fs, path::Path};

use anyhow::Context as _;
use handlebars::Handlebars;
use path_absolutize::Absolutize;
use serde_json::{Map, Value as Json};
use serde_yaml::Value as Yaml;
use tracing::info;
use walkdir::WalkDir;

use crate::{directory, handlebars_helpers, project_config::GeneratorRule};

use super::{Generator, GeneratorResult};

fn setup_handlebars(project_root: impl AsRef<Path>) -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("md2html", Box::new(handlebars_helpers::md2html));
    handlebars.register_helper("resolve", Box::new(handlebars_helpers::resolve));

    // enumerate template files
    for path in WalkDir::new(directory::get_templates_directory(project_root))
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| p.is_file() && p.extension().map(|ext| ext == "hbs").unwrap_or(false))
    {
        let abs_path = path
            .absolutize()
            .with_context(|| format!("could not absolutize path '{}'", path.display()))
            .unwrap();
        handlebars
            .register_template_file(abs_path.to_string_lossy().borrow(), &abs_path)
            .with_context(|| format!("could not register template '{}'", abs_path.display()))
            .unwrap();
        info!("registered template: {}", abs_path.display());
    }

    handlebars
}

fn split_frontmatter(text: &str) -> (Option<String>, String) {
    if let Some((front, body)) = matter::matter(text) {
        (Some(front), body)
    } else {
        (None, text.to_owned())
    }
}

fn make_data(
    yaml: &Yaml,
    content: String,
    input_filepath: impl AsRef<Path>,
    output_filepath: impl AsRef<Path>,
    project_root: impl AsRef<Path>,
) -> Map<String, Json> {
    let mut data = Map::new();
    data.insert("content".to_owned(), handlebars::to_json(content));
    data.insert("front".to_owned(), handlebars::to_json(yaml));
    data.insert(
        "input_file".to_owned(),
        handlebars::to_json(input_filepath.as_ref()),
    );
    data.insert(
        "output_file".to_owned(),
        handlebars::to_json(output_filepath.as_ref()),
    );
    data.insert(
        "project_root".to_owned(),
        handlebars::to_json(project_root.as_ref()),
    );
    data
}

#[derive(Debug)]
pub struct HandlebarsGenerator {
    handlebars: Handlebars<'static>,
}

impl HandlebarsGenerator {
    pub fn new(project_root: impl AsRef<Path>) -> Self {
        Self {
            handlebars: setup_handlebars(&project_root),
        }
    }
}

impl Generator for HandlebarsGenerator {
    fn generate(
        &self,
        input_filepath: &Path,
        output_filepath: &Path,
        project_root: &Path,
        rule: &GeneratorRule,
    ) -> GeneratorResult {
        let content =
            fs::read_to_string(input_filepath).context(format!("Reading {:?}", input_filepath))?;
        let (maybe_yaml, content) = split_frontmatter(&content);
        let yaml = match maybe_yaml {
            Some(yaml) => serde_yaml::from_str(&yaml)
                .context(format!("Invalid YAML in {:?}", input_filepath))?,
            None => serde_yaml::Value::default(),
        };

        let data = make_data(
            &yaml,
            content,
            input_filepath,
            output_filepath,
            project_root,
        );

        let abs_project_root = project_root.absolutize().with_context(|| {
            format!(
                "could not absolutize project_root '{}'",
                project_root.display()
            )
        })?;

        let html_output = self
            .handlebars
            .render(
                Path::new(
                    rule.template
                        .as_deref()
                        .unwrap_or("src/templates/page.html.hbs"),
                )
                .absolutize_from(abs_project_root.as_ref())
                .with_context(|| format!("could not absolutize template '{:?}'", rule.template))?
                .to_string_lossy()
                .borrow(),
                &data,
            )
            .context(format!("Input file: {:?}", input_filepath))?;

        if let Some(parent) = output_filepath.parent() {
            fs::create_dir_all(parent).context(format!("Create directory {:?}", parent))?;
        }
        fs::write(output_filepath, html_output).context(format!("Write {:?}", output_filepath))?;

        Ok(())
    }
}
