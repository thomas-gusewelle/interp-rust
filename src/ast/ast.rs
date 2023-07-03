use crate::lexer::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(Let),
    Return(Return),
    Expression(Expression),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(Token),
    Integer(Token),
    String(Token),
    Boolean(Token),
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
    If(Box<IfExpression>),
    Fn(Box<FnExpression>),
    Call(Box<CallExpression>),
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Expression,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Expression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Self {
        IfExpression {
            token,
            condition,
            consequence,
            alternative,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Statement>) -> Self {
        BlockStatement { token, statements }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FnExpression {
    pub token: Token,
    pub parameters: Option<Vec<Identifier>>,
    pub body: BlockStatement,
}

impl FnExpression {
    pub fn new(token: Token, parameters: Option<Vec<Identifier>>, body: BlockStatement) -> Self {
        FnExpression {
            token,
            parameters,
            body,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: Expression,
    pub arguments: Option<Vec<Expression>>,
}

impl CallExpression {
    pub fn new(token: Token, function: Expression, arguments: Option<Vec<Expression>>) -> Self {
        CallExpression {
            token,
            function,
            arguments,
        }
    }
}
