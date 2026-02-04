use std::collections::HashMap;
use derive_new::new;
use uuid::Uuid;
use std::rc::Rc;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::LineMap;
use crate::compiler::optimization::OptimizationFlags;
use crate::compiler::parser::function_meta::FunctionMeta;

/// The current compiler state. This includes what variables are available,
/// which registers are in use, etc.
#[derive(new, Debug, Clone)]
pub struct Context {
    /// All variables, constants, etc. Given by their full name.
    /// The second Uuid refers to the type of the object.
    pub objects: HashMap<Uuid, Uuid>,

    /// The list of all objects that are marked as mutable. Given
    /// by the same Uuid with which they are identified in the
    /// [objects hash map](Self::objects).
    pub mutable_objects: Vec<Uuid>,

    pub opt_flags: Rc<OptimizationFlags>,

    pub line_map: LineMap,
    
    /// The objects, mapped by their full name. The Uuid refers
    /// to the key in [the objects hash map](Self::objects)
    pub name_map: HashMap<String, Uuid>,

    /// All datatypes, including primitive ones
    /// (key: their type UUID, contents: entire object type)
    pub datatypes: HashMap<Uuid, ObjectType>,

    /// Stores the values that are known at compile time.
    /// Those values can include variables, if it is certain that they have not been mutated yet.
    /// The value is in its assembly representation, meaning bools are 0 or 1, integers got their
    /// raw values. Anything over the bounds of an i64 (which can be interpreted as an u64) can't
    /// be optimized this way, unfortunately.
    pub known_values: HashMap<Uuid, i64>,


    /// All the meta information about all functions, or at least the
    /// ones currently in scope.
    pub function_metas: Vec<FunctionMeta>,

    /// The amount of labels generated, useful for creating new label names
    /// when none are forced by the user.
    pub label_count: usize
}


impl Context {
    pub fn clear(line_map: LineMap, opt_flags: Rc<OptimizationFlags>) -> Context {
        Context { objects: HashMap::new(), mutable_objects: Vec::new(), opt_flags, line_map, name_map: HashMap::new(), datatypes: HashMap::new(), known_values: HashMap::new(), function_metas: Vec::new(), label_count: 0 }
    }
    
    pub fn generate_label(&mut self) -> Rc<String>{
        self.label_count += 1;
        Rc::new(String::from("LB") + (self.label_count - 1).to_string().as_str())   
    }
}