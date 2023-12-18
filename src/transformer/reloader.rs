use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    pipeline::EnumLoader, BlobLoader, JsonLoader, Loader, TemplateLoader, TextLoader,
    TextWithFrontmatterLoader, Value, YamlLoader,
};

use super::Transformer;

#[derive(Debug, Deserialize, Serialize)]
pub struct Reloader {
    #[serde(rename = "with")]
    loader: EnumLoader,
}

impl Transformer for Reloader {
    fn transform(
        &self,
        value: &crate::Value,
        _store: &crate::store::Store,
    ) -> anyhow::Result<crate::Value> {
        match value {
            Value::JSON(serde_json::Value::String(string)) => {
                let reader = string.as_bytes();
                match self.loader {
                    EnumLoader::TextWithFrontmatter => TextWithFrontmatterLoader::load(reader),
                    EnumLoader::Text => TextLoader::load(reader),
                    EnumLoader::Template => TemplateLoader::load(reader),
                    EnumLoader::Json => JsonLoader::load(reader),
                    EnumLoader::Blob => BlobLoader::load(reader),
                    EnumLoader::Yaml => YamlLoader::load(reader),
                }
                .with_context(|| format!("failed to load value from {}", string))
            }
            Value::Bytes(bytes) => match self.loader {
                EnumLoader::TextWithFrontmatter => TextWithFrontmatterLoader::load(&bytes[..]),
                EnumLoader::Text => TextLoader::load(&bytes[..]),
                EnumLoader::Template => TemplateLoader::load(&bytes[..]),
                EnumLoader::Json => JsonLoader::load(&bytes[..]),
                EnumLoader::Blob => BlobLoader::load(&bytes[..]),
                EnumLoader::Yaml => YamlLoader::load(&bytes[..]),
            }
            .with_context(|| "failed to load value from bytes".to_string()),
            _ => anyhow::bail!(
                "value should be string or bytes, but it was {}",
                value.get_type_name()
            ),
        }
    }
}
