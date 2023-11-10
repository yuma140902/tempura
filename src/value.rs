use handlebars::Template;

/// [`Value`] is data read from a file by the [`Loader`](crate::loader::Loader).
/// Conceptually, it is the same as JSON.
/// It is hierarchical and has types such as object, string, and numeric.

#[derive(Debug, Clone)]
pub enum Value {
    Bytes(Vec<u8>),
    Template(Template),
    JSON(serde_json::Value),
}

impl Value {
    pub fn get_type_name(&self) -> &'static str {
        match self {
            Value::Bytes(_) => "bytes",
            Value::JSON(json) => get_json_type_name(json),
            Value::Template(_) => "template",
        }
    }
}

fn get_json_type_name(json: &serde_json::Value) -> &'static str {
    match json {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "bool",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}
