use anyhow::Context;
use handlebars::Template;

use crate::{Loader, Value};

pub struct TemplateLoader;

impl Loader for TemplateLoader {
    #[tracing::instrument(err, skip_all)]
    fn load(mut reader: impl std::io::Read) -> anyhow::Result<Value> {
        let mut buf = String::new();
        reader
            .read_to_string(&mut buf)
            .context("Could not read String")?;

        let template = Template::compile(&buf)
            .with_context(|| format!("failed to compile template: {}", buf))?;

        Ok(Value::Template(template))
    }
}
