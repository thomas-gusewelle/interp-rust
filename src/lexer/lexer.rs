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
    Minus,
    Bang,
    Asterisk,
    Slash,
    GreaterThan,
    LessThan,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    If,
    Return,
    True,
    False,
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
            Token::Minus => write!(f, "Minus"),
            Token::Bang => write!(f, "Bang"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Slash => write!(f, "Slash"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::LessThan => write!(f, "LessThan"),

            Token::Comma => write!(f, "Commma"),
            Token::Semicolon => write!(f, "Semicolin"),

            Token::LParen => write!(f, "Left Paran"),
            Token::RParen => write!(f, "Right Paran"),
            Token::LBrace => write!(f, "Left Brace"),
            Token::RBrace => write!(f, "Right Brace"),

            Token::Function => write!(f, "Function"),
            Token::Let => write!(f, "Let"),
            Token::If => write!(f, "If"),
            Token::Return => write!(f, "Return"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
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
            "return" => Token::Return,
            "if" => Token::If,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident),
        };
    }

    fn read_indetifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        let buf = &self.input[position..self.position];
        return String::from_utf8_lossy(buf).into_owned();
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let buf = &self.input[position..self.position];
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
            b'!' => Token::Bang,
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            0 => Token::EOF,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident: String = self.read_indetifier();
                return self.look_up_ident(ident);
            }
            b'0'..=b'9' => {
                let ident: String = self.read_number();
                return Token::Int(ident);
            }

            _ => {
                println!("char: {:?}", self.ch as char);
                Token::Illegal
            }
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
        let input = r#"let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            
            if (5 < 10) {
            return true;
            } else {
            return false;
}
            "#;
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::LessThan,
            Token::Int(String::from("10")),
            Token::GreaterThan,
            Token::Int(String::from("5")),
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
