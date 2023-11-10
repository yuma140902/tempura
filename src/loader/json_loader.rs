use anyhow::Context;

use crate::value::Value;

use super::Loader;

pub struct JsonLoader;

impl Loader for JsonLoader {
    #[tracing::instrument(err, skip_all)]
    fn load(reader: impl std::io::Read) -> anyhow::Result<Value> {
        let json =
            serde_json::from_reader(reader).with_context(|| "Could not parse JSON".to_string())?;

        Ok(Value::JSON(json))
    }
}
