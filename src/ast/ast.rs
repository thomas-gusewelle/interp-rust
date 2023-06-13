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

impl Let {
    pub fn new(token: Token, name: Identifier, value: Expression) -> Let {
        Let { token, name, value }
    }
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: Token,
}
impl Identifier {
    pub fn new(token: Token) -> Identifier {
        Identifier { token }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {}
