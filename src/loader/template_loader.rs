use anyhow::Context;

use crate::{Loader, Value};

pub struct TemplateLoader;

// TODO: Loader.prepare(bytes) -> Option<Value>を追加する
// ResourceにHashMapを1つもたせる
// 1つはstepのインデックス→bytes
// もう1つはstepのインデックス→Loader.prepare(bytes)が返したValue

impl Loader for TemplateLoader {
    #[tracing::instrument(err, skip_all)]
    fn load(mut reader: impl std::io::Read) -> anyhow::Result<Value> {
        let mut buf = String::new();
        reader
            .read_to_string(&mut buf)
            .context("Could not read String")?;

        Ok(Value::JSON(serde_json::Value::String(buf)))
    }
}
