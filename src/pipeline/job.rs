use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Context;
use tracing::{debug, info};

use super::{Pipeline, Resource};

pub struct Job<'a> {
    pub(super) input_path: PathBuf,
    pub(super) output_path: PathBuf,
    pub(super) pipeline: &'a Pipeline,
}

impl<'a> Job<'a> {
    #[tracing::instrument(ret, skip_all, fields(pipeline = self.pipeline.name, input_path = self.input_path.to_str()))]
    pub fn execute(&self, resource: &Resource) -> anyhow::Result<()> {
        info!(
            "start pipeline {} -> {}",
            self.input_path.display(),
            self.output_path.display(),
        );

        let mut input_file = fs::File::open(&self.input_path)
            .with_context(|| format!("failed to open file \"{}\"", &self.input_path.display()))?;

        let mut input_bytes = Vec::new();
        input_file
            .read_to_end(&mut input_bytes)
            .with_context(|| format!("failed to read file \"{}\"", &self.input_path.display()))?;

        let output_bytes = self
            .pipeline
            .execute(input_bytes, resource)
            .context("pipeline failed")?;

        if let Some(parent) = self.output_path.parent() {
            debug!("create parent directory");
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory \"{}\"", parent.display()))?;
        }
        let mut output_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(&self.output_path)
            .with_context(|| {
                format!(
                    "failed to open output file \"{}\"",
                    self.output_path.display()
                )
            })?;
        output_file
            .write_all(&output_bytes)
            .with_context(|| format!("failed to write file \"{}\"", self.output_path.display()))?;

        info!("done");
        Ok(())
    }

    pub fn pipeline(&self) -> &Pipeline {
        self.pipeline
    }

    pub fn input_path(&self) -> &PathBuf {
        &self.input_path
    }

    pub fn output_path(&self) -> &PathBuf {
        &self.output_path
    }
}
