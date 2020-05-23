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