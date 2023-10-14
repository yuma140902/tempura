use std::{fs, path::Path};

use crate::project_config::GeneratorRule;

use super::Generator;

use anyhow::Context as _;

#[derive(Debug)]
pub struct EchoGenerator;

impl Generator for EchoGenerator {
    fn generate(
        &self,
        input_filepath: &std::path::Path,
        output_filepath: &std::path::Path,
        _project_root: &Path,
        _rule: &GeneratorRule,
    ) -> super::GeneratorResult {
        if let Some(parent) = output_filepath.parent() {
            fs::create_dir_all(parent).context(format!("Create directory {:?}", parent))?;
        }
        fs::copy(input_filepath, output_filepath)
            .context(format!(
                "Copying {:?} to {:?}",
                input_filepath, output_filepath
            ))
            .map(|_| ())
    }
}
