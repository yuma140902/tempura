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

    fn execute(
        &self,
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
        resource: &Resource,
    ) -> anyhow::Result<()> {
        let input_path = input_path.as_ref();
        let output_path = output_path.as_ref();
        info!(
            "start pipeline {} -> {}",
            input_path.display(),
            output_path.display(),
        );

        let mut store = Store::new();

        let input_file = fs::File::open(&input_path)
            .with_context(|| format!("failed to open file \"{}\"", &input_path.display()))?;

        debug!("start loading entry with {:?} Loader", self.entry.type_);
        let value = match self.entry.type_ {
            EntryType::TextWithFrontmatter => TextWithFrontmatterLoader::load(input_file),
            EntryType::Json => todo!(),
            EntryType::Blob => BlobLoader::load(input_file),
        }
        .with_context(|| {
            format!(
                "failed to load file \"{}\" with {:?} Loader",
                &input_path.display(),
                self.entry.type_
            )
        })?;
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

        let bytes = match store.get(&self.output_key) {
            Some(output_value) => match output_value {
                Value::JSON(serde_json::Value::String(string)) => string.as_bytes(),
                Value::Bytes(bytes) => bytes,
                value => {
                    anyhow::bail!(
                        "output value type should be string or bytes, but it was {} (output_key={})",
                        get_value_type_name(value),
                        self.output_key
                    )
                }
            },
            None => {
                anyhow::bail!("no output value found output_key={}", self.output_key);
            }
        };

        if let Some(parent) = output_path.parent() {
            debug!("create parent directory");
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory \"{}\"", parent.display()))?;
        }
        let mut output_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(&output_path)
            .with_context(|| format!("failed to open output file \"{}\"", output_path.display()))?;
        debug!("start writing output file \"{}\"", output_path.display());
        output_file
            .write_all(bytes)
            .with_context(|| format!("failed to write file \"{}\"", output_path.display()))?;
        debug!("done writing output file");

        info!("done pipeline");

        Ok(())
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
