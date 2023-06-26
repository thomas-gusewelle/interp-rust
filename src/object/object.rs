use std::fmt::Display;

use crate::ast::ast::{Expression, Statement};
use crate::lexer::lexer::Token;
use anyhow::{anyhow, Ok, Result};

#[derive(PartialEq, Clone, Debug)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    Null,
    Return(Box<Object>),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Object::Integer(u) => write!(f, "Integer Value: {}", u),
            Object::Boolean(b) => write!(f, "Bool value: {}", b),
            Object::Null => write!(f, "Null value"),
            Object::Return(o) => write!(f, "Return value: {}", o),
        };
    }
}

impl Object {
    pub fn eval(nodes: Vec<Statement>) -> Self {
        let mut result: Result<Object> = Ok(Object::Null);
        'stacks: for node in nodes.into_iter() {
            result = match node {
                Statement::Let(l) => Err(anyhow!("Todo")),
                Statement::Return(r) => {
                    let val = Object::eval(vec![Statement::Expression(r.return_value)]);
                    // return breaks the loop here
                    return Object::Return(Box::new(val));
                }
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
                            Token::Minus => match right {
                                Object::Integer(int) => Ok(Object::Integer(-int)),
                                _ => Err(anyhow!("Minus prefix can only be used with an integer")),
                            },
                            _ => Err(anyhow!("Wrong Token Type")),
                        }
                    }
                    Expression::Infix(inf) => {
                        let left = Object::eval(vec![Statement::Expression(inf.left)]);
                        let right = Object::eval(vec![Statement::Expression(inf.right)]);

                        match (left, right) {
                            (Object::Integer(il), Object::Integer(ir)) => match inf.token {
                                Token::Plus => Ok(Object::Integer(il + ir)),
                                Token::Minus => Ok(Object::Integer(il - ir)),
                                Token::Asterisk => Ok(Object::Integer(il * ir)),
                                Token::Slash => Ok(Object::Integer(il / ir)),
                                Token::LessThan => Ok(Object::Boolean(il < ir)),
                                Token::GreaterThan => Ok(Object::Boolean(il > ir)),
                                Token::Equal => Ok(Object::Boolean(il == ir)),
                                Token::NotEqual => Ok(Object::Boolean(il != ir)),
                                _ => Err(anyhow!("Wrong oeprator token for infix")),
                            },
                            (Object::Boolean(bl), Object::Boolean(br)) => match inf.token {
                                Token::Equal => Ok(Object::Boolean(bl == br)),
                                Token::NotEqual => Ok(Object::Boolean(bl != br)),
                                _ => Err(anyhow!("Wrong opertor for infix")),
                            },
                            _ => Err(anyhow!("Wrong token type in infix")),
                        }
                    }
                    Expression::If(i) => {
                        let condition = Object::eval(vec![Statement::Expression(i.condition)]);
                        if condition.is_truthy() {
                            let object = Object::eval(i.consequence.statements);
                            if object.expect_object_is(&Object::Return(Box::new(Object::Null))) {
                                return object;
                            };
                            Ok(object)
                        } else if let Some(alt) = i.alternative {
                            let object = Object::eval(alt.statements);
                            if object.expect_object_is(&Object::Return(Box::new(Object::Null))) {
                                return object;
                            };
                            Ok(object)
                        } else {
                            Ok(Object::Null)
                        }
                    }
                    _ => Ok(Object::Null),
                },
            };
        }
        result.unwrap()
    }
    fn is_truthy(&self) -> bool {
        match &self {
            Object::Null => false,
            Object::Boolean(false) => false,
            Object::Boolean(true) => true,
            _ => true,
        }
    }

    fn expect_object_is(&self, object: &Object) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(object)
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::lexer::Lexer;
    use crate::object::object::Object;
    use crate::parser::parser::Parser;

    #[test]
    fn test_return_statements() {
        struct Test {
            input: Vec<u8>,
            expected: Object,
        }
        let tests = vec![
            Test {
                input: "return 10;".into(),
                expected: Object::Return(Box::new(Object::Integer(10))),
            },
            Test {
                input: "return 10; 9;".into(),
                expected: Object::Return(Box::new(Object::Integer(10))),
            },
            Test {
                input: "return 2 * 5; 9;".into(),
                expected: Object::Return(Box::new(Object::Integer(10))),
            },
            Test {
                input: "9; return 2 * 5; 9;".into(),
                expected: Object::Return(Box::new(Object::Integer(10))),
            },
            Test {
                input: "if (10 > 1){if (10 > 1){return 10;}return 1;}".into(),
                expected: Object::Return(Box::new(Object::Integer(10))),
            },
        ];

        for test in tests.into_iter() {
            let evaluated = test_eval(test.input);
            assert_eq!(test.expected, evaluated);
        }
    }

    #[test]
    fn test_if_else_expression() {
        struct Test {
            input: Vec<u8>,
            expected: Object,
        }
        let tests = vec![
            Test {
                input: "if (true) { 10 }".into(),
                expected: Object::Integer(10),
            },
            Test {
                input: "if (false) {10}".into(),
                expected: Object::Null,
            },
            Test {
                input: "if (1){10}".into(),
                expected: Object::Integer(10),
            },
            Test {
                input: "if (1 < 2){10}".into(),
                expected: Object::Integer(10),
            },
            Test {
                input: "if (1>2){10}".into(),
                expected: Object::Null,
            },
            Test {
                input: "if (1 < 2){10}else{20}".into(),
                expected: Object::Integer(10),
            },
            Test {
                input: "if (1 > 2){10}else{20}".into(),
                expected: Object::Integer(20),
            },
        ];

        for test in tests.into_iter() {
            let evaluated = test_eval(test.input);
            assert_eq!(test.expected, evaluated);
        }
    }

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
            Test {
                input: "-10".into(),
                expected: Object::Integer(-10),
            },
            Test {
                input: "-5".into(),
                expected: Object::Integer(-5),
            },
            Test {
                input: "5 + 5 + 5".into(),
                expected: Object::Integer(15),
            },
            Test {
                input: "5 -5 -5".into(),
                expected: Object::Integer(-5),
            },
            Test {
                input: "5 * 5".into(),
                expected: Object::Integer(25),
            },
            Test {
                input: "25 / 5".into(),
                expected: Object::Integer(5),
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
            Test {
                input: "1 < 2".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "1 > 2".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "1 < 1".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "1 > 1".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "1 == 1".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "1 != 1".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "1 == 2".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "1 != 2".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "true == true".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "false == false".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "true == false".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "true != false".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "false != true".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "(1 < 2) == true".into(),
                expected: Object::Boolean(true),
            },
            Test {
                input: "(1 < 2) == false".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "(1 > 2) == true".into(),
                expected: Object::Boolean(false),
            },
            Test {
                input: "(1 > 2) == false".into(),
                expected: Object::Boolean(true),
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
