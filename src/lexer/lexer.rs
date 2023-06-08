use std::fmt::Display;

use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Illegal => write!(f, "Illegal"),
            Token::EOF => write!(f, "EOF"),

            Token::Ident(s) => write!(f, "Ident: {}", s),
            Token::Int(s) => write!(f, "Int: {}", s),

            Token::Assign => write!(f, "Assign"),
            Token::Plus => write!(f, "Plus"),

            Token::Comma => write!(f, "Commma"),
            Token::Semicolon => write!(f, "Semicolin"),

            Token::LParen => write!(f, "Left Paran"),
            Token::RParen => write!(f, "Right Paran"),
            Token::LBrace => write!(f, "Left Brace"),
            Token::RBrace => write!(f, "Right Brace"),

            Token::Function => write!(f, "Function"),
            Token::Let => write!(f, "Let"),
        };
    }
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Lexer {
        let mut lex = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lex.read_char();
        return lex;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'0' => Token::EOF,
            _ => Token::Illegal,
        };
        self.read_char();
        return token;
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{Lexer, Token};

    #[test]
    fn test_next_token() -> Result<()> {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        for token in tokens.into_iter() {
            let tok = lexer.next_token();
            println!("Expected:  {:?}, Got: {:?}", token, tok);
            assert_eq!(token, tok);
        }
        return Ok(());
    }
}
