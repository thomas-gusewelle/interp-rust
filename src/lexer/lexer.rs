use std::fmt::Display;

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

    pub fn look_up_ident(&mut self, ident: String) -> Token {
        return match ident.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            _ => Token::Ident(ident),
        };
    }

    fn read_indetifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        let buf = &self.input[position..=self.position];
        return String::from_utf8_lossy(buf).into_owned();
    }

    fn skip_whitespace(&mut self) {
        if self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let buf = &self.input[position..=self.position];
        return String::from_utf8_lossy(buf).to_string();
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            0 => Token::EOF,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident: String = self.read_indetifier();
                return self.look_up_ident(ident);
            }
            b'0'..=b'9' => {
                let ident: String = self.read_number();
                return Token::Int(ident);
            }

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
