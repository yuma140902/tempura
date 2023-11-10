mod entry;
mod job;
mod resource;
mod step;

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

pub use entry::*;
pub use job::*;
pub use resource::*;
pub use step::*;
use tracing::{debug, info, span, Level};

use crate::{directory, store::Store, BlobLoader, Loader, TextWithFrontmatterLoader, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pipeline {
    pub name: String,
    pub entry: Entry,
    pub steps: Vec<Step>,
    // TODO: もっと柔軟に出力パスを指定できるようにする
    pub output_extension: Option<String>,
    pub output_key: String,
}

impl Pipeline {
    pub fn accepts(&self, path: impl AsRef<Path>) -> bool {
        self.entry
            .match_regex
            .is_match(path.as_ref().to_string_lossy().as_ref())
    }

    #[tracing::instrument(err, skip(self, project_root), fields(pipeline = self.name))]
    pub fn prefetch_resources(&self, project_root: impl AsRef<Path>) -> anyhow::Result<Resource> {
        info!("start");
        let ret = Resource::load_for(self, project_root);
        info!("done");
        ret
    }

    fn execute(&self, entry_bytes: Vec<u8>, resource: &Resource) -> anyhow::Result<Vec<u8>> {
        let mut store = Store::new();

        debug!("start loading entry with {:?} Loader", self.entry.type_);
        let value = match self.entry.type_ {
            EntryType::TextWithFrontmatter => TextWithFrontmatterLoader::load(&entry_bytes[..]),
            EntryType::Json => todo!(),
            EntryType::Blob => BlobLoader::load(&entry_bytes[..]),
        }
        .with_context(|| format!("failed to load entry with {:?} Loader", self.entry.type_))?;
        store.set("entry".to_string(), value);
        debug!("done loading entry");

        for (index, step) in self.steps.iter().enumerate() {
            let step_span = span!(
                Level::INFO,
                "step",
                index = index + 1,
                max = self.steps.len()
            );
            let _enter = step_span.enter();
            debug!("start");
            // TODO: stepの処理
            debug!("done");
        }

        let bytes = match store.get_owned(&self.output_key) {
            Some(output_value) => match output_value {
                Value::JSON(serde_json::Value::String(string)) => string.into_bytes(),
                Value::Bytes(bytes) => bytes,
                value => {
                    anyhow::bail!(
                        "output value type should be string or bytes, but it was {} (output_key={})",
                        get_value_type_name(&value),
                        self.output_key
                    )
                }
            },
            None => {
                anyhow::bail!("no output value found output_key={}", self.output_key);
            }
        };

        Ok(bytes)
    }

    fn get_output_path(
        &self,
        input_path: impl AsRef<Path>,
        project_root: impl AsRef<Path>,
    ) -> PathBuf {
        let src_dir = directory::get_src_directory(&project_root);
        let output_dir = directory::get_output_directory(&project_root);
        let mut output_filepath =
            output_dir.join(directory::get_relative_path(&input_path, src_dir));
        if let Some(output_extension) = &self.output_extension {
            output_filepath.set_extension(output_extension);
        }
        output_filepath
    }

    pub fn to_job(&self, input_path: impl AsRef<Path>, project_root: impl AsRef<Path>) -> Job<'_> {
        Job {
            input_path: input_path.as_ref().to_path_buf(),
            output_path: self.get_output_path(&input_path, &project_root),
            pipeline: &self,
        }
    }
}

fn get_value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Bytes(_) => "bytes",
        Value::JSON(json) => get_json_type_name(json),
    }
}

fn get_json_type_name(json: &serde_json::Value) -> &'static str {
    match json {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "bool",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}
