use std::{collections::HashMap, fs, io, path::Path};

use anyhow::Context;
use path_absolutize::Absolutize;
use tracing::{debug, warn};
use walkdir::WalkDir;

use crate::{project_config::ProjectConfig, transformer::handlebars_helpers};

pub mod cli;
pub mod directory;
mod loader;
pub mod pipeline;
pub mod project_config;
pub mod store;
mod string_matcher;
pub mod transformer;
mod value;

pub use loader::*;
pub use string_matcher::*;
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

    for filepath in WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| path.is_file())
    {
        debug!("found file {}", filepath.display());
        let abs_filepath = filepath.absolutize().unwrap();
        let relative_filepath = directory::get_relative_path(&abs_filepath, &abs_project_root);
        debug!("relative path = {}", relative_filepath.display());
        let mut selected_pipeline = None;
        for pipeline in config.pipelines.iter() {
            if pipeline.accepts(&relative_filepath) {
                debug!(
                    "found pipeline \"{}\" for file \"{}\"",
                    pipeline.name,
                    relative_filepath.display()
                );
                selected_pipeline = Some(pipeline);
                break;
            }
        }

        match selected_pipeline {
            None => {
                warn!(
                    "File {}: No pipeline found",
                    relative_filepath.to_string_lossy().replace('\\', "/")
                );
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

    let mut hash = HashMap::new();
    println!("EXECUTION PLAN");
    println!("==============");
    for job in &jobs {
        hash.insert(job.input_path().clone(), job.output_path().clone());
        println!(
            "{} -> {}",
            job.input_path().display(),
            job.output_path().display()
        );
    }
    println!("==============");
    handlebars_helpers::PROJECT_ROOT
        .set(abs_project_root.to_path_buf())
        .unwrap();
    handlebars_helpers::RESOLVE_TABLE.set(hash).unwrap();

    for job in jobs {
        job.execute(resources.get(&&job.pipeline().name).unwrap_or_else(|| {
            panic!(
                "could not find prefetched resource for pipeline \"{}\"",
                job.pipeline().name
            )
        }))
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
    println!("Use older version such as v0.3.3 for now");

    Ok(())
}
