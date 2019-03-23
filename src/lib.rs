mod scanner;
mod ast;
mod typ;

use std::io;
use std::io::Write;
use std::fs;

use crate::scanner::{Scanner, TokenKind};

pub fn run(source: String) {
    let mut scanner = Scanner::new(source);
    
    let mut t = scanner.scan_token().unwrap();
    println!("{:?}", t);

    while t.kind != TokenKind::Eof {
        t = scanner.scan_token().unwrap();
        println!("{:?}", t);
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