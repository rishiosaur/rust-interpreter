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
