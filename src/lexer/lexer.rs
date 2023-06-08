#[allow(dead_code)]
pub enum Token {
    Illegal,
    EOF,

    Ident(String),
    Int(String),

    Assign,
    Plus,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    char: u8,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Lexer {
        let mut lex = Lexer {
            input,
            position: 0,
            read_position: 0,
            char: 0,
        };
        return lex;
    }
}
