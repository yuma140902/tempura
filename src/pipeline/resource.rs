use std::{collections::HashMap, fs, path::Path};

use anyhow::Context;
use path_absolutize::Absolutize;
use tracing::{debug, info, span, Level};

use crate::{Loader, TemplateLoader, Value};

use super::Pipeline;

/// Resource holds the file contents needed for the build process.
/// One resource instance is created per [`Pipeline`](crate::pipeline::Pipeline).
#[derive(Debug)]
pub struct Resource {
    byte_map: HashMap<usize, Vec<u8>>,
    value_map: HashMap<usize, Value>,
}

impl Resource {
    pub(super) fn load_for(
        pipeline: &Pipeline,
        project_root: impl AsRef<Path>,
    ) -> anyhow::Result<Self> {
        let mut byte_map = HashMap::new();
        let mut value_map = HashMap::new();
        let project_root = project_root.as_ref();
        for (index, step) in pipeline.steps.iter().enumerate() {
            if let super::Step::Load { path, with, .. } = step {
                let load_span = span!(Level::INFO, "prefetch", index = index, path = path.to_str());
                let _enter = load_span.enter();
                info!("start prefetch");

                let path = path.absolutize_from(project_root).unwrap();
                debug!("absolute path {}", path.display());
                let bytes = fs::read(&path)
                    .with_context(|| format!("failed to load file {}", path.display()))?;
                byte_map.insert(index, bytes);

                info!("finish prefetch");
                info!("start preload");

                let value = match with {
                    crate::pipeline::EnumLoader::Template => {
                        TemplateLoader::load(&byte_map.get(&index).unwrap()[..])
                    }
                    crate::pipeline::EnumLoader::Json => todo!(),
                }
                .with_context(|| format!("failed to preload with {:?}", with))?;
                value_map.insert(index, value);

                info!("finish preload");
            }
        }
        Ok(Self {
            byte_map,
            value_map,
        })
    }

    pub fn get_bytes(&self, key: &usize) -> Option<&[u8]> {
        self.byte_map.get(key).map(|v| &v[..])
    }

    pub fn get_value(&self, key: &usize) -> Option<&Value> {
        self.value_map.get(key)
    }
}
