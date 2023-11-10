use anyhow::Context;
use handlebars::Template;
use serde::{Deserialize, Serialize};

use crate::{store::Store, Value};

use super::Transformer;

#[derive(Debug, Deserialize, Serialize)]
pub struct TextAsTemplate;

impl Transformer for TextAsTemplate {
    fn transform(&self, value: &Value, _store: &Store) -> anyhow::Result<Value> {
        if let Value::JSON(serde_json::Value::String(string)) = value {
            Ok(Value::Template(Template::compile(string).with_context(
                || format!("failed to compile template {}", string),
            )?))
        } else {
            anyhow::bail!(
                "value should be string, but it was {}",
                value.get_type_name()
            )
        }
    }
}
