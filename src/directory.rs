use std::path::{Path, PathBuf};

use tracing::debug;

pub fn get_project_config_path(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join("tempura.yml")
}

pub fn get_src_directory(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join("src")
}

pub fn get_output_directory(project_root: impl AsRef<Path>) -> PathBuf {
    project_root.as_ref().join("public")
}

#[tracing::instrument(skip_all)]
pub fn get_relative_path(file: impl AsRef<Path>, base_directory: impl AsRef<Path>) -> PathBuf {
    // TODO: AbsPathとRelPathを実装する
    let ret = pathdiff::diff_paths(file.as_ref(), base_directory.as_ref()).unwrap();
    debug!(
        "diff_paths({}, {}) => {}",
        file.as_ref().display(),
        base_directory.as_ref().display(),
        ret.display()
    );
    ret
}
