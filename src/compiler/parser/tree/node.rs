use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
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
}


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
}