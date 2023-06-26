use std::fmt::Display;

use crate::ast::ast::{Expression, Statement};
use crate::lexer::lexer::Token;
use anyhow::{anyhow, Ok, Result};

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
                        _ => Err(anyhow!(
                            "Wrong Token Type: Expected: {:?}, Got: {:?}",
                            Token::Int(0),
                            i
                        )),
                    },
                    Expression::Boolean(b) => match b {
                        Token::True => Ok(Object::Boolean(true)),
                        Token::False => Ok(Object::Boolean(false)),
                        _ => Err(anyhow!("Wrong token type. Expected Boolean, Got: {:?}", b)),
                    },
                    Expression::Prefix(p) => {
                        let right = Object::eval(vec![Statement::Expression(p.right)]);
                        match &p.token {
                            Token::Bang => match right {
                                Object::Boolean(true) => Ok(Object::Boolean(false)),
                                Object::Boolean(false) => Ok(Object::Boolean(true)),
                                Object::Null => Ok(Object::Boolean(true)),
                                _ => Ok(Object::Boolean(false)),
                            },
                            _ => Err(anyhow!("Wrong Token Type")),
                        }
                    }
                    _ => Ok(Object::Null),
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

    #[test]
    fn test_boolean_experssion() {
        struct Test {
            input: Vec<u8>,
            expected: Object,
        }
        let tests = vec![
            Test {
                input: "true".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "false".into(),
                expected: Object::Boolean(false),
            },
        ];

        for test in tests.into_iter() {
            let evaluated = test_eval(test.input);
            assert_eq!(test.expected, evaluated);
        }
    }

    #[test]
    fn test_bang_operator() {
        struct Test {
            input: Vec<u8>,
            expected: Object,
        }
        let tests = vec![
            Test {
                input: "!true".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "!false".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "!5".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "!!true".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "!!false".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "!!5".into(),
                expected: Object::Boolean(true),
            },
        ];

        for test in tests.into_iter() {
            let evaluated = test_eval(test.input);
            assert_eq!(test.expected, evaluated);
        }
    }
    fn test_eval(input: Vec<u8>) -> Object {
        let lex = Lexer::new(input);
        let mut parser = Parser::new(lex);
        let program = parser.parse_program().unwrap();

        return Object::eval(program.statements);
    }
}
