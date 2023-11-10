use std::collections::HashMap;

use crate::Value;

/// Store is a key-value storage for the build process.
/// One store instance is created per [`Job`](crate::pipeline::Job).
pub struct Store(HashMap<String, Value>);

// TODO: Valueの変更を追跡してもとのファイルまで辿れるようにする

impl Store {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.0.insert(key, value);
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
