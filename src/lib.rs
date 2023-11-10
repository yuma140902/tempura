use std::{collections::HashMap, fs, io, path::Path};

use anyhow::Context;
use path_absolutize::Absolutize;
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

#[tracing::instrument(ret)]
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

    let src_dir = directory::get_src_directory(project_root);
    let abs_project_root = project_root.absolutize().unwrap();

    let mut jobs = vec![];

    for filepath in WalkDir::new(&src_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| path.is_file())
    {
        let abs_filepath = filepath.absolutize().unwrap();
        let relative_filepath = directory::get_relative_path(&abs_filepath, &abs_project_root);
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
            Some(pipeline) => jobs.push(pipeline.to_job(&abs_filepath, &abs_project_root)),
        }
    }

    let mut resources = HashMap::new();
    // 各パイプラインごとに(各Jobごとではない)必要なリソースを事前読込する
    for pipeline in config.pipelines.iter() {
        resources.insert(
            &pipeline.name,
            pipeline
                .prefetch_resources(&abs_project_root)
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

#[tracing::instrument(ret)]
pub fn init(project_root: &Path) -> io::Result<()> {
    println!("not implemented yet");

    Ok(())
}
