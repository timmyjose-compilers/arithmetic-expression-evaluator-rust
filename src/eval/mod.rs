use crate::error;

pub mod lexer;
pub mod parser;
pub mod token;

pub type Int = i32;

pub fn evaluate(input: &str) -> Result<Int, error::ArithError> {
    Ok(42)
}
