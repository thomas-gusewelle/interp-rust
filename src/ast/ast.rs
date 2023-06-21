use crate::lexer::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(Let),
    Return(Return),
    Expression(Expression),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Token),
    Integer(Token),
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
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
pub struct Return {
    pub token: Token,
    pub return_value: Expression,
}
impl Return {
    pub fn new(token: Token, return_value: Expression) -> Return {
        Return {
            token,
            return_value,
        }
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

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub right: Expression,
}

impl PrefixExpression {
    pub fn new(token: Token, right: Expression) -> Self {
        PrefixExpression { token, right }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpression {
    pub left: Expression,
    pub token: Token,
    pub right: Expression,
}

impl InfixExpression {
    pub fn new(left: Expression, token: Token, right: Expression) -> Self {
        InfixExpression { left, token, right }
    }
}
