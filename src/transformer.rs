use crate::{store::Store, Value};

mod template_renderer;

pub use template_renderer::*;

pub trait Transformer {
    fn transform(&self, value: &Value, store: &Store) -> anyhow::Result<Value>;
}
