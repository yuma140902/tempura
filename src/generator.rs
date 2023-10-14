use std::fmt::Debug;
use std::{collections::HashMap, path::Path};

use echo_generator::EchoGenerator;
use handlebars_generator::HandlebarsGenerator;

use crate::project_config::GeneratorRule;

pub mod echo_generator;
pub mod handlebars_generator;

pub trait Generator: Debug {
    fn generate(
        &self,
        input_filepath: &Path,
        output_filepath: &Path,
        project_root: &Path,
        rule: &GeneratorRule,
    ) -> GeneratorResult;
}

//pub enum GeneratorResult {
//   Success,
//  Fail,
//}

type GeneratorResult = Result<(), anyhow::Error>;

pub fn get_generators(project_root: impl AsRef<Path>) -> HashMap<String, Box<dyn Generator>> {
    let mut map: HashMap<String, Box<dyn Generator>> = HashMap::new();
    map.insert(
        "handlebars".to_owned(),
        Box::new(HandlebarsGenerator::new(&project_root)),
    );
    map.insert("echo".to_owned(), Box::new(EchoGenerator));
    map
}
