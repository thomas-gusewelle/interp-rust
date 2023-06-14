use crate::{
    ast::ast::{Expression, Identifier, Let, Program, Return, Statement},
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
            if let Some(s) = self.parse_statement() {
                println!("{:?}", s);
                statements.push(s);
            };
            self.next_token();
        }
        return Some(Program::new(statements));
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => {
                let inside = self.parse_let_statement().unwrap();
                return Some(Statement::Let(inside));
            }
            Token::Return => {
                return Some(self.parse_return_statement().unwrap());
            }
            // calls self.next_token to move the token forward when it is not a let statment. This is because
            // the funciton is not fully done.
            _ => None,
        }
    }
    fn parse_let_statement(&mut self) -> Result<Let> {
        if !self.expect_peek(Token::Ident(String::new())) {
            return Err(anyhow!("Wrong token type"));
        }
        let ident_token = self.current_token.clone();
        let identifier = Identifier::new(self.current_token.clone());

        if !self.expect_peek(Token::Assign) {
            return Err(anyhow!("Expected token of Assign"));
        }

        while !self.current_token_is(Token::Semicolon) {
            println!("Token: {:?}", self.current_token);
            self.next_token();
        }

        return Ok(Let::new(
            ident_token,
            identifier,
            crate::ast::ast::Expression {},
        ));
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        let token = self.current_token.clone();

        self.next_token();

        // TODO: Skipping expressions for now
        while !self.current_token_is(Token::Semicolon) {
            self.next_token();
        }
        let return_inside = Return::new(token, Expression {});
        Ok(Statement::Return(return_inside))
    }

    fn current_token_is(&mut self, t: Token) -> bool {
        std::mem::discriminant(&self.current_token) == std::mem::discriminant(&t)
    }
    fn peek_token_is(&mut self, t: Token) -> bool {
        std::mem::discriminant(&self.peek_token) == std::mem::discriminant(&t)
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

    use std::any;

    use anyhow::{anyhow, Ok, Result};

    use crate::{
        ast::ast::{Return, Statement},
        lexer::lexer::{Lexer, Token},
    };

    use super::Parser;
    #[test]
    fn test_let_statment() -> Result<()> {
        println!("here 101");
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input.into());
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
            println!("Here 123");
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
    #[test]
    fn test_return_statement() -> Result<()> {
        let input: Vec<u8> = r#"
            return 5;
            return 10;
            return 993322;
            "#
        .into();

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        if program.statements.len() != 3 {
            return Err(anyhow!("wrong number of statements"));
        };
        program.statements.into_iter().for_each(|stmt| {
            assert_eq!(
                std::mem::discriminant(&stmt),
                std::mem::discriminant(&Statement::Return(Return::new(
                    Token::Return,
                    crate::ast::ast::Expression {}
                )))
            );
        });
        Ok(())
    }
}
