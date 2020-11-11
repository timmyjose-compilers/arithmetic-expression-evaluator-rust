use crate::eval::token;

type Byte = u8;

pub struct Lexer {
    input: Vec<Byte>,
    position: usize,
    read_position: usize,
    ch: Byte,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.as_bytes().to_vec(),
            position: 0,
            read_position: 0,
            ch: b'\0',
        };

        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let start_position = self.position;

        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        let slice = &self.input[start_position..self.position];
        std::str::from_utf8(slice).unwrap().to_string()
    }

    pub fn next_token(&mut self) -> token::Token {
        let mut token = token::Token::new(token::TokenType::Illegal, "".to_owned());

        self.skip_whitespace();

        match self.ch {
            b'(' => token = token::Token::new(token::TokenType::LParen, "(".to_owned()),
            b')' => token = token::Token::new(token::TokenType::RParen, ")".to_owned()),
            b'+' => token = token::Token::new(token::TokenType::Plus, "+".to_owned()),
            b'-' => token = token::Token::new(token::TokenType::Minus, "-".to_owned()),
            b'*' => token = token::Token::new(token::TokenType::Asterisk, "*".to_owned()),
            b'/' => token = token::Token::new(token::TokenType::Slash, "/".to_owned()),
            b'\0' => token = token::Token::new(token::TokenType::Eof, "".to_owned()),
            _ => {
                if self.ch.is_ascii_digit() {
                    let kind = token::TokenType::Int;
                    let spelling = self.read_number();
                    token = token::Token::new(kind, spelling);
                    return token;
                }
            }
        }

        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::token;
    use super::Lexer;

    struct TokenTestcase<'a> {
        pub expected_kind: token::TokenType,
        pub expected_spelling: &'a str,
    }

    impl<'a> TokenTestcase<'a> {
        fn new(expected_kind: token::TokenType, expected_spelling: &'a str) -> Self {
            TokenTestcase {
                expected_kind,
                expected_spelling,
            }
        }
    }

    #[test]
    fn test_next_token_basic() {
        let input = r#"
            1 + 2
        "#;

        let tests = vec![
            TokenTestcase::new(token::TokenType::Int, "1"),
            TokenTestcase::new(token::TokenType::Plus, "+"),
            TokenTestcase::new(token::TokenType::Int, "2"),
            TokenTestcase::new(token::TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (idx, tt) in tests.iter().enumerate() {
            let token = lexer.next_token();

            assert_eq!(
                tt.expected_kind, token.kind,
                "test [{}] - incorrect token type. expected {:?}, got {:?}",
                idx, tt.expected_kind, token.kind
            );

            assert_eq!(
                tt.expected_spelling, token.spelling,
                "test [{}] - incorrect spelling. expected {:?}, got {:?}",
                idx, tt.expected_spelling, token.spelling
            );
        }
    }

    #[test]
    fn test_next_token_advanced() {
        let input = r#"
            ((1 + 32) * 312 / (2 - 3) * (14 + 5))
        "#;

        let tests = vec![
            TokenTestcase::new(token::TokenType::LParen, "("),
            TokenTestcase::new(token::TokenType::LParen, "("),
            TokenTestcase::new(token::TokenType::Int, "1"),
            TokenTestcase::new(token::TokenType::Plus, "+"),
            TokenTestcase::new(token::TokenType::Int, "32"),
            TokenTestcase::new(token::TokenType::RParen, ")"),
            TokenTestcase::new(token::TokenType::Asterisk, "*"),
            TokenTestcase::new(token::TokenType::Int, "312"),
            TokenTestcase::new(token::TokenType::Slash, "/"),
            TokenTestcase::new(token::TokenType::LParen, "("),
            TokenTestcase::new(token::TokenType::Int, "2"),
            TokenTestcase::new(token::TokenType::Minus, "-"),
            TokenTestcase::new(token::TokenType::Int, "3"),
            TokenTestcase::new(token::TokenType::RParen, ")"),
            TokenTestcase::new(token::TokenType::Asterisk, "*"),
            TokenTestcase::new(token::TokenType::LParen, "("),
            TokenTestcase::new(token::TokenType::Int, "14"),
            TokenTestcase::new(token::TokenType::Plus, "+"),
            TokenTestcase::new(token::TokenType::Int, "5"),
            TokenTestcase::new(token::TokenType::RParen, ")"),
            TokenTestcase::new(token::TokenType::RParen, ")"),
            TokenTestcase::new(token::TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input);

        for (idx, tt) in tests.iter().enumerate() {
            let token = lexer.next_token();

            assert_eq!(
                tt.expected_kind, token.kind,
                "test [{}] - incorrect token type. expected {:?}, got {:?}",
                idx, tt.expected_kind, token.kind
            );

            assert_eq!(
                tt.expected_spelling, token.spelling,
                "test [{}] - incorrect spelling. expected {:?}, got {:?}",
                idx, tt.expected_spelling, token.spelling
            );
        }
    }
}
