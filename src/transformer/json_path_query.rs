use serde::{Deserialize, Serialize};
use serde_json_path::JsonPath;

use crate::{store::Store, Value};

use super::Transformer;

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonPathQuery {
    pub query: JsonPath,
}

impl Transformer for JsonPathQuery {
    fn transform(&self, value: &Value, _store: &Store) -> anyhow::Result<Value> {
        if let Value::JSON(json) = value {
            let result = self
                .query
                .query(json)
                .exactly_one()
                .map_err(|e| anyhow::anyhow!("failed to find json path {e}"))?;
            Ok(Value::JSON(result.clone()))
        } else {
            anyhow::bail!("value should be json, but it was {}", value.get_type_name())
        }
    }
}
