use std::path::PathBuf;

use super::{Pipeline, Resource};

pub struct Job<'a> {
    pub(super) input_path: PathBuf,
    pub(super) output_path: PathBuf,
    pub(super) pipeline: &'a Pipeline,
}

impl<'a> Job<'a> {
    pub fn execute(&self, resource: &Resource) -> anyhow::Result<()> {
        self.pipeline
            .execute(&self.input_path, &self.output_path, resource)
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
