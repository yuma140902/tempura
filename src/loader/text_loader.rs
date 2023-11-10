use anyhow::Context;

use crate::value::Value;

use super::Loader;

pub struct TextLoader;

impl Loader for TextLoader {
    #[tracing::instrument(err, skip_all)]
    fn load(mut reader: impl std::io::Read) -> anyhow::Result<Value> {
        let mut buf = String::new();
        reader
            .read_to_string(&mut buf)
            .context("Could not read String")?;

        Ok(Value::JSON(serde_json::Value::String(buf)))
    }
}
