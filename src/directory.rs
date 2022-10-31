use std::path::{Path, PathBuf};

pub fn get_project_config_path(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join("tempura.json")
}

pub fn get_pages_directory(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join("src/pages")
}

pub fn get_templates_directory(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join("src/templates")
}

pub fn get_output_directory(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join("public")
}
