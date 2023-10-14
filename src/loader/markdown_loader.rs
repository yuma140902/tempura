use std::borrow::Cow;
use std::io::Read;

use anyhow::Context;

use crate::loader::Loader;
use crate::value::Value;

pub struct MarkdownLoader;

/// Decompose the input into front matter and content portions.
fn decompose_frontmatter(text: &str) -> (Option<String>, Cow<'_, str>) {
    if let Some((front, body)) = matter::matter(text) {
        (Some(front), Cow::Owned(body))
    } else {
        (None, Cow::Borrowed(text))
    }
}

impl Loader for MarkdownLoader {
    fn load(mut reader: impl Read) -> anyhow::Result<Value> {
        let mut buf = String::new();
        reader
            .read_to_string(&mut buf)
            .context("Could not read to string")?;

        let (maybe_yaml, content) = decompose_frontmatter(&buf);
        let yaml = match maybe_yaml {
            Some(yaml) => serde_yaml::from_str(&yaml).context("Invalid YAML in front matter")?,
            None => serde_yaml::Value::default(),
        };

        let mut map = serde_yaml::Mapping::default();
        map.insert(
            serde_yaml::Value::String("content".to_string()),
            serde_yaml::Value::String(content.to_string()),
        );
        map.insert(serde_yaml::Value::String("front".to_string()), yaml);

        Ok(Value::YAML(serde_yaml::Value::Mapping(map)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decompose_frontmatter_normal() {
        let result = decompose_frontmatter(
            r#"---
some: front matter
---
Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."#,
        );

        assert_eq!(result, (Some("some: front matter".to_string()), Cow::Owned("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string())));
    }

    // TODO:
    /*#[test]
        fn decompose_frontmatter_front_only() {
            let result = decompose_frontmatter(
                r#"---
    some: front matter
    ---"#,
            );

            assert_eq!(
                result,
                (
                    Some("some: front matter".to_string()),
                    Cow::Owned("".to_string())
                )
            );
        }*/

    #[test]
    fn decompose_frontmatter_front_only() {
        let result = decompose_frontmatter("some text");

        assert_eq!(result, (None, Cow::Borrowed("some text")));
    }
}
