use std::io::Read;

use crate::value::Value;

mod blob_loader;
mod json_loader;
mod template_loader;
mod text_loader;
mod text_with_frontmatter_loader;
mod yaml_loader;

pub use blob_loader::*;
pub use json_loader::*;
pub use template_loader::*;
pub use text_loader::*;
pub use text_with_frontmatter_loader::*;
pub use yaml_loader::*;

/// [`Loader`] reads [`Value`] from a file. There are several Loaders depending on the type of file.
/// For example, [`TextWithFrontmatterLoader`] usually loads markdown files, while [`BlobLoader`] loads static resources as-is.
/// Which Loader reads which file is specified by the user in the configuration file `tempura.json`.
pub trait Loader {
    fn load(reader: impl Read) -> anyhow::Result<Value>;
}
