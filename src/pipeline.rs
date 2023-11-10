mod entry;
mod job;
mod resource;
mod step;

use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

pub use entry::*;
pub use job::*;
pub use resource::*;
pub use step::*;
use tracing::{debug, info, span, Level};

use crate::{
    directory, store::Store, transformer::Transformer, BlobLoader, JsonLoader, Loader,
    TemplateLoader, TextWithFrontmatterLoader, Value,
};

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
            .is_match(&path.as_ref().to_string_lossy().replace("\\", "/"))
    }

    #[tracing::instrument(err, skip(self, project_root), fields(pipeline = self.name))]
    pub fn prefetch_resources(&self, project_root: impl AsRef<Path>) -> anyhow::Result<Resource> {
        info!("start prepare");
        let ret = Resource::load_for(self, project_root);
        info!("finish prepare");
        ret
    }

    fn execute(
        &self,
        entry_bytes: Vec<u8>,
        resource: &Resource,
        entry_directory: Option<String>,
    ) -> anyhow::Result<Vec<u8>> {
        let mut store = Store::new();

        debug!("start loading entry with {:?} Loader", self.entry.type_);
        let value = match self.entry.type_ {
            EnumLoader::TextWithFrontmatter => TextWithFrontmatterLoader::load(&entry_bytes[..]),
            EnumLoader::Json => JsonLoader::load(&entry_bytes[..]),
            EnumLoader::Blob => BlobLoader::load(&entry_bytes[..]),
            EnumLoader::Template => TemplateLoader::load(&entry_bytes[..]),
            EnumLoader::Text => JsonLoader::load(&entry_bytes[..]),
        }
        .with_context(|| format!("failed to load entry with {:?} Loader", self.entry.type_))?;
        store.set("entry".to_string(), value);
        if let Some(entry_directory) = entry_directory {
            store.set(
                "___entry_directory".to_string(),
                Value::JSON(serde_json::Value::String(entry_directory)),
            );
        }
        debug!("finish loading entry");

        for (index, step) in self.steps.iter().enumerate() {
            let step_span = span!(
                Level::INFO,
                "step",
                index = index + 1,
                max = self.steps.len()
            );
            let _enter = step_span.enter();
            debug!("start");
            match step {
                Step::Load { key, with, .. } => {
                    if let Some(value) = resource.get_value(&index) {
                        store.set(key.to_string(), value.clone());
                    } else if let Some(bytes) = resource.get_bytes(&index) {
                        let value = match with {
                            EnumLoader::Template => TemplateLoader::load(bytes),
                            EnumLoader::Json => JsonLoader::load(bytes),
                            EnumLoader::TextWithFrontmatter => {
                                TextWithFrontmatterLoader::load(bytes)
                            }
                            EnumLoader::Blob => BlobLoader::load(bytes),
                            EnumLoader::Text => JsonLoader::load(bytes),
                        }
                        .with_context(|| {
                            format!(
                                "failed to load {} with {:?} Loader (steps index: {})",
                                key, self.entry.type_, index
                            )
                        })?;
                        store.set(key.to_string(), value);
                    } else {
                        anyhow::bail!("no value prefetched for key {}", key);
                    }
                }
                Step::Transform {
                    input,
                    output,
                    with,
                } => {
                    if let Some(input) = store.get_combined(input) {
                        debug!("transform input type: {}", input.get_type_name());
                        let value = match with {
                            EnumTransformer::TemplateRenderer(t) => t.transform(&input, &store),
                            EnumTransformer::JsonPath(t) => t.transform(&input, &store),
                            EnumTransformer::JsonPathAll(t) => t.transform(&input, &store),
                            EnumTransformer::TextAsTemplate(t) => t.transform(&input, &store),
                        }
                        .with_context(|| "transformer failed".to_string())?;
                        debug!("transform output type: {}", value.get_type_name());
                        store.set(output.to_string(), value);
                    } else {
                        anyhow::bail!("no value found in the store for input key {:?}", input,)
                    }
                }
                Step::DumpStore => {
                    println!("{store}");
                }
            }
            debug!("finish");
        }

        let bytes = match store.get_owned(&self.output_key) {
            Some(output_value) => match output_value {
                Value::JSON(serde_json::Value::String(string)) => string.into_bytes(),
                Value::Bytes(bytes) => bytes,
                value => {
                    anyhow::bail!(
                        "output value type should be string or bytes, but it was {} (output_key={})",
                        value.get_type_name(),
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
            pipeline: self,
        }
    }
}
