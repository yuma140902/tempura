use std::path::PathBuf;

use tracing::info;

use super::{Pipeline, Resource};

pub struct Job<'a> {
    pub(super) input_path: PathBuf,
    pub(super) output_path: PathBuf,
    pub(super) pipeline: &'a Pipeline,
}

impl<'a> Job<'a> {
    #[tracing::instrument(ret, skip_all, fields(pipeline = self.pipeline.name, input_path = self.input_path.to_str()))]
    pub fn execute(&self, resource: &Resource) -> anyhow::Result<()> {
        info!("start");
        let ret = self
            .pipeline
            .execute(&self.input_path, &self.output_path, resource);
        info!("done");
        ret
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
