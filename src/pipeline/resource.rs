use std::{collections::HashMap, fs, path::Path};

use anyhow::Context;
use path_absolutize::Absolutize;
use tracing::{debug, info, span, Level};

use crate::{
    BlobLoader, JsonLoader, Loader, TemplateLoader, TextLoader, TextWithFrontmatterLoader, Value,
    YamlLoader,
};

use super::Pipeline;

/// Resource holds the file contents needed for the build process.
/// One resource instance is created per [`Pipeline`]
#[derive(Debug)]
pub struct Resource {
    value_map: HashMap<usize, Value>,
}

impl Resource {
    pub(super) fn load_for(
        pipeline: &Pipeline,
        project_root: impl AsRef<Path>,
    ) -> anyhow::Result<Self> {
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

                info!("finish prefetch");
                info!("start preload");

                let bytes = &bytes[..];
                let value = match with {
                    crate::pipeline::EnumLoader::Template => TemplateLoader::load(bytes),
                    crate::pipeline::EnumLoader::Json => JsonLoader::load(bytes),
                    crate::pipeline::EnumLoader::TextWithFrontmatter => {
                        TextWithFrontmatterLoader::load(bytes)
                    }
                    crate::pipeline::EnumLoader::Blob => BlobLoader::load(bytes),
                    crate::pipeline::EnumLoader::Text => TextLoader::load(bytes),
                    crate::pipeline::EnumLoader::Yaml => YamlLoader::load(bytes),
                }
                .with_context(|| format!("failed to preload with {:?}", with))?;
                value_map.insert(index, value);

                info!("finish preload");
            }
        }
        Ok(Self { value_map })
    }

    pub fn get_value(&self, key: &usize) -> Option<&Value> {
        self.value_map.get(key)
    }
}
