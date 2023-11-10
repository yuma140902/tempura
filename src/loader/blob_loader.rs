use anyhow::Context;

use crate::value::Value;

use super::Loader;

pub struct BlobLoader;

impl Loader for BlobLoader {
    #[tracing::instrument(err, skip_all)]
    fn load(mut reader: impl std::io::Read) -> anyhow::Result<Value> {
        let mut buf = Vec::new();
        reader
            .read_to_end(&mut buf)
            .context("Could not read Vec<u8>")?;

        Ok(Value::Bytes(buf))
    }
}
