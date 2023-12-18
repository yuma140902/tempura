use anyhow::Context;

use crate::value::Value;

use super::Loader;

pub struct YamlLoader;

impl Loader for YamlLoader {
    fn load(reader: impl std::io::prelude::Read) -> anyhow::Result<Value> {
        let yaml = serde_yaml::from_reader(reader).with_context(|| "Could not parse YAML".to_string())?;

        Ok(Value::JSON(yaml))
    }
}
