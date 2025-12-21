use uuid::Uuid;
use crate::compiler::backend::context::Context;
use crate::compiler::parser::tree::node::Node;

pub fn flatten(line: Box<dyn Node>, context: Context) -> Vec<Instruction> {
    
}

/// A representation where variables still have their old names, but
/// unlisted data just gets an UUID assigned.
#[derive(Clone)]
pub enum Instruction {
    /// Move (1) into (0)
    Move(Uuid, Uuid),
    /// Move immediate value (1) into 0
    MoveData(Uuid, i64),
    
    Add(Uuid, Uuid),
    
    /// Load (size of (datatype (2))) bytes of object at (1) into (0)
    Load(Uuid, Uuid, Uuid),
    Store(Uuid, Uuid, Uuid),
}