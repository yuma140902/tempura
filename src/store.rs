use std::{collections::HashMap, fmt::Display};

use tracing::warn;

use crate::{pipeline::InputKey, Value};

/// Store is a key-value storage for the build process.
/// One store instance is created per [`Job`](crate::pipeline::Job).
#[derive(Debug)]
pub struct Store(HashMap<String, Value>);

// TODO: Valueの変更を追跡してもとのファイルまで辿れるようにする

impl Store {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    #[tracing::instrument(skip_all)]
    pub fn get_combined(&self, key: &InputKey) -> Option<Value> {
        match key {
            InputKey::Single(key) => self.get(key).map(ToOwned::to_owned),
            InputKey::List(keys) => {
                let mut v = vec![];
                for key in keys {
                    if let Some(Value::JSON(json)) = self.get(key) {
                        v.push(json.clone());
                    } else {
                        warn!("key {} not found or has invalid type", key);
                        v.push(serde_json::Value::Null);
                    }
                }
                Some(Value::JSON(serde_json::Value::Array(v)))
            }
            InputKey::Map(map) => {
                let mut value_map = serde_json::Map::new();
                for (kk, vk) in map {
                    if let Some(Value::JSON(json)) = self.get(vk) {
                        value_map.insert(kk.to_string(), json.clone());
                    } else {
                        warn!("key {} not found or has invalid type", vk);
                        value_map.insert(kk.to_string(), serde_json::Value::Null);
                    }
                }
                Some(Value::JSON(serde_json::Value::Object(value_map)))
            }
        }
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.0.insert(key, value);
    }

    pub fn get_owned(mut self, key: &str) -> Option<Value> {
        self.0.remove(key)
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in &self.0 {
            f.write_fmt(format_args!("{} = ", k))?;
            match v {
                Value::Bytes(bytes) => {
                    f.write_str("Bytes([")?;
                    for byte in bytes.iter().take(20) {
                        f.write_fmt(format_args!("{}, ", byte))?;
                    }
                    f.write_str("...])\n")?;
                }
                Value::Template(_) => f.write_str("(compiled template)\n")?,
                Value::JSON(json) => f.write_fmt(format_args!("{:#?}", json))?,
            }
        }
        Ok(())
    }
}
