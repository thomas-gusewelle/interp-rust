use std::fmt::Display;

use crate::ast::ast::{Expression, Statement};
use crate::lexer::lexer::Token;
use anyhow::{anyhow, Result};

#[derive(PartialEq, Clone, Debug)]
pub enum Object {
    Integer(usize),
    Boolean(bool),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Object::Integer(u) => write!(f, "Integer Value: {}", u),
            Object::Boolean(b) => write!(f, "Bool value: {}", b),
            Object::Null => write!(f, "Null value"),
        };
    }
}

impl Object {
    pub fn eval(nodes: Vec<Statement>) -> Self {
        let mut result: Result<Object> = Ok(Object::Null);
        for node in nodes.into_iter() {
            result = match node {
                Statement::Let(l) => Err(anyhow!("Todo")),
                Statement::Return(r) => Err(anyhow!("Todo")),
                Statement::Expression(e) => match e {
                    Expression::Integer(i) => match i {
                        Token::Int(int) => Ok(Object::Integer(int)),
                        _ => Err(anyhow!("Todo")),
                    },
                    _ => Err(anyhow!("Todo")),
                },
            };
        }
        result.unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::lexer::Lexer;
    use crate::object::object::Object;
    use crate::parser::parser::Parser;

    #[test]
    fn test_eval_integer_expression() {
        struct Test {
            input: Vec<u8>,
            expected: Object,
        }
        let tests = vec![
            Test {
                input: "5".into(),
                expected: Object::Integer(5),
            },
            Test {
                input: "10".into(),
                expected: Object::Integer(10),
            },
        ];

        for test in tests.into_iter() {
            let evaluated = test_eval(test.input);
            assert_eq!(test.expected, evaluated);
        }
    }

    pub fn test_eval(input: Vec<u8>) -> Object {
        let lex = Lexer::new(input);
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();

        return Object::eval(program.statements);
    }
}
