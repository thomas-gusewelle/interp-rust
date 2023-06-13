use crate::{
    ast::ast::{Identifier, Let, Program, Statement},
    lexer::lexer::{Lexer, Token},
};
use anyhow::{anyhow, Result};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let parser = Parser {
            lexer,
            current_token,
            peek_token,
        };

        return parser;
    }
    pub fn parse_program(&mut self) -> Option<Program> {
        let mut statements: Vec<Statement> = Vec::new();
        while self.current_token != Token::EOF {
            statements.push(self.parse_statement())
        }
        return Some(Program::new(statements));
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    fn parse_statement(&mut self) -> Statement {
        match self.current_token {
            Token::Let => {
                let inside = self.parse_let_statement().unwrap();
                return Statement::Let(inside);
            }
            _ => todo!(),
        }
    }
    fn parse_let_statement(&mut self) -> Result<Let> {
        if !self.expect_peek(Token::Ident(String::new())) {
            return Err(anyhow!("Wrong token type"));
        }

        let identifier = Identifier::new(self.current_token.clone());

        if !self.expect_peek(Token::Assign) {
            return Err(anyhow!("Expected token of Assign"));
        }

        while !self.current_token_is(Token::Semicolon) {
            self.next_token();
        }

        return Ok(Let::new(
            self.current_token.clone(),
            identifier,
            crate::ast::ast::Expression {},
        ));
    }

    fn current_token_is(&mut self, t: Token) -> bool {
        return self.current_token == t;
    }
    fn peek_token_is(&mut self, t: Token) -> bool {
        return self.current_token == t;
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {

    use anyhow::{anyhow, Result};

    use crate::{
        ast::ast::Statement,
        lexer::lexer::{Lexer, Token},
    };

    use super::Parser;
    #[test]
    fn test_let_statment() -> Result<()> {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let mut lexer = Lexer::new(input.into());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        if program.statements.len() != 3 {
            return Err(anyhow!("not right number of statments"));
        }

        let expected_idents = vec![
            Token::Ident(String::from("x")),
            Token::Ident(String::from("y")),
            Token::Ident(String::from("foobar")),
        ];
        for (i, ident) in expected_idents.into_iter().enumerate() {
            let statment = &program.statements[i];
            match statment {
                Statement::Let(x) => {
                    assert_eq!(ident, x.token)
                }
                _ => todo!(),
            }
        }
        Ok(())
    }
}
