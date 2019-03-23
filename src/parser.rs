use crate::ast::{Decl, Expr, Pattern};
use crate::scanner::{Scanner, Token, TokenKind};
use crate::error::Reportable;

use std::collections::HashMap;

pub struct Parser {
    scanner: Scanner,

    last: Token,
    current: Token,
}

impl Parser {

}

pub enum Precedence {
    None,
}

struct ParseRule {
    precedence: Precedence,
    prefix: Option<fn(&mut Parser) -> Expr>,
    infix: Option<fn(&mut Parser, Expr) -> Expr>,
}

lazy_static! {
    static ref PARSE_TABLE: HashMap<TokenKind, ParseRule> =
    vec![
        // Placeholder example
        (TokenKind::LeftBrace, ParseRule {
            precedence: Precedence::None,
            prefix: None, 
            infix: None,
        }),
    ].into_iter().collect();
}


pub struct ParserError {
    line: u32,
    col: u32,
    msg: String,
}

impl Reportable for ParserError {
    fn position(&self) -> (u32, u32) {
        return (self.line, self.col);
    }

    fn message(&self) -> &String {
        return &(self.msg);
    }
}