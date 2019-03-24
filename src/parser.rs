use crate::ast::{Decl, Expr, Pattern, Literal, ExprKind, ConstantExpr, UnaryExpr};
use crate::scanner::{Scanner, Token, TokenKind};
use crate::typ::Typ;
use crate::error::Report;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Parser {
    scanner: Scanner,

    previous: Token,
    current: Token,
}

impl Parser {
    pub fn new(source: String) -> Parser {
        Parser {
            scanner: Scanner::new(source),
            previous: Token { kind: TokenKind::None, line: 0, col: 0, lexeme: String::new() },
            current: Token { kind: TokenKind::None, line: 0, col: 0, lexeme: String::new() },
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        self.advance();
        let mut ast: Vec<Expr> = Vec::new();

        while !self.is_at_end() {
            let expr = self.parse_precedence(Precedence::Or);
            if let Ok(expr) = expr {
                ast.push(expr);
            } else if let Err(err) = expr {
                err.report_in(&self.scanner.source);
                self.advance();
            };
        }

        ast
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<Expr, ParserError> {
        self.advance();
        let prefix_fn = get_parse_rule(self.previous.kind).prefix;

        if let Some(prefix_fn) = prefix_fn {
            let mut expr = prefix_fn(self)?;

            while get_parse_rule(self.previous.kind).precedence >= precedence {
                self.advance();
                
                let infix_fn = get_parse_rule(self.current.kind).infix;

                if let Some(infix_fn) = infix_fn {
                    self.advance();
                    expr = infix_fn(self, expr)?;
                }
            }

            return Ok(expr);
        }

        Err(self.make_error_at(&self.previous, "Expected expression.".to_string()))
    }

    fn parse_number(&mut self) -> Result<Expr, ParserError> {
        if self.previous.lexeme.contains(".") {
            let value = self.previous.lexeme.parse::<f64>().unwrap();
            Ok(Literal::Float(value).into())
        } else {
            let value = self.previous.lexeme.parse::<i32>().unwrap();
            Ok(Literal::Int(value).into())
        }
    }

    fn parse_bool(&mut self) -> Result<Expr, ParserError> {
        let value = match self.previous.kind {
            TokenKind::True => true,
            TokenKind::False => false,
            _ => unreachable!(),
        };

        Ok(Literal::Bool(value).into())
    }

    fn parse_string(&mut self) -> Result<Expr, ParserError> {
        let value = String::from(self.previous.lexeme.clone());
        Ok(Literal::String(Box::new(value)).into())
    }

    fn parse_constant(&mut self) -> Result<Expr, ParserError> {
        Ok(ConstantExpr{name: self.previous.clone()}.into())
    }

    fn parse_unary(&mut self) -> Result<Expr, ParserError> {
        let operator = self.previous.clone();
        let operand = self.parse_precedence(Precedence::Unary)?;
        Ok(UnaryExpr{operator, operand}.into())
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();
        let mut token = self.scanner.scan_token();

        if let Ok(token) = token {
            self.current = token;
        } else {
            while let Err(err) = token { 
                err.report_in(&self.scanner.source);
                token = self.scanner.scan_token();
            }
            
            if let Ok(token) = token {
                self.current = token;
            }
        }

    }

    fn check(&self, kind: TokenKind) -> bool {
        self.current.kind == kind
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            return true;
        }

        false
    }

    fn expect(&mut self, kind: TokenKind, msg: String) -> Result<(), ParserError> {
        if !self.consume(kind) {
            return Err(self.make_error_at(&self.current, msg));
        }

        return Ok(())
    }

    fn make_error_at(&self, token: &Token, msg: String) -> ParserError {
        ParserError {
            token: token.clone(),
            msg
        }
    }

    fn is_at_end(&self) -> bool {
        self.current.kind == TokenKind::Eof
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Precedence {
    None,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    ApplyRight, // fn <- arg
    ApplyLeft, // arg -> fn
    Unary,
    Primary,
}

struct ParseRule {
    precedence: Precedence,
    prefix: Option<fn(&mut Parser) -> Result<Expr, ParserError>>,
    infix: Option<fn(&mut Parser, Expr) -> Result<Expr, ParserError>>,
}

lazy_static! {
    static ref PARSE_TABLE: HashMap<TokenKind, ParseRule> =
    vec![
        (TokenKind::Number, ParseRule {
            precedence: Precedence::None,
            prefix: Some(Parser::parse_number), 
            infix: None,
        }),

        (TokenKind::True, ParseRule {
            precedence: Precedence::None,
            prefix: Some(Parser::parse_bool), 
            infix: None,
        }),

        (TokenKind::False, ParseRule {
            precedence: Precedence::None,
            prefix: Some(Parser::parse_bool), 
            infix: None,
        }),

        (TokenKind::String, ParseRule {
            precedence: Precedence::None,
            prefix: Some(Parser::parse_string), 
            infix: None,
        }),

        (TokenKind::Identifier, ParseRule {
            precedence: Precedence::None,
            prefix: Some(Parser::parse_constant), 
            infix: None,
        }),

        (TokenKind::Minus, ParseRule {
            precedence: Precedence::None,
            prefix: Some(Parser::parse_unary), 
            infix: None,
        }),

        (TokenKind::Bang, ParseRule {
            precedence: Precedence::None,
            prefix: Some(Parser::parse_unary), 
            infix: None,
        }),
    ].into_iter().collect();
}

fn get_parse_rule(kind: TokenKind) -> &'static ParseRule {
    if let Some(rule) = PARSE_TABLE.get(&kind) {
        rule
    } else {
        &(ParseRule {
            precedence: Precedence::None,
            prefix: None, 
            infix: None,
        })
    }
}

pub struct ParserError {
    token: Token,
    msg: String,
}

impl Report for ParserError {
    fn position(&self) -> (u32, u32) {
        (self.token.line, self.token.col)
    }

    fn message(&self) -> &String {
        &(self.msg)
    }

    fn place(&self) -> String {
        format!(" at {}", match self.token.kind {
            TokenKind::Newline => "newline".to_string(),
            TokenKind::Eof => "end".to_string(),
            _ => format!("'{}'", self.token.lexeme),
        })
    }
}