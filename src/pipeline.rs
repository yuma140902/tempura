mod entry;
mod step;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub use entry::*;
pub use step::*;

use crate::directory;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pipeline {
    pub entry: Entry,
    pub steps: Vec<Step>,
    // TODO: もっと柔軟に出力パスを指定できるようにする
    pub output_extension: Option<String>,
}

impl Pipeline {
    pub fn accepts(&self, path: impl AsRef<Path>) -> bool {
        self.entry
            .match_regex
            .is_match(path.as_ref().to_string_lossy().as_ref())
    }

    pub fn get_needed_resources(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        for step in &self.steps {
            match step {
                Step::Load { path, .. } => paths.push(path.clone()),
                Step::Transform { .. } => {}
            }
        }
        paths
    }

    fn execute(
        &self,
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let input_path = input_path.as_ref();
        let output_path = output_path.as_ref();
        println!(
            "{} -> {} (pipeline: {:?})",
            input_path.display(),
            output_path.display(),
            self
        );
        // TODO: パイプラインの中身を実装
        Ok(())
    }

    fn get_output_path(
        &self,
        input_path: impl AsRef<Path>,
        project_root: impl AsRef<Path>,
    ) -> PathBuf {
        let pages_directory = directory::get_pages_directory(&project_root);
        let output_directory = directory::get_output_directory(&project_root);
        let mut output_filepath =
            output_directory.join(directory::get_relative_path(&input_path, pages_directory));
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

pub struct Job<'a> {
    input_path: PathBuf,
    output_path: PathBuf,
    pipeline: &'a Pipeline,
}

impl<'a> Job<'a> {
    pub fn execute(&self) -> anyhow::Result<()> {
        self.pipeline.execute(&self.input_path, &self.output_path)
    }
}
