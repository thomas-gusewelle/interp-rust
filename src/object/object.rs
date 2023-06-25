use std::fmt::Display;

use crate::ast::ast::{Expression, Statement};

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
        let result = Result<Object::Null>;
        for node in nodes.into_iter() {
            let result = match node {
                Statement::Let(l) => Err(anyhow("Todo"))
                Statement::Return(r) => todo!(),
                Statement::Expression(e) => match e {
                    Expression::Integer(i) => match i {
                        Token::Integer(int) => Object::Integer(int),
                        _ => todo!(),
                    },
                    _ => todo!(),
                };
            }
        }
    }
}
