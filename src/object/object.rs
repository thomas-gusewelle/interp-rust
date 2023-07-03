use std::collections::HashMap;
use std::fmt::Display;

use crate::ast::ast::{BlockStatement, Expression, Identifier, Statement};
use crate::lexer::lexer::Token;
use anyhow::{anyhow, Ok, Result};

#[derive(PartialEq, Clone, Debug)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer_env: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer_env: None,
        }
    }
    pub fn new_enclosed_environment(&self) -> Self {
        Environment {
            store: HashMap::new(),
            outer_env: Some(Box::new(self.to_owned())),
        }
    }
    pub fn get(&self, name: &String) -> Result<Object> {
        let obj = self.store.get(name);
        match obj {
            Some(o) => return Ok(o.to_owned()),
            None => {
                if let Some(outer) = &self.outer_env {
                    let object = outer.store.get(name);
                    match object {
                        Some(o) => Ok(o.to_owned()),
                        None => Err(anyhow!(
                            "Error: Object does not exist in inner or outer environment"
                        )),
                    }
                } else {
                    Err(anyhow!("Error: Object not in env and no outer env exists"))
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    String(String),
    Null,
    Return(Box<Object>),
    Let(Box<Object>),
    Function(FunctionObject),
}
#[derive(PartialEq, Clone, Debug)]
pub struct FunctionObject {
    pub parameters: Option<Vec<Identifier>>,
    pub body: BlockStatement,
    pub environment: Environment,
}
impl FunctionObject {
    pub fn new(
        parameters: Option<Vec<Identifier>>,
        body: BlockStatement,
        environment: Environment,
    ) -> Self {
        FunctionObject {
            parameters,
            body,
            environment,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Object::Integer(u) => write!(f, "Integer Value: {}", u),
            Object::String(s) => write!(f, "String value: {}", s),
            Object::Boolean(b) => write!(f, "Bool value: {}", b),
            Object::Null => write!(f, "Null value"),
            Object::Return(o) => write!(f, "Return value: {}", o),
            Object::Let(l) => write!(f, "Let Value: {}", l),
            Object::Function(func) => write!(f, "Function Value: {:?}", func),
        };
    }
}

impl Object {
    pub fn eval(nodes: Vec<Statement>, env: &mut Environment) -> Self {
        let mut result: Result<Object> = Ok(Object::Null);
        for node in nodes.into_iter() {
            result = match node {
                Statement::Let(l) => {
                    // println!("This is the val: {:?}", l);

                    let val = Object::eval(vec![Statement::Expression(l.value)], env);
                    // println!("Eval value: {:?}", val);
                    match l.token {
                        Token::Ident(s) => env.store.insert(s, val.clone()),
                        _ => panic!("Wrong token here"),
                    };
                    Ok(val)
                }
                Statement::Return(r) => {
                    let val = Object::eval(vec![Statement::Expression(r.return_value)], env);
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
                    Expression::String(s) => Ok(Object::String(s.to_owned())),
                    Expression::Boolean(b) => match b {
                        Token::True => Ok(Object::Boolean(true)),
                        Token::False => Ok(Object::Boolean(false)),
                        _ => Err(anyhow!("Wrong token type. Expected Boolean, Got: {:?}", b)),
                    },
                    Expression::Prefix(p) => {
                        let right = Object::eval(vec![Statement::Expression(p.right)], env);
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
                        let left = Object::eval(vec![Statement::Expression(inf.left)], env);
                        let right = Object::eval(vec![Statement::Expression(inf.right)], env);

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
                            (Object::String(sl), Object::String(sr)) => match inf.token {
                                Token::Plus => Ok(Object::String(sl + &sr)),
                                _ => Err(anyhow!("Wrong operator used to concatinate strings")),
                            },
                            _ => Err(anyhow!("Wrong token type in infix")),
                        }
                    }
                    Expression::If(i) => {
                        let condition = Object::eval(vec![Statement::Expression(i.condition)], env);
                        // checking both if and else if for return statement to facilitiate nested
                        // block statements that have returns
                        if condition.is_truthy() {
                            let object = Object::eval(i.consequence.statements, env);
                            if object.expect_object_is(&Object::Return(Box::new(Object::Null))) {
                                return object;
                            };
                            Ok(object)
                        } else if let Some(alt) = i.alternative {
                            let object = Object::eval(alt.statements, env);
                            if object.expect_object_is(&Object::Return(Box::new(Object::Null))) {
                                return object;
                            };
                            Ok(object)
                        } else {
                            Ok(Object::Null)
                        }
                    }
                    Expression::Identifier(i) => match i {
                        Token::Ident(s) => env.get(&s),
                        _ => Err(anyhow!("Wrong token type for identifier")),
                    },
                    Expression::Fn(func) => {
                        let clone_env = env.clone();
                        Ok(Object::Function(FunctionObject::new(
                            func.parameters,
                            func.body,
                            clone_env,
                        )))
                    }
                    Expression::Call(call) => {
                        // Get function from call
                        let func = Object::eval(vec![Statement::Expression(call.function)], env);
                        // turn arguments into objects
                        let mut args: Vec<Object> = vec![];
                        if let Some(arguments) = call.arguments {
                            for arg in arguments.into_iter() {
                                let eval = Object::eval(vec![Statement::Expression(arg)], env);
                                args.push(eval)
                            }
                        }

                        //apply the fucntion
                        match func {
                            Object::Function(f) => {
                                let mut extended_env = f.environment.new_enclosed_environment();
                                // gets the params from function and adds the idents to extended_env
                                if let Some(params) = f.parameters {
                                    for (i, param) in params.into_iter().enumerate() {
                                        match param.token {
                                            Token::Ident(s) => {
                                                extended_env
                                                    .store
                                                    .insert(s.clone(), args[i].to_owned());
                                            }
                                            _ => todo!(),
                                        }
                                    }
                                }
                                let eval_body = Object::eval(f.body.statements, &mut extended_env);
                                match eval_body {
                                    Object::Return(r) => Ok(r.as_ref().to_owned()),
                                    _ => Ok(eval_body),
                                }
                            }
                            _ => Err(anyhow!("Not a function")),
                        }
                    } // _ => Ok(Object::Null),
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

    use super::Environment;

    #[test]
    fn test_closures() {
        struct Test {
            input: Vec<u8>,
            expected: Object,
        }
        let tests = vec![Test {
            input: "let newAdder = fn(x) {fn(y) { x + y };};let addTwo = newAdder(2);addTwo(2);"
                .into(),
            expected: Object::Integer(4),
        }];

        for test in tests.into_iter() {
            let evaluated = test_eval(test.input);
            assert_eq!(test.expected, evaluated);
        }
    }
    #[test]
    fn test_function() {
        struct Test {
            input: Vec<u8>,
            expected: Object,
        }
        let tests = vec![
            Test {
                input: "let identity = fn(x){x}; identity(5);".into(),
                expected: Object::Integer(5),
            },
            Test {
                input: "let add = fn(x,y){x+y;}; add(5+5, add(5,5));".into(),
                expected: Object::Integer(20),
            },
        ];

        for test in tests.into_iter() {
            let evaluated = test_eval(test.input);
            assert_eq!(test.expected, evaluated);
        }
    }
    #[test]
    fn test_let_statements() {
        struct Test {
            input: Vec<u8>,
            expected: Object,
        }
        let tests = vec![
            Test {
                input: "let a = 5; a;".into(),
                expected: Object::Integer(5),
            },
            Test {
                input: "let a = 5*5; a;".into(),
                expected: Object::Integer(25),
            },
            Test {
                input: "let a =5; let b = a; b".into(),
                expected: Object::Integer(5),
            },
            Test {
                input: "let a =5; let b =a; let c= a+b+5;".into(),
                expected: Object::Integer(15),
            },
        ];

        for test in tests.into_iter() {
            let evaluated = test_eval(test.input);
            assert_eq!(test.expected, evaluated);
        }
    }
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
        let mut env = Environment::new();

        return Object::eval(program.statements, &mut env);
    }
}
