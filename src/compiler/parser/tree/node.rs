use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use derive_new::*;
use downcast_rs::{Downcast, impl_downcast};
use uuid::Uuid;
use crate::compiler::backend::context::Context;
use crate::compiler::backend::flattener::Instruction;
use crate::compiler::data_types::datatypes_general::Buildable;
use crate::compiler::data_types::integer::IntegerType;
use crate::compiler::data_types::object::{ObjectType, Trait};
use crate::compiler::line_map::{DisplayCodeInfo, DisplayCodeKind, NotificationInfo, TokenPosition};
use crate::compiler::parser::function_meta::{FunctionArgument, FunctionMeta, FunctionStyle};
use crate::compiler::parser::future::CodeFuture;
use crate::compiler::parser::parse_datatype::ParameterDescriptor;
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
    #[allow(dead_code)]
    fn get_future(&self, current: CodeFuture) -> CodeFuture;

    /// Returns all the nodes this node contains if applicable
    #[allow(dead_code)]
    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>>;


    /// Gets the data type if possible. Multiple data types will lead
    /// to problems if it's not clear which datatype is expected.
    /// If there is no return type, None will be returned here.
    fn get_datatypes(&self, all_types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>>;


    /// ### Unpacks Shells Recursively
    ///
    /// If the node is a shell (a node that contains only one subnode and
    /// no data that is not contained in its core as well, e.g. how
    /// [ValueNode] is to [ArithmeticNode] and
    /// [LiteralValueNode].
    ///
    /// Shell nodes get unpacked by unpacking their contents and returning that.
    /// Non-Shell nodes don't unpack themselves. They return themselves.
    fn unpack(&self) -> Box<dyn Node>;

    /// Turns the node into [instructions](Instruction) and potentially an Uuid for
    /// the resulting value if applicable.
    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>);

    /// ### Output Meta Information
    ///
    /// Returns None if no output can be expected from the node.
    /// If the node does deliver output, this outputs whether the value can be mutated
    /// by choice (when the value can't be re-used anyway) or if it needs to be preserved
    /// unless the user is specific about changing it (e.g. in an assignment).
    fn output_is_randomly_mutable(&self) -> Option<bool>;


    /// ### Performs Early Context Changes
    ///
    /// This shouldn't be touched if:
    /// 1. The context changes are only required in later parsing.
    /// 2. The context changes do not need to be available outside the scope of the arguments
    ///
    /// The context changes in here get performed before
    /// [generate_instructions](Self::generate_instructions) gets executed.
    ///
    /// However, it should always be overridden when subnodes are contained as they
    /// need to perform it.
    fn perform_early_context_changes(&mut self, _context: &mut Context) {
        if !self.get_sub_nodes().is_empty() {
            todo!("Missing implementation of perform_early_context_changes in type of: {:#?}. Please read for the documentation comment for this trait function and create an implementation.", self)
        }
    }


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
        println!("getting thing on subnode: {:?}", self.get_sub_node());
        self.get_sub_node().get_datatypes(all_types, context)
    }

    fn unpack(&self) -> Box<dyn Node> {
        self.get_sub_node().unpack()
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        self.unpack().generate_instructions(context)
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        self.get_sub_node().output_is_randomly_mutable()
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
    
    fn get_datatypes(&self, _all_types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>> {
        if let Some(type_) = self.data_type.clone() {
            return Some(vec![type_.clone()]);
        }

        let object_uuid = context.name_map.get(&self.identifier);

        println!("uuid: {object_uuid:?}");


        let type_uuid = context.objects.get(object_uuid?);
        let type_ = context.datatypes.get(type_uuid?);


        Some(vec![type_?.clone()])
    }
    
    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let object_uuid = context.name_map.get(&self.identifier);

        (vec![], object_uuid.copied())
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        Some(false)
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

    fn generate_instructions(&self, _context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        todo!()
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        Some(true)
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

    fn get_datatypes(&self, all_types: Vec<ObjectType>, _context: Context) -> Option<Vec<ObjectType>> {
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

    fn generate_instructions(&self, _context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let uuid = Uuid::new_v4();

        (
            vec![
                Instruction::MoveData(uuid, self.content as i64)
            ]
            ,
            Some(uuid)
        )
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        Some(true)
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

    fn get_datatypes(&self, all_types: Vec<ObjectType>, _context: Context) -> Option<Vec<ObjectType>> {

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

    fn generate_instructions(&self, _context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let uuid = Uuid::new_v4();

        (
            vec![
                Instruction::MoveData(uuid, self.content as i64)
            ]
        ,
            Some(uuid)
        )
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        Some(true)
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
        let self_ = self.clone();
        let a = self_.argument_a.generate_instructions(context);
        let b = self_.argument_b.generate_instructions(context);

        let mut x = a.1.unwrap();

        if self.argument_b.output_is_randomly_mutable() == Some(true) && self.operation.is_commutative() && self.argument_a.output_is_randomly_mutable() != Some(true) {
            return (
                [
                    a.0,
                    b.0,
                    vec![],
                    match self.operation {
                        Operation::Addition => vec![Instruction::Add(b.1.unwrap(), x)],
                        Operation::Subtraction => vec![Instruction::Sub(b.1.unwrap(), x)],
                        Operation::Multiplication => vec![Instruction::Mul(b.1.unwrap(), x)],
                        Operation::Division => vec![Instruction::Div(b.1.unwrap(), x)],
                        Operation::Modulo => vec![Instruction::Mod(b.1.unwrap(), x)],

                        _ => todo!()
                    }
                ].concat(),
                Some(b.1.unwrap())
            )
        }

        if self.argument_a.output_is_randomly_mutable() != Some(true) {
            x = Uuid::new_v4();
        }

        (
            [
                a.0,
                b.0,
                if x!= a.1.unwrap() {vec![
                    Instruction::Move(x, a.1.unwrap())
                ]} else {vec![]},
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

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        Some(self.argument_a.output_is_randomly_mutable()? || self.argument_b.output_is_randomly_mutable()?)
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
    
    fn get_datatypes(&self, _types: Vec<ObjectType>, _context: Context) -> Option<Vec<ObjectType>> {
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
        instructions.append(right_side.0.as_mut());

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

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        None
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

    fn generate_instructions(&self, _context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        todo!()
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        None
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
            let assignment_result = assigned_value.clone().generate_instructions(context).clone();


            let mut assignment_instructions = assignment_result.0;
            instructions.append(&mut assignment_instructions);

            if assigned_value.output_is_randomly_mutable() == Some(true) {
                result_uuid = assignment_result.1;
            } else {
                result_uuid = Some(Uuid::new_v4());
                instructions.push(Instruction::Move(result_uuid.unwrap(), assignment_result.1.unwrap()));
            }
        }

        if result_uuid.is_none() {
            // If there's no uuid, one needs to be assigned still
            result_uuid = Some(Uuid::new_v4());
        }

        let value = self.assigned_value.clone().unwrap();
        let datatypes = value.get_datatypes(context.datatypes.values().cloned().collect(), context.clone());

        println!("value: {:#?}, datatpyes: {datatypes:#?}", value);

        context.objects.insert(result_uuid.unwrap(), datatypes.unwrap()[0].type_uuid);
        context.name_map.insert(self.identifier.clone(), result_uuid.unwrap());

        if self.is_mutable {
            context.mutable_objects.push(result_uuid.unwrap());
        }


        (instructions, None)
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        None
    }
}

/// A node containing multiple lines of code. Note that this node is always an expression,
/// never a statement. Use with caution.
#[derive(Clone, Debug, new)]
pub struct CodeBlockNode {
    position: (usize, TokenPosition),
    pub label: Option<Rc<String>>,
    code: Vec<Rc<dyn Node>>,
}

impl CodeBlockNode {
    pub fn push_code(&mut self, code: Rc<dyn Node>) {
        self.code.push(code);
    }
    
    pub fn append_code(&mut self, code: Vec<Rc<dyn Node>>) {
        self.code.append(&mut code.clone());
    }

    pub fn assign_label(&mut self, context: &mut Context) -> Rc<String> {
        let name: Rc<String> = {if let Some(label) = self.label.clone() { label.clone() } else { context.label_count += 1; Rc::new(String::from("LB") + (context.label_count - 1).to_string().as_str()) }};

        self.label = Some(name.clone());

        name
    }
}

impl Node for CodeBlockNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }

    fn get_future(&self, _current: CodeFuture) -> CodeFuture {
        todo!()
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        self.code.clone()
    }

    fn get_datatypes(&self, _all_types: Vec<ObjectType>, _: Context) -> Option<Vec<ObjectType>> {
        None
    }

    fn unpack(&self) -> Box<dyn Node> {
        todo!()
    }

    fn perform_early_context_changes(&mut self, context: &mut Context) {
        for i in 0..self.code.iter().len() {
            let clone = self.code[i].clone().downcast_rc::<FunctionDeclarationNode>();

            if clone.is_err() {
                continue;
            }

            let mut clone = clone.unwrap().deref().clone();
            clone.perform_early_context_changes(context);
            self.code[i] = Rc::new(clone);
        }
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let mut copy = self.clone();
        let name: Rc<String> = copy.assign_label(context);
        
        let mut instructions: Vec<Instruction> = vec![];

        instructions.push(Instruction::Label(name, false));

        for code in self.code.iter() {
            instructions.append(code.generate_instructions(context).0.as_mut());
        }

        (instructions, None)
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        None
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

    fn get_future(&self, _current: CodeFuture) -> CodeFuture {
        CodeFuture::Never
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![]
    }

    fn get_datatypes(&self, _all_types: Vec<ObjectType>, _: Context) -> Option<Vec<ObjectType>> {
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

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        None
    }
}

#[derive(Clone, Debug, new)]
pub struct FunctionCallNode {
    name: Rc<String>,
    arguments: Vec<Rc<dyn Node>>,
    position: (usize, TokenPosition),
}

impl Node for FunctionCallNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![]
    }

    fn get_datatypes(&self, _all_types: Vec<ObjectType>, context: Context) -> Option<Vec<ObjectType>> {
        let function_metas = context.function_metas;

        let function_meta = function_metas.iter().find(|&x|x.code_name.as_str()==self.name.as_str())?;
        let type_uuid = function_meta.return_type_uuid?;
        let type_ = context.datatypes.get(&type_uuid)?;

        Some(vec![type_.clone()])
    }

    fn unpack(&self) -> Box<dyn Node> {
        todo!()
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let function_metas = context.function_metas.clone();

        let function_meta = function_metas.iter().find(|&x|x.code_name.as_str()==self.name.as_str()).unwrap();
        let asm_fn_name = function_meta.assembly_name.clone();

        let mut return_uuid: Option<Uuid> = None;
        let mut return_uuids: Vec<Uuid> = vec![];

        if function_meta.return_type_uuid.is_some() {
            return_uuid = Some(Uuid::new_v4());
            return_uuids.push(return_uuid.unwrap());
        }


        let mut args: Vec<Uuid> = vec![];
        let mut instructions: Vec<Instruction> = vec![];
        let mut moves: Vec<(Uuid, Uuid)> = vec![];

        if self.arguments.len() != function_meta.arguments.len() {
            let display_info = DisplayCodeInfo::new(
                self.position.0 as u32,
                self.position.1.start as u32,
                (self.position.1.start + self.position.1.length) as i32,
                vec![],
                DisplayCodeKind::InitialError
            );

            let name = function_meta.code_name.clone();
            let expected_args = function_meta.arguments.len();
            let actual_args = self.arguments.len();

            let notification = NotificationInfo::new(
                "Wrong Amount of Arguments".to_string(),
                format!("Function '{}' expected {} argument(s) but received {}.", name, expected_args, actual_args),
                vec![display_info]
            );

            context.line_map.display_error(notification);
        }

        for arg in self.arguments.iter() {
            let arg_result = arg.generate_instructions(context);

            instructions.append(&mut arg_result.0.clone());

            if arg.output_is_randomly_mutable() == Some(true) {
                args.push(arg_result.1.unwrap());
            } else {
                let arg_uuid = Uuid::new_v4();
                args.push(arg_uuid);
                moves.push((arg_uuid, arg_result.1.unwrap()));
            }
        }

        for move_ in moves {
            instructions.push(Instruction::Move(move_.0, move_.1));
        }


        (
            [
                instructions,
                vec![Instruction::Call(asm_fn_name, args, return_uuids)],
            ].concat()
            ,
            return_uuid
        )
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        None
    }
}

#[derive(Clone, Debug, new)]
pub struct CodeBlockArray {
    pub position: (usize, TokenPosition),
    pub code_blocks: Vec<CodeBlockNode>
}

impl Node for CodeBlockArray {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }
    
    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }
    
    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![]
    }
    
    fn get_datatypes(&self, _all_types: Vec<ObjectType>, _context: Context) -> Option<Vec<ObjectType>> {
        None
    }

    fn perform_early_context_changes(&mut self, context: &mut Context) {
        for i in 0..self.code_blocks.len() {
            self.code_blocks[i].perform_early_context_changes(context);
        }
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new((*self).clone())
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let mut instructions: Vec<Instruction> = vec![];

        for code_block in self.code_blocks.iter() {
            instructions.append(code_block.generate_instructions(context).0.as_mut());
        }

        (
            instructions,
            None
        )
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        None
    }
}


#[derive(Clone, Debug, new)]
pub struct FunctionDeclarationNode {
    pub position: (usize, TokenPosition),
    pub name: Rc<String>,
    pub block: Rc<CodeBlockNode>,
    pub parameters: Rc<Vec<ParameterDescriptor>>,

    /// The generated parameters
    #[new(default)]
    parameter_function_args: Vec<FunctionArgument>
}

impl Node for FunctionDeclarationNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }
    
    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }
    
    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![self.block.clone()]
    }
    
    fn get_datatypes(&self, _all_types: Vec<ObjectType>, _context: Context) -> Option<Vec<ObjectType>> {
        None
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new((*self).clone())
    }

    fn perform_early_context_changes(&mut self, context: &mut Context) {
        let mut block = self.block.deref().clone();
        let asm_label = block.assign_label(context);
        self.parameter_function_args = self.parameters.iter().map(|x|x.generate_function_argument()).collect();

        context.function_metas.push(
            FunctionMeta::new(
                self.name.deref().clone(),
                asm_label.deref().clone(),
                FunctionStyle::C,
                None,
                self.parameter_function_args.clone()
            )
        );



        block.perform_early_context_changes(context);
        self.block = Rc::new(block);
    }

    fn generate_instructions(&self, context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        let block = self.block.deref().clone();
        let parameter_uuids = self.parameter_function_args.iter().map(|x|x.own_uuid);
        let mut instructions: Vec<Instruction> = parameter_uuids.enumerate().map(|x|Instruction::ReceiveArgument(x.1, x.0 as u8)).collect();

        // Update the context
        for i in 0..self.parameters.len() {
            if let Some(name) = self.parameters[i].internal_name.clone() {
                let uuid = self.parameter_function_args[i].own_uuid;
                let type_uuid = self.parameter_function_args[i].type_uuid;


                context.objects.insert(uuid, type_uuid);
                context.name_map.insert(name.clone(), uuid);

                println!("inserted object named {name} with uuid {uuid} as having the type: {type_uuid}")
            }
        }

        instructions.append(&mut block.generate_instructions(context).0.to_vec());


        instructions.push(
            Instruction::FunctionEnd
        );


        (
            instructions,
            None
        )
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        None
    }
}

#[derive(Clone, Debug, new)]
pub struct ArgumentsNode<T> {
    pub position: (usize, TokenPosition),
    pub args: Rc<Vec<T>>
}

impl<T: 'static + Debug + Clone> Node for ArgumentsNode<T> {
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
        Box::new((*self).clone())
    }

    fn generate_instructions(&self, _: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        (vec![], None)
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        None
    }
}


#[derive(Clone, Debug, new)]
pub struct StringLiteralNode {
    pub position: (usize, TokenPosition),
    pub string: Rc<String>
}

impl Node for StringLiteralNode {
    fn get_position(&self) -> (usize, TokenPosition) {
        self.position.clone()
    }

    fn get_future(&self, current: CodeFuture) -> CodeFuture {
        current
    }

    fn get_sub_nodes(&self) -> Vec<Rc<dyn Node>> {
        vec![]
    }

    fn get_datatypes(&self, all_types: Vec<ObjectType>, _context: Context) -> Option<Vec<ObjectType>> {
        Some(vec![all_types.iter().find(|&x| x.traits.contains(&Trait::new(Trait::BASIC_STRING.to_string())))?.clone()])
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new((*self).clone())
    }

    fn generate_instructions(&self, _context: &mut Context) -> (Vec<Instruction>, Option<Uuid>) {
        todo!()
    }

    fn output_is_randomly_mutable(&self) -> Option<bool> {
        Some(true)
    }
}