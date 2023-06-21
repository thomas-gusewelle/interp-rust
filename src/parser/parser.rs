use std::{collections::HashMap, path::Prefix};

use crate::{
    ast::ast::{
        Expression, Identifier, InfixExpression, Let, PrefixExpression, Program, Return, Statement,
    },
    lexer::lexer::{Lexer, Token},
};
use anyhow::{anyhow, Ok, Result};

#[derive(Debug, PartialEq, Clone)]
pub enum Precidence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl From<&Token> for Precidence {
    fn from(token: &Token) -> Self {
        match token {
            Token::Equal => Precidence::Equals,
            Token::NotEqual => Precidence::Equals,
            Token::LessThan => Precidence::LessGreater,
            Token::GreaterThan => Precidence::LessGreater,
            Token::Plus => Precidence::Sum,
            Token::Minus => Precidence::Sum,
            Token::Slash => Precidence::Product,
            Token::Asterisk => Precidence::Product,
            _ => Precidence::Lowest,
        }
    }
}

type PrefixParseFn = Result<Expression>;
type InfixParseFn = fn(Expression) -> Result<Expression>;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<Token, PrefixParseFn>,
    infix_parse_fns: HashMap<Token, InfixParseFn>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let mut parser = Parser {
            lexer,
            current_token: current_token.clone(),
            peek_token,
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
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

    pub fn errors(&mut self) -> Vec<String> {
        return self.errors.clone();
    }

    fn peek_error(&mut self, t: Token) {
        self.errors
            .push(format!("Expected {} but got {}", self.peek_token, t));
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
            _ => {
                return Some(self.parse_expression_statement().unwrap());
            }
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
            Expression::Identifier(self.current_token.clone()),
        ));
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        let token = self.current_token.clone();

        self.next_token();

        // TODO: Skipping expressions for now
        while !self.current_token_is(Token::Semicolon) {
            self.next_token();
        }
        let return_inside = Return::new(token, Expression::Identifier(self.current_token.clone()));
        Ok(Statement::Return(return_inside))
    }

    fn current_token_is(&mut self, t: Token) -> bool {
        std::mem::discriminant(&self.current_token) == std::mem::discriminant(&t)
    }
    fn peek_token_is(&mut self, t: Token) -> bool {
        std::mem::discriminant(&self.peek_token) == std::mem::discriminant(&t)
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            return true;
        } else {
            self.peek_error(t.clone());
            return false;
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expression = self.parse_expression(Precidence::Lowest).unwrap();

        if self.peek_token_is(Token::Semicolon) {
            self.next_token();
        };

        return Ok(Statement::Expression(expression));
    }

    fn parse_expression(&mut self, precidence: Precidence) -> Result<Expression> {
        let mut expression = match self.current_token {
            Token::Ident(_) => Ok(Expression::Identifier(self.current_token.clone())),
            Token::Int(_) => Ok(Expression::Integer(self.current_token.clone())),
            Token::Bang | Token::Minus => self.parse_prefix(),
            _ => Err(anyhow!("Unknown token type")),
        };

        while !self.peek_token_is(Token::Semicolon)
            && (precidence.clone() as i32) < (self.peek_precedence() as i32)
        {
            self.next_token();
            let infix_exp = self.parse_infix(expression?.clone());
            expression = infix_exp;
        }
        expression
    }

    fn parse_prefix(&mut self) -> Result<Expression> {
        let token = self.current_token.clone();
        self.next_token();
        let right = self.parse_expression(Precidence::Prefix).unwrap();

        Ok(Expression::Prefix(Box::new(PrefixExpression::new(
            token, right,
        ))))
    }
    fn parse_infix(&mut self, left: Expression) -> Result<Expression> {
        let token = self.current_token.clone();
        let precidence = Precidence::from(&self.current_token);
        self.next_token();
        let right = self.parse_expression(precidence);

        Ok(Expression::Infix(Box::new(InfixExpression::new(
            left,
            token,
            right.unwrap(),
        ))))
    }
    fn peek_precedence(&mut self) -> Precidence {
        Precidence::from(&self.peek_token)
    }
    fn next_token_is_infix_operator(&self) -> bool {
        match self.peek_token {
            Token::Plus
            | Token::Minus
            | Token::Slash
            | Token::Asterisk
            | Token::Equal
            | Token::NotEqual
            | Token::LessThan
            | Token::GreaterThan => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use anyhow::{anyhow, Ok, Result};

    use crate::{
        ast::ast::{Expression, PrefixExpression, Return, Statement},
        lexer::lexer::{Lexer, Token},
        parser,
    };

    fn check_errors(errors: Vec<String>) {
        if errors.len() == 0 {
            return;
        };

        for err in errors.into_iter() {
            print!("Error: {:?}", err);
        }
        panic!("There were errors in the parser")
    }

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
        check_errors(parser.errors.clone());

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
        check_errors(parser.errors().clone());

        let program = parser.parse_program().unwrap();

        if program.statements.len() != 3 {
            return Err(anyhow!("wrong number of statements"));
        };
        program.statements.into_iter().for_each(|stmt| {
            assert_eq!(
                std::mem::discriminant(&stmt),
                std::mem::discriminant(&Statement::Return(Return::new(
                    Token::Return,
                    Expression::Identifier(Token::Return),
                )))
            );
        });
        Ok(())
    }

    #[test]
    fn test_identifier_expression() -> Result<()> {
        let input: Vec<u8> = "foobar;".into();

        let lex = Lexer::new(input);
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();
        check_errors(parser.errors().clone());

        if program.statements.len() != 1 {
            return Err(anyhow!("Wrong number of statements"));
        };

        match &program.statements[0] {
            Statement::Expression(exp) => match exp {
                Expression::Identifier(t) => match t {
                    Token::Ident(s) => {
                        assert_eq!(s, &"foobar".to_string())
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            },
            _ => println!("Other"),
        }
        Ok(())
    }

    #[test]
    fn test_integer_expression() -> Result<()> {
        let input: Vec<u8> = "5;".into();

        let lex = Lexer::new(input);
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();
        check_errors(parser.errors().clone());

        if program.statements.len() != 1 {
            return Err(anyhow!("Wrong number of statements"));
        };

        match &program.statements[0] {
            Statement::Expression(exp) => match exp {
                Expression::Integer(t) => match t {
                    Token::Int(s) => {
                        println!("this is s: {}", s);
                        assert_eq!(s.to_owned(), 5 as usize)
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            },
            _ => println!("Other"),
        }
        Ok(())
    }

    #[test]
    fn test_parsing_prefix_expressions() -> Result<()> {
        struct prefix_test {
            input: String,
            operator: Token,
            int_value: usize,
        }
        let tests = vec![
            prefix_test {
                input: "!5".to_string(),
                operator: Token::Bang,
                int_value: 5 as usize,
            },
            prefix_test {
                input: "-15".to_string(),
                operator: Token::Minus,
                int_value: 15 as usize,
            },
        ];

        for (i, test) in tests.into_iter().enumerate() {
            let lex = Lexer::new(test.input.into());
            let mut parser = Parser::new(lex);
            let program = parser.parse_program().unwrap();
            check_errors(parser.errors().clone());

            if program.statements.len() != 1 {
                return Err(anyhow!("Wrong number of statments"));
            };

            match &program.statements[0] {
                Statement::Expression(exp) => match exp {
                    Expression::Prefix(t) => {
                        let prefix_expression = t;
                        assert_eq!(prefix_expression.token, test.operator);
                        match &prefix_expression.right {
                            Expression::Integer(t) => assert_eq!(t, &Token::Int(test.int_value)),
                            _ => todo!(),
                        }
                    }
                    _ => todo!(),
                },
                _ => println!("Other"),
            }
        }

        Ok(())
    }

    #[test]
    fn test_parsing_infix_expressions() -> Result<()> {
        struct infix {
            input: Vec<u8>,
            left_token: Expression,
            operator: Token,
            right_token: Expression,
        }

        let tests = vec![
            infix {
                input: "5 + 5;".into(),
                left_token: Expression::Integer(Token::Int(5)),
                operator: Token::Plus,
                right_token: Expression::Integer(Token::Int(5)),
            },
            infix {
                input: "5 - 5;".into(),
                left_token: Expression::Integer(Token::Int(5)),
                operator: Token::Minus,
                right_token: Expression::Integer(Token::Int(5)),
            },
            infix {
                input: "5 * 5;".into(),
                left_token: Expression::Integer(Token::Int(5)),
                operator: Token::Asterisk,
                right_token: Expression::Integer(Token::Int(5)),
            },
            infix {
                input: "5 / 5;".into(),
                left_token: Expression::Integer(Token::Int(5)),
                operator: Token::Slash,
                right_token: Expression::Integer(Token::Int(5)),
            },
            infix {
                input: "5 > 5;".into(),
                left_token: Expression::Integer(Token::Int(5)),
                operator: Token::GreaterThan,
                right_token: Expression::Integer(Token::Int(5)),
            },
            infix {
                input: "5 < 5;".into(),
                left_token: Expression::Integer(Token::Int(5)),
                operator: Token::LessThan,
                right_token: Expression::Integer(Token::Int(5)),
            },
            infix {
                input: "5 == 5;".into(),
                left_token: Expression::Integer(Token::Int(5)),
                operator: Token::Equal,
                right_token: Expression::Integer(Token::Int(5)),
            },
            infix {
                input: "5 != 5".into(),
                left_token: Expression::Integer(Token::Int(5)),
                operator: Token::NotEqual,
                right_token: Expression::Integer(Token::Int(5)),
            },
        ];

        for test in tests.into_iter() {
            let lex = Lexer::new(test.input);
            let mut parser = Parser::new(lex);
            let program = parser.parse_program().unwrap();
            check_errors(parser.errors().clone());

            if program.statements.len() != 1 {
                return Err(anyhow!("Wrong number of statements"));
            };

            match &program.statements[0] {
                Statement::Expression(exp) => match exp {
                    // TODO: finish writing test
                    Expression::Infix(i) => {
                        assert_eq!(i.left, test.left_token);
                        assert_eq!(i.token, test.operator);
                        assert_eq!(i.right, test.right_token);
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            }
        }

        Ok(())
    }
}
