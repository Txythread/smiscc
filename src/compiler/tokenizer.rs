use crate::compiler::line_map::{LineMap, TokenPosition};
use crate::compiler::object::Object;

/// Turn the split string into tokens ("classify" them)
///
/// A simple `"10"` would be turned into an object with the  or whatever the default number system is set to.
pub fn tokenize(separated: Vec<Vec<String>>, line_map: &mut LineMap) {

}


/// **Note:** A token always includes the position from
/// which it stems. This is important for producing debug
/// information about the user's code.
#[derive(Clone)]
pub enum Token {
    /// A static/constant object that can be accessed instantly.
    Object(Object, TokenPosition),

    /// A variable that might be changed. It therefore can't be
    /// optimized away.
    Variable(Object, TokenPosition),

    /// ### Equation operation
    /// An equation operation that sets a variable to another token.
    /// This token should be an object of the same type in the end.
    Set(Object, Box<Token>, TokenPosition),

    /// ### A block of tokens/code
    /// This might, for example, be a function body, a loop body or an if body.
    Block(Vec<Token>, TokenPosition),

    UnspecifiedString(String, TokenPosition),
}