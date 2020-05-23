use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Multiply,
    Divide,
    Add,
    Subtract,
    LParen,
    RParen,
    RBracket,
    LBracket,

    Integer(i32),
    Float(f32)
}

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    pub index: usize,
    pub column: usize,
    pub line: usize
}

impl Position {
    pub fn advance(&mut self, character: char) {
        self.index += 1;
        self.column += 1;

        if character == '\n' {
            self.line += 1;
            self.column = 0;
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Token({:?})", self.token_type)
    }
}