mod entry;
mod job;
mod resource;
mod step;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub use entry::*;
pub use job::*;
pub use resource::*;
pub use step::*;

use crate::directory;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pipeline {
    pub name: String,
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

    #[tracing::instrument(skip(self, project_root), fields(pipeline_name = self.name, project_root = project_root.as_ref().to_str()))]
    pub fn prefetch_resources(&self, project_root: impl AsRef<Path>) -> anyhow::Result<Resource> {
        Resource::load_for(self, project_root)
    }

    fn execute(
        &self,
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
        resource: &Resource,
    ) -> anyhow::Result<()> {
        let input_path = input_path.as_ref();
        let output_path = output_path.as_ref();
        println!(
            "{} -> {} (pipeline: {})",
            input_path.display(),
            output_path.display(),
            self.name
        );
        println!("resource: {:?}", resource);
        // TODO: パイプラインの中身を実装
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
