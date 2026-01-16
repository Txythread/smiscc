use derive_new::new;
use std::rc::Rc;
use crate::compiler::data_types::object::ObjectType;
use crate::compiler::line_map::LineMap;
use crate::compiler::parser::statements::Statements;
use crate::compiler::parser::tree::node::CodeBlockNode;
use crate::compiler::tokenization::token::Token;


/// Contains information about the parsing state
///
/// Some parse_xyz(...) functions might expect more arguments,
/// but this structure contains most of the commonly required
/// information.
#[derive(Debug)]
#[derive(new)]
pub struct ParserMetaState<'a> {
    /// The tokens of the file currently being parsed.
    /// Those stem from the tokenizer.
    pub tokens: Rc<Vec<Token>>,

    /// The position of the token currently being parsed.
    /// This is an efficient way to transmit which tokens have
    /// been parsed between multiple parsing functions.
    pub cursor: &'a mut usize,

    pub line_map: &'a mut LineMap,

    /// The pre-calculated statements (let, var, ...)
    pub statements: Rc<Vec<Statements>>,

    /// The index of the file currently being processed (look at
    /// tokens). This is helpful for producing debug information.
    pub file_number: &'a mut usize,

    /// The generated code blocks are mostly used while the blocks
    /// get generated only.
    pub blocks: &'a mut Vec<CodeBlockNode>,

    /// The index of the [block](Self::blocks) that should be written to.
    pub current_block_idx: &'a mut usize,

    /// I can't fucking remember ever using or writing this, so it's whatever.
    pub code_block_depth: &'a mut u32,

    /// A list of all the available datatypes.
    /// **Note:** I might need to update this to a `Vec<Rc<ObjectType>>`
    pub datatypes: &'a mut Rc<Vec<ObjectType>>,
}
