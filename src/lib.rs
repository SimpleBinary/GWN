#[macro_use]
extern crate lazy_static;

mod scanner;
mod parser;
mod error;
mod ast;
mod typ;

use std::io;
use std::io::Write;
use std::fs;

use crate::scanner::{Scanner, TokenKind};
use crate::parser::Parser;

pub fn run(source: String) {
    let mut parser = Parser::new(source);
    let ast = parser.parse();
    for node in ast {
        println!("{:?}", node);
    }
}

pub fn run_file(filename: String) {
    let contents = 
        fs::read_to_string(filename).unwrap();
    
    run(contents);
}

pub fn run_repl() {
    loop {
        print!("gwn > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Unable to read line.");

        run(input);
    }
}