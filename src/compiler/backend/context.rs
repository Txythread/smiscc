use std::collections::HashMap;
use derive_new::new;
use uuid::Uuid;
use crate::compiler::data_types::object::{Object, ObjectType};

/// The current compiler state. This includes what variables are available,
/// which registers are in use, etc.
#[derive(new)]
pub struct Context {
    /// All variables, constants, etc. Given by their full name.
    /// The second Uuid refers to the type of the object.
    pub objects: HashMap<Uuid, Uuid>,

    /// All datatypes, including primitive ones
    pub datatypes: HashMap<Uuid, ObjectType>,

    /// The general purpose registers mapped to the objects they contain
    pub reg_map: Vec<String>,
    
    /// The stack offset in the current block
    pub stack_size: usize,
}


impl Context {
    pub fn clear() -> Context {
        Context { objects: HashMap::new(), datatypes: HashMap::new(), reg_map: Vec::new(), stack_size: 0 }
    }
}