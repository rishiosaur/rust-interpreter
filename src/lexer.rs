use crate::token::Token;
use crate::token::TokenType::*;
use crate::token::Position;

pub struct Lexer {
    position: Position,
    text: String,
    current_char: Option<char>,
}
