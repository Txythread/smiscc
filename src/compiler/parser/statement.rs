use std::fmt::Debug;
use std::rc::Rc;
use crate::compiler::parser::parse::ExpressionKind;
use crate::compiler::parser::tree::node::Node;
use crate::config::tokenization_options::Keyword;

pub trait Statement: Debug {
    /// Gets the [keyword](Keyword) that marks this statement.
    fn get_affiliated_keyword(&self) -> Option<Keyword>;
    
    /// ### Gets the header format.
    /// 
    /// This tells the parser what to expect.  
    /// The "header" refers to the part that can make all context changes,
    /// but does not actually operate on/change the value. For example, this 
    /// could be `let abc: u32;`. This has context actions, but nothing is 
    /// allocated on the stack or the heap. No values get moved into registers
    /// either.
    fn get_header_format(&self) -> Vec<(ExpressionKind, bool)>;
    
    
    /// ### Gets the body format
    /// 
    /// This tells the parser what to expect.  
    /// The "body" refers to everything that's part of the statement but outside
    /// the header. The header is defined [here](Self::get_header_format).
    fn get_body_format(&self) -> Vec<(ExpressionKind, bool)>;
    
    /// ### Generates a node using the header
    /// 
    /// **Note:** The affiliated keyword starting this statement is not expected to
    /// be transmitted.
    fn generate_header_node(&self, arguments: Vec<Rc<dyn Node>>) -> Option<Rc<dyn Node>>;

    /// ### Generates a node for the entire statement
    /// 
    /// This includes both header and body.  
    /// **Note:** The affiliated keyword starting this statement is not expected to
    /// be transmitted.
    fn generate_entire_node(&self, arguments: Vec<Rc<dyn Node>>) -> Option<Rc<dyn Node>>;
}