use std::path::{Path, PathBuf};

pub fn get_project_config_path(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join("tempura.json")
}

pub fn get_pages_directory(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join("src").join("pages")
}

pub fn get_output_directory(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join("public")
}

pub fn get_relative_path(file: impl AsRef<Path>, base_directory: impl AsRef<Path>) -> PathBuf {
    pathdiff::diff_paths(file, base_directory).unwrap()
}
