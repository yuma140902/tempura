mod entry;
mod job;
mod resource;
mod step;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

pub use entry::*;
pub use job::*;
pub use resource::*;
pub use step::*;
use tracing::info;

use crate::{directory, store::Store, BlobLoader, Loader, TextWithFrontmatterLoader};

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
            "start pipeline \"{}\" {} -> {}",
            self.name,
            input_path.display(),
            output_path.display(),
        );
        // TODO: パイプラインの中身を実装

        let mut store = Store::new();

        let input_file = fs::File::open(&input_path)
            .with_context(|| format!("failed to open file \"{}\"", &input_path.display()))?;

        info!("start loading entry with {:?} Loader", self.entry.type_);
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
        info!("done loading entry");

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
