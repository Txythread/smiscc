use crate::compiler::line_map::TokenPosition;

pub struct TreeNode {
    token_position: TokenPosition,
    sub_nodes: Vec<Box<TreeNode>>,
    kind: TreeNodeKind,
}

pub enum TreeNodeKind {
    ConstantDeclaration,
    VariableDeclaration,
    FunctionCall,
    ArithmeticOperation,
    ArrayIndex,
    Line,
    Body
}