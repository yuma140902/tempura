use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{store::Store, TemplateEngine, Value};

use super::Transformer;

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateRenderer {
    pub template_key: String,
    pub current_directory: Option<CurrentDirectory>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum CurrentDirectory {
    EntryDirectory,
    Path { path: String },
}

impl Transformer for TemplateRenderer {
    fn transform(&self, value: &Value, store: &Store) -> anyhow::Result<Value> {
        let mut engine = TemplateEngine::new();
        if let Some(template_value) = store.get(&self.template_key) {
            match template_value {
                Value::JSON(serde_json::Value::String(string)) => {
                    engine
                        .register_template_from_string(&self.template_key, string.to_string())
                        .with_context(|| "failed to register template from string".to_string())?;
                }
                Value::Template(template) => {
                    engine.register_template(&self.template_key, template.clone());
                }
                _ => {
                    // TODO: すべてのanyhow::bailと.context、.with_contextに対してtracing::instruments(err)を付ける
                    anyhow::bail!(
                        "template value should be string or template, but it was {}",
                        template_value.get_type_name()
                    )
                }
            }
        } else {
            anyhow::bail!(
                "no template value found for template_key {}",
                self.template_key
            )
        }

        let current_directory: Option<String> = if let Some(CurrentDirectory::EntryDirectory) =
            &self.current_directory
        {
            match store.get("___entry_directory") {
                Some(Value::JSON(serde_json::Value::String(entry_dir))) => Some(entry_dir.into()),
                Some(_) => {
                    anyhow::bail!("___entry_directory was not string");
                }
                None => {
                    warn!("___entry_directory was not found");
                    None
                }
            }
        } else if let Some(CurrentDirectory::Path { path }) = &self.current_directory {
            Some(path.clone())
        } else {
            None
        };

        let result_string = engine
            .render(
                &self.template_key,
                &current_directory,
                &transform_value_for_rendering(value)
                    .context("failed to transform value for rendering")?,
            )
            .context("failed to render template")?;

        Ok(Value::JSON(serde_json::Value::String(result_string)))
    }
}

fn transform_value_for_rendering(value: &Value) -> anyhow::Result<serde_json::Value> {
    match value {
        Value::Bytes(bytes) => {
            Ok(serde_json::to_value(bytes).context("failed to transform bytes to JSON")?)
        }
        Value::JSON(json) => match json {
            serde_json::Value::Null => anyhow::bail!("failed to transform null to object"),
            serde_json::Value::Bool(_)
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_)
            | serde_json::Value::Array(_) => {
                let mut map = serde_json::Map::new();
                map.insert(value.get_type_name().to_string(), json.clone());
                Ok(serde_json::Value::Object(map))
            }
            serde_json::Value::Object(_) => Ok(json.clone()),
        },
        Value::Template(_) => anyhow::bail!("failed because template data is also template"),
    }
}
