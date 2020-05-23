use token::Token;
pub trait Factor {
    pub fn new (token: Token) -> Self;
}
pub struct NumberNode {
    pub token: Token
}

impl Factor for NumberNode {
    pub fn new(token: Token) -> Self {
        Self { token: token }
    }
}
pub struct BinOpNode<T: Factor> {
    pub left_node: T,
    pub op_tok: Token,
    pub right_node: T
}

impl std::fmt::Display for BinOpNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(format!("{} {} {}", self.left_node, self.op_tok, self.right_node))
    }
} 