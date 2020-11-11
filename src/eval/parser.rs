use super::{lexer, token};

pub struct Parser {
    lexer: lexer::Lexer,
    curr_token: token::Token,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Self {
        let mut p = Parser {
            lexer: lexer,
            curr_token: token::Token::new(token::TokenType::Illegal, "".to_owned()),
        };

        p.accept_it();
        p
    }

    fn accept_it(&mut self) {
        self.curr_token = self.lexer.next_token();
    }

    fn accept(&mut self, tt: token::TokenType) {
        if self.curr_token.kind != tt {
            panic!(
                "expected to accept token of kind {:?}, got token of kind {:?}",
                tt, self.curr_token.kind
            );
        }
        self.curr_token = self.lexer.next_token();
    }
}
