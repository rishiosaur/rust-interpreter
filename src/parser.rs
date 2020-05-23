use nodes::{BinOpNode, NumberNode};
use token::Token;
struct Parser {
    tokens: Vec<Token>,
    index: u32
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            index: 0
        }
    }

}