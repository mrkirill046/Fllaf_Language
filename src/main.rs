use std::env;
use std::fs::File;
use std::io::{self, Read};

mod lexer;
mod token;

use crate::lexer::Lexer;
use crate::token::Token;

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).expect("No input file specified. Use fllaf <filename>.fll");
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut lexer = Lexer::new(&contents);

    loop {
        let token = lexer.next_token();

        if token == Token::EndOfFile {
            break;
        }

        println!("{:?}", token);
    }

    Ok(())
}
