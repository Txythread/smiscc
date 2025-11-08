use std::rc::Rc;
use crate::compiler::data_types::data_types::Buildable;
use crate::compiler::data_types::integer::IntegerType;
use crate::compiler::data_types::object::{ObjectType, Trait};
use crate::compiler::line_map::TokenPosition;
use crate::compiler::parser::future::CodeFuture;
use crate::util::operator::Operation;

/// A trait requiring basic functionality for any node in an abstract
/// syntax tree.
trait Node {
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
    fn get_datatypes(&self, all_types: Vec<(ObjectType, Box<dyn Buildable>)>) -> Option<Vec<ObjectType>>;


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
}

/// Any node that has a type that can be resolved to a value.
/// **Note**: This is used in the parser to group values and parse
/// expressions as arguments into statements.
pub enum ValueNode {
    Arithmetic(ArithmeticNode),
    Literal(LiteralValueNode)
}

impl ValueNode {
    /// Gets the node the value node contains
    fn get_sub_node(&self) -> Box<dyn Node> {
        match self {
            ValueNode::Arithmetic(node) => Box::new(node.clone()),
            ValueNode::Literal(node) => Box::new(node.clone()),
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

    fn get_datatypes(&self, all_types: Vec<(ObjectType, Box<dyn Buildable>)>) -> Option<Vec<ObjectType>> {
        self.get_sub_node().get_datatypes(all_types)
    }

    fn unpack(&self) -> Box<dyn Node> {
        self.get_sub_node().unpack()
    }
}

#[derive(Clone)]
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

    fn get_datatypes(&self, all_types: Vec<(ObjectType, Box<dyn Buildable>)>) -> Option<Vec<ObjectType>> {
        self.get_sub_node().get_datatypes(all_types)
    }

    fn unpack(&self) -> Box<dyn Node> {
        self.get_sub_node().unpack()
    }
}

#[derive(Clone)]
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

    fn get_datatypes(&self, all_types: Vec<(ObjectType, Box<dyn Buildable>)>) -> Option<Vec<ObjectType>> {
        for type_ in all_types.iter().clone() {
            // Integer?
            if type_.0.has_trait(Trait::BOOLEAN_COMPATIBLE) {
                return Some(vec![type_.0.clone()]);
            }
        }

        panic!("Can't Find Boolean Data Type");
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct IntegerLiteralNode {
    content: i128,
    kind: Option<IntegerType>,
    position: (usize, TokenPosition),
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

    fn get_datatypes(&self, all_types: Vec<(ObjectType, Box<dyn Buildable>)>) -> Option<Vec<ObjectType>> {

        if let Some(kind) = self.kind.clone() {
            for type_ in all_types.iter().clone() {
                if type_.1.get_name() == kind.get_name() {
                    return Some(vec![type_.0.clone()]);
                }
            }
        }

        // Get all integer types
        let mut compatible_types: Vec<ObjectType> = vec![];

        for type_ in all_types.iter().clone() {
            // Integer?
            if type_.0.has_trait(Trait::INTEGER) {
                compatible_types.push(type_.0.clone());
            }
        }

        Some(compatible_types)
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}

/// A [node](Node) in the syntax tree that contains an arithmetic operation
/// and it's arguments.
///
/// This could represent calculations like `a + b` where a and b could be
/// either basic or complex types.
#[derive(Clone)]
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

    fn get_datatypes(&self, all_types: Vec<(ObjectType, Box<dyn Buildable>)>) -> Option<Vec<ObjectType>> {
        if self.operation.is_boolean() {
            // Find the boolean type in the types
            for type_ in all_types.iter().clone() {
                if type_.0.has_trait(Trait::BOOLEAN_COMPATIBLE){
                    return Some(vec![type_.0.clone()]);
                }
            }
        }

        // It should just be the data type of the first and second argument, which should be equivalent
        self.argument_a.get_datatypes(all_types)
    }

    fn unpack(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}