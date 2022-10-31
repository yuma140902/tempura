use std::path::{Path, PathBuf};

pub fn get_pages_directory(root_directory: impl AsRef<Path>) -> PathBuf {
    root_directory.as_ref().join("pages")
}

pub fn get_templates_directory(root_directory: impl AsRef<Path>) -> PathBuf {
    root_directory.as_ref().join("templates")
}

pub fn get_output_directory(root_directory: impl AsRef<Path>) -> PathBuf {
    root_directory.as_ref().join("public")
}
