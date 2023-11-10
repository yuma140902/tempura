use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use tracing::info;

use super::Pipeline;

/// Resource holds the file contents needed for the build process.
/// One resource instance is created per [`Pipeline`](crate::pipeline::Pipeline).
#[derive(Debug)]
pub struct Resource(HashMap<PathBuf, Vec<u8>>);

impl Resource {
    pub(super) fn load_for(
        pipeline: &Pipeline,
        project_root: impl AsRef<Path>,
    ) -> anyhow::Result<Self> {
        let mut map = HashMap::new();
        let project_root = project_root.as_ref();
        for path in pipeline.get_needed_paths() {
            let path = project_root.join(path);
            info!("prefetching resource {}", path.display());
            let bytes = fs::read(&path)
                .with_context(|| format!("failed to load file {}", path.display()))?;
            map.insert(path, bytes);
        }
        Ok(Self(map))
    }
}