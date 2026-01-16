use std::rc::Rc;
use uuid::Uuid;
use crate::compiler::backend::context::Context;
use crate::compiler::parser::tree::node::Node;

pub fn flatten(line: Rc<dyn Node>, context: &mut Context) -> Vec<Instruction> {




    let result = line.generate_instructions(context);
    let mut instructions = result.0;
    let _obj_uuid = result.1;


    // Find the last occurrence of an object's usage in the
    // assembly and insert drop statements afterward.

    let mut objects: Vec<Uuid> = vec![];

    for i in 1..instructions.len() {
        let i = instructions.len() - i;

        // Find objects that are not needed after this point
        for object in instructions[i].get_objects() {
            if !objects.contains(&object) {
                objects.push(object);

                // Drop the object at this point
                instructions.insert(i + 1, Instruction::Drop(object))

            }
        }

        /*/ Find objects that are not needed at this point
        for object in instructions[i].get_overridden() {
            if let Some(removal_index) = objects.iter().position(|x| *x==object) {
                objects.remove(removal_index);
            }
        }
         // */
    }

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
    Sub(Uuid, Uuid),
    Mul(Uuid, Uuid),
    Div(Uuid, Uuid),
    Mod(Uuid, Uuid),
    
    /// Load (size of (datatype (2))) bytes of object at (1) into (0)
    Load(Uuid, Uuid, u8),
    Store(Uuid, Uuid, u8),

    /// Removes an object from the list of objects that need to be
    /// maintained. This will not clean the heap if this is a pointer.
    Drop(Uuid),

    /// Exit the current program while returning the given object
    Exit(Uuid),

    /// Calls a function given its assembly name.
    /// This doesn't only jump, it performs a subroutine, it branches,
    /// calls a function, however you might want to call it.
    Call(/* assembly name: */String, /* inputs: */Vec<Uuid>, /* outputs: */Vec<Uuid>),

    /// Defines a label at the current code position
    /// This label is globalized if the bool is true
    Label(Rc<String>, bool),
    
    
    ReceiveArgument(Uuid, u8),
    
    FunctionEnd,
}

impl Instruction {
    pub fn get_objects(&self) -> Vec<Uuid> {
        match self {
            Instruction::Move(a, b) => vec![*a, *b],
            Instruction::Add(a, b) => vec![*a, *b],
            Instruction::Sub(a, b) => vec![*a, *b],
            Instruction::Mul(a, b) => vec![*a, *b],
            Instruction::Div(a, b) => vec![*a, *b],
            Instruction::Mod(a, b) => vec![*a, *b],
            Instruction::Load(a, b, _) => vec![*a, *b],
            Instruction::Store(a, b, _) => vec![*a, *b],
            Instruction::Drop(a) => vec![*a],
            Instruction::MoveData(a, _) => vec![*a],
            Instruction::Exit(a) => vec![*a],
            Instruction::Call(_, args, outs) => vec![args.clone(), outs.clone()].concat(),
            Instruction::Label(_, _) | Instruction::FunctionEnd => vec![],
            Instruction::ReceiveArgument(_, _) => { vec![] }
        }
    }

    /// Gets all the objects that are overridden, meaning their value
    /// doesn't matter for this step and what it contained could be
    /// discarded before.
    #[allow(dead_code)]
    pub fn get_overridden(&self) -> Vec<Uuid> {
        match self {
            Instruction::Move(a, _) => vec![*a],
            Instruction::MoveData(a, _) => vec![*a],
            _ => vec![]
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum InstructionMeta {
    MoveReg,
    MoveImm,

    AddReg,
    AddImm,
    SubReg,
    SubImm,
    MulReg,
    DivReg,
    ModReg,

    Load,
    Store,

    StackLoad,
    StackStore,

    Exit,

    Call,

    Label
}