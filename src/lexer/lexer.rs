use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Token {
    Illegal,
    EOF,

    Ident(String),
    Int(isize),
    String(String),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Function,
    Let,
    If,
    Else,
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
            Token::String(s) => write!(f, "String: {}", s),

            Token::Assign => write!(f, "Assign"),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
            Token::Bang => write!(f, "Bang"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Slash => write!(f, "Slash"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::LessThan => write!(f, "LessThan"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "Not Equal"),

            Token::Comma => write!(f, "Commma"),
            Token::Semicolon => write!(f, "Semicolin"),

            Token::LParen => write!(f, "Left Paran"),
            Token::RParen => write!(f, "Right Paran"),
            Token::LBrace => write!(f, "Left Brace"),
            Token::RBrace => write!(f, "Right Brace"),
            Token::LBracket => write!(f, "Left Bracket"),
            Token::RBracket => write!(f, "Right Bracket"),

            Token::Function => write!(f, "Function"),
            Token::Let => write!(f, "Let"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'[' => Token::LBracket,
            b']' => Token::RBracket,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'-' => Token::Minus,
            b'*' => Token::Asterisk,
            b'/' => Token::Slash,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            0 => Token::EOF,
            b'\'' | b'"' => {
                let string = self.read_string();
                return Token::String(string);
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident: String = self.read_indetifier();
                return self.look_up_ident(ident);
            }
            b'0'..=b'9' => {
                return Token::Int(self.read_number());
            }

            _ => {
                println!("char: {:?}", self.ch as char);
                Token::Illegal
            }
        };

        self.read_char();
        return token;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn look_up_ident(&mut self, ident: String) -> Token {
        return match ident.as_str() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
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

    fn read_string(&mut self) -> String {
        self.read_char();
        let position = self.position;
        while self.ch != b'\'' && self.ch != b'"' && self.ch != 0 {
            self.read_char();
        }
        let end_postition = self.position;
        self.read_char();
        String::from_utf8_lossy(&self.input[position..end_postition]).into_owned()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> isize {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let buf = &self.input[position..self.position];
        let string = String::from_utf8_lossy(buf).to_string();
        string.parse().unwrap()
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::{Lexer, Token};
    #[test]
    fn test_string_token() -> Result<()> {
        let input: Vec<u8> = r#""Hello World""#.into();

        let mut lex = Lexer::new(input);
        let tokens = vec![Token::String(String::from("Hello World"))];

        for token in tokens.into_iter() {
            let tok = lex.next_token();
            println!("Expected:  {:?}, Got: {:?}", token, tok);
            assert_eq!(token, tok);
        }
        Ok(())
    }
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
10 == 10;
10 != 9;
"Hello World";
[1,2];
            "#;
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
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
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::GreaterThan,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(10),
            Token::Equal,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEqual,
            Token::Int(9),
            Token::Semicolon,
            Token::String(String::from("Hello World")),
            Token::Semicolon,
            Token::LBracket,
            Token::Int(1),
            Token::Comma,
            Token::Int(2),
            Token::RBracket,
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
