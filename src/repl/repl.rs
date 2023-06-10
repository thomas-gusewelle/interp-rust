use std::io::Write;

use crate::lexer::lexer::{Lexer, Token};

pub fn start() {
    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).unwrap();
        if input_string.is_empty() {
            return;
        }
        let mut lexer = Lexer::new(input_string.into_bytes());
        loop {
            let token = lexer.next_token();
            println!("{}", token);
            if token == Token::EOF {
                break;
            }
        }
    }
}
