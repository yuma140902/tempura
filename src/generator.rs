use std::fmt::Debug;
use std::{collections::HashMap, path::Path};

use echo_generator::EchoGenerator;
use handlebars_generator::HandlebarsGenerator;

use crate::project_config::GeneratorRule;

pub trait Generator: Debug {
    fn generate(
        &self,
        input_filepath: &Path,
        output_filepath: &Path,
        project_root: &Path,
        rule: &GeneratorRule,
    ) -> GeneratorResult;
}

pub enum GeneratorResult {
    Success,
    Fail,
}

pub fn get_generators(project_root: impl AsRef<Path>) -> HashMap<String, Box<dyn Generator>> {
    let mut map: HashMap<String, Box<dyn Generator>> = HashMap::new();
    map.insert(
        "handlebars".to_owned(),
        Box::new(HandlebarsGenerator::new(&project_root)),
    );
    map.insert("echo".to_owned(), Box::new(EchoGenerator));
    map
}

mod handlebars_generator {
    use std::{borrow::Borrow, fs, path::Path};

    use handlebars::{handlebars_helper, Handlebars};
    use path_absolutize::Absolutize;
    use serde_json::{Map, Value as Json};
    use serde_yaml::Value as Yaml;
    use tracing::info;
    use walkdir::WalkDir;

    use crate::project_config::GeneratorRule;

    use super::{Generator, GeneratorResult};

    handlebars_helper!(md2html: |markdown: String| {
        let options = pulldown_cmark::Options::empty();
        let parser = pulldown_cmark::Parser::new_ext(&markdown, options);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        html_output
    });

    fn setup_handlebars(project_root: impl AsRef<Path>) -> Handlebars<'static> {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("md2html", Box::new(md2html));

        for path in WalkDir::new(project_root.as_ref().join("src"))
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .filter(|p| p.is_file() && p.extension().map(|ext| ext == "hbs").unwrap_or(false))
        {
            let abs_path = path.absolutize().unwrap();
            handlebars
                .register_template_file(abs_path.to_string_lossy().borrow(), &abs_path)
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

    fn make_data(yaml: &Yaml, content: String) -> Map<String, Json> {
        let mut data = Map::new();
        data.insert("content".to_owned(), handlebars::to_json(content));
        data.insert("front".to_owned(), handlebars::to_json(yaml));
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
            let content = fs::read_to_string(&input_filepath).unwrap();
            let (maybe_yaml, content) = split_frontmatter(&content);
            let yaml = maybe_yaml.unwrap();
            let yaml: Yaml = serde_yaml::from_str(&yaml).unwrap();

            let data = make_data(&yaml, content);

            let html_output = self
                .handlebars
                .render(
                    Path::new(
                        rule.template
                            .as_deref()
                            .unwrap_or("src/templates/page.html.hbs"),
                    )
                    .absolutize_from(project_root.absolutize().unwrap().borrow())
                    .unwrap()
                    .to_string_lossy()
                    .borrow(),
                    &data,
                )
                .unwrap();

            if let Some(parent) = output_filepath.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&output_filepath, html_output).unwrap();

            GeneratorResult::Success
        }
    }
}

mod echo_generator {
    use std::{fs, path::Path};

    use crate::project_config::GeneratorRule;

    use super::{Generator, GeneratorResult};

    #[derive(Debug)]
    pub struct EchoGenerator;

    impl Generator for EchoGenerator {
        fn generate(
            &self,
            input_filepath: &std::path::Path,
            output_filepath: &std::path::Path,
            _project_root: &Path,
            _rule: &GeneratorRule,
        ) -> super::GeneratorResult {
            fs::copy(input_filepath, output_filepath).unwrap();

            GeneratorResult::Success
        }
    }
}
