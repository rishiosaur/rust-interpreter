mod token;
mod lexer;
mod nodes;

use lexer::Lexer;

fn main() {
    let text = "288.80000 * 34.988 + 32 - 4".into();
    let mut lexer = Lexer::new(text);

    println!("{:#?}", lexer.lex())
}