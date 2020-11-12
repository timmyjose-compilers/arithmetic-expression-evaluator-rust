use super::{ast, lexer, token};

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

    // start ::= expr
    pub fn parse(&mut self) -> ast::ArithmeticExpression {
        let program_ast = self.parse_expr();
        self.accept(token::TokenType::Eof);
        program_ast
    }

    // expr ::= term (+ expr | - expr | epsilon)
    fn parse_expr(&mut self) -> ast::ArithmeticExpression {
        let term_ast = self.parse_term();

        match self.curr_token.kind {
            token::TokenType::Plus => {
                self.accept_it();
                let expr_ast = self.parse_expr();
                return ast::ArithmeticExpression::Add(Box::new(term_ast), Box::new(expr_ast));
            }
            token::TokenType::Minus => {
                self.accept_it();
                let expr_ast = self.parse_expr();
                return ast::ArithmeticExpression::Sub(Box::new(term_ast), Box::new(expr_ast));
            }

            token::TokenType::Illegal => panic!("`parse_expr`: got an illegal value."),

            _ => term_ast,
        }
    }

    // term ::= factor (* term | / term | epsilon)
    fn parse_term(&mut self) -> ast::ArithmeticExpression {
        let factor_ast = self.parse_factor();

        match self.curr_token.kind {
            token::TokenType::Asterisk => {
                self.accept_it();
                let term_ast = self.parse_term();
                return ast::ArithmeticExpression::Mul(Box::new(factor_ast), Box::new(term_ast));
            }
            token::TokenType::Slash => {
                self.accept_it();
                let term_ast = self.parse_term();
                return ast::ArithmeticExpression::Div(Box::new(factor_ast), Box::new(term_ast));
            }

            token::TokenType::Illegal => panic!("`parse_term`: got an illegal value."),

            _ => factor_ast,
        }
    }

    // factor ::= ( expr ) | integer
    fn parse_factor(&mut self) -> ast::ArithmeticExpression {
        match self.curr_token.kind {
            token::TokenType::LParen => {
                self.accept_it();
                let expr_ast = self.parse_expr();
                self.accept(token::TokenType::RParen);
                return expr_ast;
            }
            token::TokenType::Int => {
                let ival = self.parse_integer();
                return ast::ArithmeticExpression::Value(ival);
            }
            _ => panic!(
                "`parse_factor`: unexpected token of kind {:?}.",
                self.curr_token.kind
            ),
        }
    }

    // integer ::= ... -2 | -1 | 0 | 1 | 2 ...
    fn parse_integer(&mut self) -> i32 {
        if let Ok(ival) = self.curr_token.spelling.parse::<i32>() {
            self.accept_it();
            return ival;
        }
        panic!("`parse_integer`: invalid integer");
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::eval::{ast, lexer};

    #[test]
    fn test_parse_expression1() {
        let input = r#"12345"#;

        let expected_ast = ast::ArithmeticExpression::Value(12345);

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program_ast = parser.parse();
        assert_eq!(
            expected_ast, program_ast,
            "incorrect ast. expected {:?}, got {:?}",
            expected_ast, program_ast
        );
    }

    #[test]
    fn test_parse_expression2() {
        let input = r#"12345 + 23456"#;

        let expected_ast = ast::ArithmeticExpression::Add(
            Box::new(ast::ArithmeticExpression::Value(12345)),
            Box::new(ast::ArithmeticExpression::Value(23456)),
        );

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program_ast = parser.parse();
        assert_eq!(
            expected_ast, program_ast,
            "incorrect ast. expected {:?}, got {:?}",
            expected_ast, program_ast
        );
    }

    #[test]
    fn test_parse_expression3() {
        let input = r#" 12 * (2 + 3)"#;
        let expected_ast = ast::ArithmeticExpression::Mul(
            Box::new(ast::ArithmeticExpression::Value(12)),
            Box::new(ast::ArithmeticExpression::Add(
                Box::new(ast::ArithmeticExpression::Value(2)),
                Box::new(ast::ArithmeticExpression::Value(3)),
            )),
        );

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program_ast = parser.parse();
        assert_eq!(
            expected_ast, program_ast,
            "incorrect ast. expected {:?}, got {:?}",
            expected_ast, program_ast
        );
    }

    #[test]
    fn test_parse_expression4() {
        let input = r#"12 + 2 * 3"#;
        let expected_ast = ast::ArithmeticExpression::Add(
            Box::new(ast::ArithmeticExpression::Value(12)),
            Box::new(ast::ArithmeticExpression::Mul(
                Box::new(ast::ArithmeticExpression::Value(2)),
                Box::new(ast::ArithmeticExpression::Value(3)),
            )),
        );

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program_ast = parser.parse();

        assert_eq!(
            expected_ast, program_ast,
            "incorrect ast. expected {:?}, got {:?}",
            expected_ast, program_ast
        );
    }

    #[test]
    fn test_parse_expression5() {
        let input = r#"1 * 2 + 3"#;
        let expected_ast = ast::ArithmeticExpression::Add(
            Box::new(ast::ArithmeticExpression::Mul(
                Box::new(ast::ArithmeticExpression::Value(1)),
                Box::new(ast::ArithmeticExpression::Value(2)),
            )),
            Box::new(ast::ArithmeticExpression::Value(3)),
        );

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program_ast = parser.parse();

        assert_eq!(
            expected_ast, program_ast,
            "incorrect ast. expected {:?}, got {:?}",
            expected_ast, program_ast
        );
    }

    #[test]
    fn test_parse_expression6() {
        let input = r#"1 + 2 + 3"#;
        let expected_ast = ast::ArithmeticExpression::Add(
            Box::new(ast::ArithmeticExpression::Value(1)),
            Box::new(ast::ArithmeticExpression::Add(
                Box::new(ast::ArithmeticExpression::Value(2)),
                Box::new(ast::ArithmeticExpression::Value(3)),
            )),
        );

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program_ast = parser.parse();

        assert_eq!(
            expected_ast, program_ast,
            "incorrect ast. expected {:?}, got {:?}",
            expected_ast, program_ast
        );
    }

    #[test]
    fn test_parse_expression7() {
        let input = r#"11 + 2 - 3 * 14 / 5"#;
        let expected_ast = ast::ArithmeticExpression::Add(
            Box::new(ast::ArithmeticExpression::Value(11)),
            Box::new(ast::ArithmeticExpression::Sub(
                Box::new(ast::ArithmeticExpression::Value(2)),
                Box::new(ast::ArithmeticExpression::Mul(
                    Box::new(ast::ArithmeticExpression::Value(3)),
                    Box::new(ast::ArithmeticExpression::Div(
                        Box::new(ast::ArithmeticExpression::Value(14)),
                        Box::new(ast::ArithmeticExpression::Value(5)),
                    )),
                )),
            )),
        );

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program_ast = parser.parse();

        assert_eq!(
            expected_ast, program_ast,
            "incorrect ast. expected {:?}, got {:?}",
            expected_ast, program_ast
        );
    }

    #[test]
    fn test_parse_expression8() {
        let input = r#"((11 + 2) - 3)"#;
        let expected_ast = ast::ArithmeticExpression::Sub(
            Box::new(ast::ArithmeticExpression::Add(
                Box::new(ast::ArithmeticExpression::Value(11)),
                Box::new(ast::ArithmeticExpression::Value(2)),
            )),
            Box::new(ast::ArithmeticExpression::Value(3)),
        );

        let lexer = lexer::Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program_ast = parser.parse();

        assert_eq!(
            expected_ast, program_ast,
            "incorrect ast. expected {:?}, got {:?}",
            expected_ast, program_ast
        );
    }
}
