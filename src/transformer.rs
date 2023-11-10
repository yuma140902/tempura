use crate::{store::Store, Value};

mod json_path_query;
mod json_path_query_all;
mod template_renderer;

pub use json_path_query::*;
pub use json_path_query_all::*;
pub use template_renderer::*;

pub trait Transformer {
    fn transform(&self, value: &Value, store: &Store) -> anyhow::Result<Value>;
}
