use crate::token::Token;
use crate::token::TokenType::*;
use crate::token::Position;

pub struct Lexer {
    position: Position,
    text: String,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        println!("{}", text);
        Self {
            position: Position { column: 1, index: 0, line: 1 },
            text: text.clone(),
            current_char: Some(text.as_bytes()[0] as char),
        }
    }

    fn advance(&mut self) {
        if let Some(val) = self.current_char {
            self.position.advance(val);
        }

        if self.position.index > (self.text.len()-1) {
            println!("Out of bounds");
            self.current_char = None;
        } else {
            self.current_char = Some(self.text.as_bytes()[self.position.index] as char);
        }
    }
 }