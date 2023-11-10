use crate::{store::Store, Value};

mod template_renderer {
    use std::borrow::Cow;

    use anyhow::Context;
    use serde::{Deserialize, Serialize};

    use crate::{store::Store, TemplateEngine, Value};

    use super::Transformer;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TemplateRenderer {
        pub template_key: String,
    }

    impl Transformer for TemplateRenderer {
        fn transform(&self, value: &Value, store: &Store) -> anyhow::Result<Value> {
            let mut engine = TemplateEngine::new();
            let template_string = if let Some(template_value) = store.get(&self.template_key) {
                match template_value {
                    Value::JSON(serde_json::Value::String(string)) => string,
                    _ => {
                        // TODO: すべてのanyhow::bailに対してtracing::instruments(err)を付ける
                        anyhow::bail!(
                            "template value should be string or Template, but it was {}",
                            template_value.get_type_name()
                        )
                    }
                }
            } else {
                anyhow::bail!(
                    "no template value found for template_key {}",
                    self.template_key
                )
            };

            engine
                .register_template_from_string(&self.template_key, template_string.to_string())
                .with_context(|| format!("failed to register template from string"))?;

            let result_string = engine
                .render(
                    &self.template_key,
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
        }
    }
}

pub use template_renderer::*;

pub trait Transformer {
    fn transform(&self, value: &Value, store: &Store) -> anyhow::Result<Value>;
}
