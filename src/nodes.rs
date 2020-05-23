use crate::token::Token;

pub struct NumberNode {
    pub token: Token
}

impl NumberNode {
    fn new(token: Token) -> Self {
        Self { token: token }
    }
}

pub struct BinOpNode {
    pub left_node: NumberNode,
    pub op_tok: Token,
    pub right_node: NumberNode
}

impl std::fmt::Display for BinOpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(format!("{} {} {}", self.left_node, self.op_tok, self.right_node))
    }
} 