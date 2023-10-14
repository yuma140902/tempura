use std::io::Read;

use crate::value::Value;

/// [`Loader`] reads [`Value`] from a file. There are several Loaders depending on the type of file.
/// For example, [`MarkdownLoader`] usually loads markdown files, while [`StaticResourceLoader`] loads static resources.
/// Which Loader reads which file is specified by the user in the configuration file `tempura.json`.
pub trait Loader {
    fn load(reader: impl Read) -> Value;
}
