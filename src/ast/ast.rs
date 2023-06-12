use crate::lexer::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(Let),
    Return,
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Program {
        return Program { statements };
    }
}

#[derive(Debug, PartialEq)]
pub struct Let {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: Token,
}

#[derive(Debug, PartialEq)]
pub struct Expression {}
