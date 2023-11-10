use serde::{Deserialize, Serialize};
use serde_json_path::JsonPath;
use tracing::warn;

use crate::{store::Store, Value};

use super::Transformer;

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonPathQueryAll {
    pub query: JsonPath,
}

impl Transformer for JsonPathQueryAll {
    #[tracing::instrument(skip(value, _store))]
    fn transform(&self, value: &Value, _store: &Store) -> anyhow::Result<Value> {
        if let Value::JSON(json) = value {
            let result = self.query.query(json).all();
            if result.is_empty() {
                warn!("zero elements extracted");
            }

            Ok(Value::JSON(serde_json::Value::Array(
                result.into_iter().map(ToOwned::to_owned).collect(),
            )))
        } else {
            anyhow::bail!("value should be json, but it was {}", value.get_type_name())
        }
    }
}
