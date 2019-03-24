use crate::error::Report;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // Bracket types
    LeftBrace,      // '{'
    RightBrace,     // '}'
    LeftSquare,     // '['
    RightSquare,    // ']'
    LeftParen,      // '('
    RightParen,     // ')'

    // 1 character
    Slash,          // '/'
    Star,           // '*'
    Carat,          // '^'
    Percent,        // '%'
    Bang,           // '!'
    Colon,          // ':'
    Comma,          // ','
    Pipe,           // '|'
    Question,       // '?'

    // 1 or 2 character
    Plus,           // '+'
    PlusPlus,       // '++'

    Equal,          // '='
    EqualEqual,     // '=='

    Less,           // '<'
    LessEqual,      // '<='
    LeftArrow,      // '<-'

    Greater,        // '>'
    GreaterEqual,   // '>='

    Minus,          // '-'
    RightArrow,     // '->'

    // Keywords
    And,            // 'and'
    Or,             // 'or'
    Not,            // 'not'
    True,           // 'true'
    False,          // 'false'

    // Other
    Number,
    String,
    Identifier,
    Newline,

    None,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub col: u32,
    pub lexeme: String,
}

pub struct Scanner {
    pub source: Vec<char>,

    start: usize,
    current: usize,

    line: u32,
    col: u32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            col: 0,
        }
    }

    // Scan a single token from 'source'. 
    // Returns ScannerError on failure, due to:
    // - unrecognised character
    // - unrecognised escape sequence
    // - unterminated string literal
    pub fn scan_token(&mut self) -> Result<Token, ScannerError> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return Ok(self.make_token(TokenKind::Eof));
        }

        let c = self.advance();

        if c.is_digit(10) {
            return Ok(self.scan_number());
        }

        if is_identifier_start(c) {
            return Ok(self.scan_identifier());
        }

        match c {
            '{' => Ok(self.make_token(TokenKind::LeftBrace)),
            '}' => Ok(self.make_token(TokenKind::RightBrace)),
            '[' => Ok(self.make_token(TokenKind::LeftSquare)),
            ']' => Ok(self.make_token(TokenKind::RightSquare)),
            '(' => Ok(self.make_token(TokenKind::LeftParen)),
            ')' => Ok(self.make_token(TokenKind::RightParen)),
            
            '/' => Ok(self.make_token(TokenKind::Slash)),
            '*' => Ok(self.make_token(TokenKind::Star)),
            '^' => Ok(self.make_token(TokenKind::Carat)),
            '%' => Ok(self.make_token(TokenKind::Percent)),
            '!' => Ok(self.make_token(TokenKind::Bang)),
            ':' => Ok(self.make_token(TokenKind::Colon)),
            ',' => Ok(self.make_token(TokenKind::Comma)),
            '|' => Ok(self.make_token(TokenKind::Pipe)),
            '?' => Ok(self.make_token(TokenKind::Question)),

            '"' => self.scan_string(),

            '+' => {
                let t = if self.consume('+') {TokenKind::PlusPlus}
                        else {TokenKind::Plus};
                Ok(self.make_token(t))
            },

            '=' => {
                let t = if self.consume('=') {TokenKind::EqualEqual}
                        else {TokenKind::Equal};
                Ok(self.make_token(t))
            },

            '<' => {
                let t = if self.consume('=') {TokenKind::LessEqual}
                    else if self.consume('-') {TokenKind::LeftArrow}
                    else {TokenKind::Less};
                Ok(self.make_token(t))
            },

            '>' => {
                let t = if self.consume('=') {TokenKind::GreaterEqual}
                        else {TokenKind::Greater};
                Ok(self.make_token(t))
            },

            '-' => {
                let t = if self.consume('>') {TokenKind::RightArrow}
                        else {TokenKind::Minus};
                Ok(self.make_token(t))
            },

            '\n' => {
                self.col = 0;
                self.line += 1;

                while !self.is_at_end() && self.consume('\n') {
                    self.line += 1;
                }
                Ok(self.make_token(TokenKind::Newline))
            },

            _ => Err(self.make_error(format!("Rogue unrecognised character '{}'.", c))),
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                },

                '#' => {
                    while self.peek() != '\n' {
                        self.advance();
                    }
                }

                _ => {return;}
            }
        }
    }

    // Scan a number literal, e.g. '2.40' or '3'.
    fn scan_number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.make_token(TokenKind::Number)
    }

    // Scan either an identifier or a keyword.
    fn scan_identifier(&mut self) -> Token {
        while is_identifier_body(self.peek()) {
            self.advance();
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();
        
        self.make_token(match &lexeme[..] {
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Identifier,
        })
    }

    // Scan a string literal.
    // This may fail, in which case a ScannerError will be returned, due to:
    // - an unrecognised escape sequence
    // - an unterminated string literal
    fn scan_string(&mut self) -> Result<Token, ScannerError> {
        let mut lexeme = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            let c = self.advance();

            // Check for escape sequences
            if c == '\\' {
                let escape = self.advance();
                match escape {
                    'n' => lexeme.push('\n'),
                    't' => lexeme.push('\t'),
                    'r' => lexeme.push('\r'),
                    '"' => lexeme.push('"'),
                    _ => return Err(self.make_error(format!("Unrecognised escape sequence '\\{}'.", escape))),
                }
            } else {
                lexeme.push(c);
            }
        }

        if self.is_at_end() {
            return Err(self.make_error("Unterminated string literal.".to_string()));
        }

        self.advance();
        Ok(Token {
            kind: TokenKind::String,
            line: self.line,
            col: self.col,
            lexeme,
        })
    }

    // Create a new Token of the specified type at the current position
    fn make_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            line: self.line,
            col: self.col,
            lexeme: self.source[self.start..self.current].iter().collect(),
        }
    }

    // Create a new ScannerError at the current position
    fn make_error(&self, msg: String) -> ScannerError {
        ScannerError {
            msg,
            place: self.source[self.current],
            line: self.line,
            col: self.col,
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.col += 1;
        c
    }

    fn peek(&self) -> char {
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        self.source[self.current + 1]
    }

    fn consume(&mut self, expected: char) -> bool {
        if self.peek() == expected {
            self.advance();
            return true;
        }

        false
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier_body(c: char) -> bool {
    return is_identifier_start(c) || c.is_digit(10)
}

#[derive(Debug)]
pub struct ScannerError {
    msg: String,
    place: char,
    line: u32,
    col: u32,
}

impl Report for ScannerError {
    fn position(&self) -> (u32, u32) {
        (self.line, self.col)
    }

    fn place(&self) -> String {
        format!(" at '{}'", self.place)
    }

    fn message(&self) -> &String {
        &(self.msg)
    }
}