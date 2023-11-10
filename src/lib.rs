use std::{collections::HashMap, fs, io, path::Path};

use anyhow::Context;
use tracing::error;
use walkdir::WalkDir;

use crate::project_config::ProjectConfig;

pub mod cli;
pub mod directory;
pub mod handlebars_helpers;
mod loader;
pub mod pipeline;
pub mod project_config;
pub mod store;
mod template_engine;
mod value;

pub use loader::*;
pub use template_engine::*;
pub use value::*;

#[tracing::instrument]
pub fn build(project_root: &Path) -> anyhow::Result<()> {
    let project_config_path = directory::get_project_config_path(project_root);
    let config: ProjectConfig = serde_yaml::from_str(
        &fs::read_to_string(&project_config_path)
            .with_context(|| format!("could not load file {}", project_config_path.display()))?,
    )
    .with_context(|| {
        format!(
            "failed to parse project config {}",
            project_config_path.display()
        )
    })?;

    dbg!(&config);

    let pages_directory = directory::get_pages_directory(project_root);

    let mut jobs = vec![];

    for filepath in WalkDir::new(&pages_directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| path.is_file())
    {
        // TODO: filepathはpages_directoryからの相対パスのはずだが確証がないので調べる
        let relative_filepath = filepath;
        let mut selected_pipeline = None;
        for pipeline in config.pipelines.iter() {
            if pipeline.accepts(&relative_filepath) {
                selected_pipeline = Some(pipeline);
                break;
            }
        }

        match selected_pipeline {
            None => {
                error!("File {}: No pipeline found", relative_filepath.display());
                continue;
            }
            Some(pipeline) => jobs.push(pipeline.to_job(&relative_filepath, &project_root)),
        }
    }

    let mut resources = HashMap::new();
    // 各パイプラインごとに(各Jobごとではない)必要なリソースを事前読込する
    for pipeline in config.pipelines.iter() {
        resources.insert(
            &pipeline.name,
            pipeline
                .prefetch_resources(&project_root)
                .with_context(|| {
                    format!(
                        "failed to prefetch files for pipeline \"{}\"",
                        pipeline.name
                    )
                })?,
        );
    }

    for job in jobs {
        job.execute(resources.get(&&job.pipeline().name).expect(&format!(
            "could not find prefetched resource for pipeline \"{}\"",
            job.pipeline().name
        )))
        .with_context(|| {
            format!(
                "failed to complete job for pipeline \"{}\" and entry file \"{}\"",
                job.pipeline().name,
                job.input_path().display()
            )
        })?;
    }

    Ok(())
}

#[tracing::instrument]
pub fn init(project_root: &Path) -> io::Result<()> {
    println!("not implemented yet");

    Ok(())
}
