use crate::eval::{lexer, token};
use std::env;
use std::io::{self, Write};

const PROMPT: &'static str = ">> ";

fn get_user() -> String {
    match env::var("USER") {
        Ok(user) => user,
        Err(e) => panic!("failed to get user information: {}", e),
    }
}

fn get_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.trim().to_string()
}

pub fn run() {
    let user = get_user();
    println!(
        "Welcome, {}. Enter arithmetic expressions to evaluate them, or Ctrl+C to quit",
        user
    );

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let input = get_line();
        // match eval::evaluate(&input) {
        //     Ok(res) => println!("{}", res),
        //     Err(e) => println!("Invalid input: {}. Try again!", e),
        // }
        let mut lexer = lexer::Lexer::new(&input);
        loop {
            let token = lexer.next_token();
            if token.kind == token::TokenType::Eof {
                break;
            }

            println!("{:?}", token);
        }
    }
}
