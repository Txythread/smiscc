use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use derive_new::*;
use downcast_rs::{Downcast, impl_downcast};
use uuid::Uuid;
use crate::compiler::backend::context::Context;
use crate::compiler::backend::flattener::Instruction;
use crate::compiler::data_types::data_types::Buildable;
use crate::compiler::data_types::integer::IntegerType;
use crate::compiler::data_types::object::{ObjectType, Trait};
use crate::compiler::line_map::{DisplayCodeInfo, DisplayCodeKind, NotificationInfo, TokenPosition};
use crate::compiler::parser::future::CodeFuture;
use crate::util::operator::Operation;

/// A trait requiring basic functionality for any node in an abstract
/// syntax tree.
pub trait Node: Debug + Downcast {
    /// Gets the position of the entire node in the line map.
    /// The first value (.0) refers to the line number. The second one
    /// is the absolute token position (in characters, start to end)
    fn get_position(&self) -> (usize, TokenPosition);

    /// Gets a change of future if necessary (like return delivering
    /// no future whatsoever).
    fn get_future(&self, current: CodeFuture) -> CodeFuture;

    /// Returns all the nodes this node contains if applicable
    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>>;


    /// Gets the data type if possible. Multiple data types will lead
    /// to problems if it's not clear which datatype is expected.
    /// If there is no return type, None will be returned here.
    fn get_datatypes(&self, all_types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>>;


    /// ### Unpacks Shells Recursively
    ///
    /// If the node is a shell (a node that contains only one subnode and
    /// no data that is not contained in its core as well, e.g. how
    /// [ValueNode](ValueNode) is to [ArithmeticNode](ArithmeticNode) and
    /// [LiteralValueNode](LiteralValueNode)).
    ///
    /// Shell nodes get unpacked by unpacking their contents and returning that.
    /// Non-Shell nodes don't unpack themselves. They return themselves.
    fn unpack(&self) -> Box<dyn Node>;

    /// Turns the node into [instructions](Instruction) and potentially an Uuid for
    /// the resulting value if applicable.
    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>);
}
impl_downcast!(Node);

/// Any node that has a type that can be resolved to a value.
/// **Note**: This is used in the parser to group values and parse
/// expressions as arguments into statements.
#[derive(Clone, Debug)]
pub enum ValueNode {
    Arithmetic(ArithmeticNode),
    Literal(LiteralValueNode),
    Identifier(IdentifierNode),
}


impl ValueNode {
    /// Gets the node the value node contains
    fn get_sub_node(&self) -> Box<dyn Node> {
        match self {
            ValueNode::Arithmetic(node) => Box::new(node.clone()),
            ValueNode::Literal(node) => Box::new(node.clone()),
            ValueNode::Identifier(node) => Box::new(node.clone()),
        }
    }
}

impl Node for ValueNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.get_sub_node().get_position()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        self.get_sub_node().get_future(current)
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        self.get_sub_node().get_sub_nodes()
    }

    fn get_datatypes(&self, all_types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>> {
        self.get_sub_node().get_datatypes(all_types, context)
    }

    fn unpack(&self) -> Box<dyn Node> {
        self.get_sub_node().unpack()
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        self.unpack().generate_instructions(context)
    }
}

/// A node that contains an identifier such as a variable or type name.  
/// **Note:** Where this is stored might make it refer to different things
/// (declarations are different from calls)
#[derive(Clone, Debug, new, PartialEq)]
pub struct IdentifierNode {
    /// The literal identifier contained.  
    /// This could be a shortened version.
    pub identifier: String,
    
    /// The datatype (if determined already)
    data_type: Option<ObjectType>,
    
    position: (usize, TokenPosition)
}

impl Node for IdentifierNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }
    
    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }
    
    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![]
    }
    
    fn get_datatypes(&self, all_types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>> {
        if let Some(type_) = self.data_type.clone() {
            return Some(vec![type_.clone()]);
        }

        let object_uuid = context.name_map.get(&self.identifier);
        let type_uuid = context.objects.get(object_uuid?);
        let type_ = context.datatypes.get(type_uuid?);

        Some(vec![type_?.clone()])
    }
    
    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let object_uuid = context.name_map.get(&self.identifier);

        (vec![], object_uuid.map(|uuid| uuid.clone()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValueNode {
    Integer(IntegerLiteralNode),
    Boolean(BoolLiteralNode)
}

impl LiteralValueNode {
    fn get_sub_node(&self) -> Box<dyn Node> {
        match self {
            LiteralValueNode::Integer(node) => Box::new(node.clone()),
            LiteralValueNode::Boolean(node) => Box::new(node.clone()),
        }
    }
}

impl Node for LiteralValueNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.get_sub_node().get_position()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        self.get_sub_node().get_future(current)
    }


    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        self.get_sub_node().get_sub_nodes()
    }

    fn get_datatypes(&self, all_types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>> {
        self.get_sub_node().get_datatypes(all_types, context)
    }

    fn unpack(&self) -> Box<dyn Node> {
        self.get_sub_node().unpack()
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        todo!()
    }
}

#[derive(Clone, Debug, new, PartialEq)]
pub struct BoolLiteralNode {
    content: bool,
    position: (usize, TokenPosition),
}

impl Node for BoolLiteralNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        Vec::new()
    }

    fn get_datatypes(&self, all_types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>> {
        for type_ in all_types.iter().clone() {
            // Integer?
            if type_.has_trait(Trait::BOOLEAN_COMPATIBLE) {
                return Some(vec![type_.clone()]);
            }
        }

        panic!("Can't Find Boolean Data Type");
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let uuid = Uuid::new_v4();

        (
            vec![
                Instruction::MoveData(uuid, self.content as i64)
            ]
            ,
            Some(uuid)
        )
    }
}

#[derive(Clone, Debug, new, PartialEq)]
pub struct IntegerLiteralNode {
    pub(crate) content: i128,
    pub(crate) kind: Option<IntegerType>,
    pub(crate) position: (usize, TokenPosition),
}

impl Node for IntegerLiteralNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![]
    }

    fn get_datatypes(&self, all_types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>> {

        if let Some(kind) = self.kind.clone() {
            for type_ in all_types.iter().clone() {
                if type_.name == kind.get_name() {
                    return Some(vec![type_.clone()]);
                }
            }
        }

        // Get all integer types
        let mut compatible_types: Vec<ObjectType> = vec![];

        for type_ in all_types.iter().clone() {
            // Integer?
            if type_.has_trait(Trait::INTEGER) {
                compatible_types.push(type_.clone());
            }
        }

        Some(compatible_types)
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let uuid = Uuid::new_v4();

        (
            vec![
                Instruction::MoveData(uuid, self.content as i64)
            ]
        ,
            Some(uuid)
        )
    }
}

/// A [node](Node) in the syntax tree that contains an arithmetic operation
/// and it's arguments.
///
/// This could represent calculations like `a + b` where a and b could be
/// either basic or complex types.
#[derive(Clone, Debug, new)]
pub struct ArithmeticNode {
    /// The [operation](Operation) this node should perform (e.g. a **-** b).
    operation: Operation,

    /// The first argument of the operation (e.g. **a** - b)
    argument_a: Rc<dyn Node>,

    /// The second argument of the operation (e.g. a - **b**)
    argument_b: Rc<dyn Node>,

    position: (usize, TokenPosition),
}

impl Node for ArithmeticNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![self.argument_a.clone(), self.argument_b.clone()]
    }

    fn get_datatypes(&self, all_types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>> {
        if self.operation.is_boolean() {
            // Find the boolean type in the types
            for type_ in all_types.iter().clone() {
                if type_.has_trait(Trait::BOOLEAN_COMPATIBLE){
                    return Some(vec![type_.clone()]);
                }
            }
        }

        // It should just be the data type of the first and second argument, which should be equivalent
        self.argument_a.get_datatypes(all_types, context)
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let a = self.argument_a.generate_instructions(context);
        let b = self.argument_b.generate_instructions(context);

        let x = Uuid::new_v4();


        (
            vec![
                a.0,
                b.0,
                vec![
                    Instruction::Move(x, a.1.unwrap())
                ],
                match self.operation {
                    Operation::Addition => vec![Instruction::Add(x, b.1.unwrap())],
                    Operation::Subtraction => vec![Instruction::Sub(x, b.1.unwrap())],
                    Operation::Multiplication => vec![Instruction::Mul(x, b.1.unwrap())],
                    Operation::Division => vec![Instruction::Div(x, b.1.unwrap())],
                    Operation::Modulo => vec![Instruction::Mod(x, b.1.unwrap())],

                    _ => todo!()
                }
            ].concat(),
            Some(x)
        )
    }
}

#[derive(Clone, Debug, new)]
pub struct AssignmentNode {
    left_side: Rc<IdentifierNode>,
    right_side: Rc<dyn Node>,
    position: (usize, TokenPosition),
}


impl Node for AssignmentNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }
    
    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }
    
    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![]
    }
    
    fn get_datatypes(&self, types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>> {
        None
    }
    
    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let mut instructions: Vec<Instruction> = vec![];
        let mut left_side = self.left_side.generate_instructions(context);
        let mut right_side = self.right_side.generate_instructions(context);

        instructions.append(left_side.0.as_mut());
        instructions.append(&mut right_side.0.as_mut());

        // Look if the left side is in the list of mutable objects
        if !context.mutable_objects.contains(&left_side.1.unwrap()) {
            let display_info = DisplayCodeInfo::new(
                self.position.0 as u32,
                self.position.1.start as u32,
                self.position.1.start as i32 + self.position.1.length as i32,
                vec![
                    "*hint:* consider making this variable mutable".to_string()
                ],
                DisplayCodeKind::InitialError
            );

            let notification = NotificationInfo::new(
                "Attempt To Modify Immutable Variable".to_string(),
                "This assignment tries to alter a left side that is immutable".to_string(),
                vec![display_info],
            );

            context.line_map.display_error(notification);

            return (instructions, None);
        }


        instructions.push(Instruction::Move(left_side.1.unwrap(), right_side.1.unwrap()));


        (instructions, None)
    }
}

#[derive(Clone, Debug, new)]
pub struct AssignmentSymbolNode {
    position: (usize, TokenPosition),
}


impl Node for AssignmentSymbolNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![]
    }

    fn get_datatypes(&self, _: Vec<ObjectType>, _: Context) -> Option<Vec<ObjectType>> {
        None
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        todo!()
    }
}

#[derive(Clone, Debug, new)]
pub struct LetNode {
    identifier: String,
    assigned_value: Option<Rc<dyn Node>>,
    is_mutable: bool,
    position: (usize, TokenPosition),
}

impl Node for LetNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![]
    }

    fn get_datatypes(&self, _: Vec<ObjectType>, _: Context) -> Option<Vec<ObjectType>> {
        None
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let mut instructions: Vec<Instruction> = vec![];
        let mut result_uuid: Option<Uuid> = None;

        if let Some(assigned_value) = self.assigned_value.clone() {
            let assignment_result = self.assigned_value.clone().unwrap().generate_instructions(context);
            let mut assignment_instructions = assignment_result.0;
            instructions.append(&mut assignment_instructions);
            result_uuid = assignment_result.1;
        }

        if result_uuid.is_none() {
            // If there's no uuid, one needs to be assigned still
            result_uuid = Some(Uuid::new_v4());
        }

        (*context).objects.insert(result_uuid.unwrap(), self.assigned_value.clone().unwrap().get_datatypes(context.datatypes.values().map(|x|x.clone()).collect(), context.clone()).unwrap()[0].type_uuid);
        (*context).name_map.insert(self.identifier.clone(), result_uuid.unwrap());

        if self.is_mutable {
            (*context).mutable_objects.push(result_uuid.unwrap());
        }


        (instructions, None)
    }
}

/// A node containing multiple lines of code. Note that this node is always an expression,
/// never a statement. Use with caution.
#[derive(Debug, new)]
pub struct CodeBlockNode {
    position: (usize, TokenPosition),
    code: Vec<Rc<dyn Node>>,
}


impl Node for CodeBlockNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        todo!()
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        self.code.clone()
    }

    fn get_datatypes(&self, all_types: Vec<ObjectType>, _: Context) -> Option<Vec<ObjectType>> {
        None
    }

    fn unpack(&self) -> Box<dyn Node> {
        todo!()
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let mut instructions: Vec<Instruction> = vec![];

        for code in self.code.iter() {
            instructions.append(code.generate_instructions(context).0.as_mut());
        }

        (instructions, None)
    }
}


/// A node that exits the process. No dark web stuff.
#[derive(Debug, new)]
pub struct ExitNode {
    return_value: Rc<dyn Node>,
    position: (usize, TokenPosition),
}


impl Node for ExitNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        todo!()
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![]
    }

    fn get_datatypes(&self, all_types: Vec<ObjectType>, _: Context) -> Option<Vec<ObjectType>> {
        None
    }

    fn unpack(&self) -> Box<dyn Node> {
        todo!()
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let mut instructions: Vec<Instruction> = vec![];
        let mut return_value = self.return_value.generate_instructions(context);

        instructions.append(return_value.0.as_mut());

        instructions.push(Instruction::Exit(return_value.1.unwrap()));

        (instructions, None)
    }
}