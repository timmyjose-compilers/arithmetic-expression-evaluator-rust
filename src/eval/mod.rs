use traits::Evaluatable;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod traits;

pub fn evaluate(input: &str) -> i32 {
    let mut parser = parser::Parser::new(lexer::Lexer::new(input));
    parser.parse().evaluate()
}
