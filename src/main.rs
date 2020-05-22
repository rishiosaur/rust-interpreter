use std::fmt;

#[derive(Debug, Clone)]
#[derive(PartialEq)]
enum TokenType {
    Integer(i32),
    Plus,
    EOF,
    Subtract
}


use TokenType::*;

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Integer(x) => format!("Integer: {}", x),
            Plus => "Plus".into(),
            EOF => "EOF".into(),
            Subtract => "Subtract".into()
        };
        write!(f, "{}", output)
    }
}

#[derive(Clone, Debug)]
struct Token {
    token_type: TokenType
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Token({})", self.token_type)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Interpreter {
    text: String,
    position: usize,
    token: Option<Token>
}

impl Interpreter {
    pub fn new(text: String) -> Self {
        Interpreter {
            text: text,
            position: 0,
            token: None
        }
    } 

    pub fn get_next_token (&mut self) -> Option<Token> {
        if self.position > self.text.len() - 1 {
            return Some(Token { token_type: EOF });
        }

        let mut current_char: char = self.text.as_bytes()[self.position] as char;

        match current_char {
            x if x.is_digit(10) => {
                let mut digits: String = String::new();

                

                while current_char.is_digit(10) {
                    // If the position is at the end, then break the loop; we are at the end of the program.
                    if self.position == self.text.len() - 1 {
                        digits.push(current_char);
                        break;
                    }

                    digits.push(current_char);

                    //Update position
                    self.position += 1;

                    // Update the current character
                    current_char = self.text.as_bytes()[self.position] as char;
                }
                
                // self.position += 1;
                Some(Token { token_type: Integer(digits.parse::<i32>().unwrap()) })
            }

            x if x == ' ' => {
                self.position += 1;
                return self.get_next_token();
            }

            x if x == '+' => {
                self.position += 1;
                Some(Token { token_type: Plus })
            }

            x if x == '-' => {
                self.position += 1;
                Some(Token { token_type: Subtract })
            }

            _ => {
                panic!("Unexpected token found at position {}", self.position);
            }
        }
    }

    fn eat(&mut self, token: Token) {
        let ctok: Token = self.clone().token.unwrap();
        if ctok.token_type == token.token_type {
            self.token = self.get_next_token();
        } else {
            panic!("Validation error for eat");
        }
    }

    fn expr(&mut self) -> i32 {
        // Advance to next token
        self.token = self.get_next_token();
        let mut left = 0;
        let mut right = 0;
        let mut operator = "nothing";

        let token = self.clone().token.unwrap();
        if let Integer(value) = token.token_type {
            left = value;
            self.eat(token);
        }

        let token = self.clone().token.unwrap();
        if token.token_type == Plus {
            operator = "plus";
            self.eat(token);
        } else if token.token_type == Subtract {
            operator = "subtract";
            self.eat(token);
        }

        let token = self.clone().token.unwrap();
        if let Integer(value) = token.token_type {
            right = value;
            self.eat(token);
        }

        match operator.as_ref() {
            "plus" => left + right,
            "subtract" => left - right,
            _ => panic!("No operator found.")
        }
    }
 
}


fn main () {
    let mut interpreter = Interpreter::new("488 - 2".into());
    let result = interpreter.expr();

    println!("{}", result);
}