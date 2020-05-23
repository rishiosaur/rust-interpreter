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

    pub fn advance(&mut self) {
        self.index += 1;
        if self.index < self.tokens.len()-1 {
            // Get the current token
        }
    } 
    pub fn expr() {

    }
    pub fn term() {

    }
    pub fn factor() {

    }
}