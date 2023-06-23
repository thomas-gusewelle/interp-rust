use std::io::Write;

use crate::{lexer::lexer::Lexer, parser::parser::Parser};

pub fn start() {
    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string).unwrap();
        if input_string.is_empty() {
            return;
        }
        let lexer = Lexer::new(input_string.into_bytes());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        if parser.errors().len() != 0 {
            println!("There was an error in the program");
            continue;
        }

        println!("{:?}", program);
    }
}
