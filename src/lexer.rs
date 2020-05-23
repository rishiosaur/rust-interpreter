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

    fn integer (&mut self) -> Token {
        println!("Integer detected");
        let mut digits = String::new();

        // Counting points for floating-points
        let mut dots = 0;

        while self.current_char != None && (self.current_char.unwrap().is_digit(10) || self.current_char.unwrap() == '.') {
            println!("Character detected");
            if self.current_char.unwrap() == '.' {
                println!("Dot detected");
                dots += 1;
                digits.push('.');
                self.advance();
            } else {
                println!("Number detected");
                digits.push(self.current_char.unwrap());
                self.advance();
            }
        }

        if dots > 0 {
            Token { token_type:  Float(digits.parse::<f32>().unwrap()) }
        } else {
            Token { token_type: Integer(digits.parse::<i32>().unwrap()) }
        }

    }

    pub fn lex (&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        println!("Making");

        while self.current_char != None {
            println!("Character detected");
            match self.current_char.unwrap() {
                char if char.is_whitespace() => self.advance(),

                // Operators
                '+' => {
                    tokens.push(Token { token_type: Add });
                    self.advance();
                }

                '-' => {
                    tokens.push(Token { token_type: Subtract });
                    self.advance();
                }

                '*' => {
                    tokens.push(Token { token_type: Multiply });
                    self.advance();
                }

                '/' => {
                    tokens.push(Token { token_type: Divide });
                    self.advance();
                }


                // Digit
                char if char.is_digit(10) => {
                    tokens.push(self.integer());
                    self.advance();
                },

                _ => {
                    panic!(format!("Error in parsing at position: {:#?}", self.position))
                }
            }
        }

        tokens
    }
 }