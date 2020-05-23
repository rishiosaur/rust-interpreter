use nodes::{BinOpNode, NumberNode};
use token::Token;
struct Parser {
    tokens: Vec<Token>,
    index: u32
}

