use std::rc::Rc;
use uuid::Uuid;
use crate::compiler::backend::context::Context;
use crate::compiler::data_types::data_types::Buildable;
use crate::compiler::data_types::integer::IntegerType;
use crate::compiler::parser::tree::node::Node;

pub fn flatten(line: Rc<dyn Node>, context: &mut Context) -> Vec<Instruction> {
    // Generate the datatypes
    let u32_ = IntegerType::Unsigned32BitInteger;
    let u32_type = u32_.build_type();

    context.datatypes.insert(u32_type.type_uuid, u32_type);


    let result = line.generate_instructions(context);
    let instructions = result.0;
    let obj_uuid = result.1;

    println!("Node generated uuid: {:#?} and instructions: {:?}", obj_uuid, instructions);

    instructions
}

/// A representation where variables still have their old names, but
/// unlisted data just gets an UUID assigned.
#[derive(Clone, Debug)]
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