use crate::compiler::data_types::data_types::{BuildResult, Buildable};
use crate::compiler::data_types::object::{ObjectType, Trait};
use crate::compiler::tokenization::token::Token;

pub struct Boolean {}

impl Boolean {
    /// The name of the type referring to the boolean type in the user's code
    pub const BOOLEAN_KEYWORD: &str = "bool";

    pub fn new() -> Self {
        Self {}
    }
}

impl Buildable for Boolean {
    fn build(&self, _tokens: Vec<Token>, _parent_type: ObjectType) -> BuildResult {
        todo!()
    }

    fn build_type(&self) -> ObjectType {
        let uuid = uuid::Uuid::new_v4();

        let mut type_ = ObjectType::new("bool".to_string(), uuid);

        type_.add_complex_trait(Trait::SIZED, vec!["1".to_string()]);
        type_.add_trait(Trait::VALUE_TYPE);
        type_.add_trait(Trait::BOOLEAN_COMPATIBLE);


        type_
    }

    fn get_name(&self) -> String {
        Self::BOOLEAN_KEYWORD.to_string()
    }
}