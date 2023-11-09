mod entry;
mod step;

use serde::{Deserialize, Serialize};

pub use entry::*;
pub use step::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Pipeline {
    pub entry: Entry,
    pub steps: Vec<Step>,
    // TODO: もっと柔軟に出力パスを指定できるようにする
    pub output_extension: Option<String>,
}
