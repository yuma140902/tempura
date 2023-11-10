use std::{collections::HashMap, fs, path::Path};

use anyhow::Context;
use path_absolutize::Absolutize;
use tracing::info;

use super::Pipeline;

/// Resource holds the file contents needed for the build process.
/// One resource instance is created per [`Pipeline`](crate::pipeline::Pipeline).
#[derive(Debug)]
pub struct Resource(HashMap<String, Vec<u8>>);

impl Resource {
    pub(super) fn load_for(
        pipeline: &Pipeline,
        project_root: impl AsRef<Path>,
    ) -> anyhow::Result<Self> {
        let mut map = HashMap::new();
        let project_root = project_root.as_ref();
        for step in &pipeline.steps {
            match step {
                super::Step::Load { path, name, .. } => {
                    let path = path.absolutize_from(&project_root).unwrap();

                    info!("prefetching resource {}", path.display());
                    let bytes = fs::read(&path)
                        .with_context(|| format!("failed to load file {}", path.display()))?;
                    map.insert(name.to_string(), bytes);
                }
                _ => {}
            }
        }
        Ok(Self(map))
    }
}
