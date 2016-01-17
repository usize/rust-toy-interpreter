#![allow(dead_code)]

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType {
    Int,
    Float,
    Str,
    Identifier,
    Equals,
    Let,
    Function,
    Return,
    LPar,
    RPar,
    LCBrace,
    RCBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Period,
    BinOp
}

#[derive(Debug)]
pub enum BinOp {
    Mul,
    Div,
    Plus,
    Min,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    token_type : TokenType,
    value      : String,
    row        : u32,
    col        : u32,
}

impl Token {

    fn new(token_type: TokenType, value: String, row: u32, col: u32) -> Token {
        return Token{token_type: token_type, value: value, row: row, col: col};
    }

}

pub struct Lexer {
    tokens: Vec<Token>,
    lines:  u32,
    start_pos: usize,
    cursor: usize,
    top: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        return Lexer{
            tokens:     Vec::new(),
            lines:      0,
            start_pos:  0,
            cursor:     0,
            top:        0,
        }
    }

    pub fn from(tokens: Vec<Token>) -> Lexer {
        return Lexer{
            tokens:     tokens,
            lines:      0,
            start_pos:  0,
            cursor:     0,
            top:        0,
        }
    }

    pub fn tokens(&self) -> &Vec<Token> {
        return &self.tokens;
    }

    pub fn reset(&mut self) {
        self.tokens = Vec::new();
        self.lines = 0;
        self.start_pos = 0;
        self.cursor = 0;
        self.top = 0;
    }

    fn add_token(&mut self, token_type: TokenType, line: &str) {
        self.tokens.push(
            Token::new(
                token_type,
                String::from(&line[self.start_pos..self.cursor]),
                self.lines,
                self.cursor as u32
            )
        );
        // go ahead and fix our substring window
        self.start_pos = self.cursor;
    }

    fn chr(&self, line: &str) -> char {
        match line.chars().nth(self.cursor) {
            Some(value) => return value,
            None => return ' ',
        }
    }
    fn peek(&self, line: &str) -> char {
        match line.chars().nth(self.cursor + 1) {
            Some(value) => return value,
            None => return ' ',
        }
    }

    fn skip_whitespace(&mut self, line: &str) {
        while self.chr(line) == ' ' && self.cursor < line.len() - 1 {
            self.cursor += 1;
            self.start_pos += 1;
        }
    }

    // Matches a string to a binop, along with its precedence
    pub fn bin_op(v: &str) -> Option<(BinOp, u8)> {
        match v {
            "+" => Some((BinOp::Plus, 2)),
            "-" => Some((BinOp::Min, 2)),
            "*" => Some((BinOp::Mul, 1)),
            "/" => Some((BinOp::Div, 1)),
            _ => None
        }
    }

    fn is_binop(&mut self, line: &str) -> bool {
        match Lexer::bin_op(&line[self.start_pos..self.cursor + 1]) {
            Some(_) => true,
            None => false
        }
    }

    fn keyword(&mut self, line: &str) -> Option<TokenType> {
        match &line[self.start_pos..self.cursor] {
            "let"      => Some(TokenType::Let),
            "function" => Some(TokenType::Function),
            "return"   => Some(TokenType::Return),
             _ => None
        }
    }

    // returns true if there is still a "next" token
    pub fn next_token(&mut self) -> bool {
        if self.top + 1 < self.tokens.len() {
            self.top += 1;
            return true;
        }
        return false;
    }

    // returns true if there is still a "prev" token
    pub fn prev_token(&mut self) -> bool {
        if self.top != 0 {
            self.top -= 1;
            return true;
        }
        return false;
    }

    pub fn tokens_remaining(&self) -> usize {
        return (self.tokens.len() - 1) - self.top;
    }

    pub fn curr_token(&self) -> &Token {
        return &self.tokens[self.top];
    }

    pub fn curr_type(&self) -> &TokenType {
        return &self.curr_token().token_type;
    }

    pub fn curr_value(&self) -> String {
        return self.curr_token().value.clone();
    }

    pub fn current_is_type(&self, t: TokenType) -> bool {
        return self.curr_token().token_type == t;
    }

    pub fn match_token(&self, t: TokenType) -> Result<&Token, String> {
        if self.curr_token().token_type == t {
            return Ok(self.curr_token());
        }
        return Err(format!("expected: {:?} , found: {:?}", t, self.curr_token()));
    }

    pub fn tokenize(&mut self, lines: String) -> Result<(), String>{
        for line in lines.lines() {
            self.lines += 1;
            self.start_pos = 0;
            self.cursor = 0;

            while self.cursor < line.len() {
                // skip whitespace
                self.skip_whitespace(line);

                // Strings
                if self.chr(line) == '"' || self.chr(line) == '\'' {
                    let delim = self.chr(line);
                    self.start_pos += 1;
                    self.cursor += 1;
                    // TODO: handle escaping
                    while self.chr(line) != delim {
                        self.cursor += 1;
                    }
                    self.add_token(TokenType::Str, line);
                    self.start_pos += 1;
                    self.cursor += 1;
                    continue;
                }

                // Identifiers .. Keywords
                if self.chr(line).is_alphabetic() {
                    while self.chr(line).is_alphabetic() {
                        self.cursor += 1;
                    }
                    match self.keyword(line) {
                        Some(tt) => self.add_token(tt, line),
                        None     => self.add_token(TokenType::Identifier, line)
                    }
                    continue;
                }

                // IntS 'N FloatS
                if self.chr(line).is_digit(10) ||
                   self.chr(line) == '.' && self.peek(line).is_digit(10) ||
                   self.chr(line) == '-' && self.peek(line).is_digit(10) {
                    if self.chr(line) == '-' {
                        self.cursor += 1;
                    }
                    while self.chr(line).is_digit(10) {
                        self.cursor += 1;
                    }
                    if self.chr(line) == '.' && self.peek(line).is_digit(10) {
                        self.cursor += 1;
                        while self.chr(line).is_digit(10) {
                            self.cursor += 1;
                        }
                        self.add_token(TokenType::Float, line);
                        continue;
                    }
                    self.add_token(TokenType::Int, line);
                    continue;
                }

                // BinOpS || UNARYOPS
                if self.is_binop(line) {
                    self.cursor += 1;
                    self.add_token(TokenType::BinOp, line);
                    continue;
                }

                // MISC
                match self.chr(line) {
                    '='  => {
                        self.cursor += 1;
                        self.add_token(TokenType::Equals, line);
                        continue;
                    },
                    '('  => {
                        self.cursor += 1;
                        self.add_token(TokenType::LPar, line);
                        continue;
                    },
                    ')'  => {
                        self.cursor += 1;
                        self.add_token(TokenType::RPar, line);
                        continue;
                    },
                    '{'  => {
                        self.cursor += 1;
                        self.add_token(TokenType::LCBrace, line);
                        continue;
                    },
                    '}'  => {
                        self.cursor += 1;
                        self.add_token(TokenType::RCBrace, line);
                        continue;
                    },
                    '['  => {
                        self.cursor += 1;
                        self.add_token(TokenType::LBracket, line);
                        continue;
                    },
                    ']'  => {
                        self.cursor += 1;
                        self.add_token(TokenType::RBracket, line);
                        continue;
                    },
                    ':'  => {
                        self.cursor += 1;
                        self.add_token(TokenType::Colon, line);
                        continue;
                    },
                    ','  => {
                        self.cursor += 1;
                        self.add_token(TokenType::Comma, line);
                        continue;
                    },
                    '.'  => {
                        self.cursor += 1;
                        self.add_token(TokenType::Period, line);
                        continue;
                    },
                    '\n' => break,
                    '\0' => break,
                    _    => {
                        return Err(format!("unknown symbol: {}, ln: {} col: {}",
                                            &line[self.start_pos..line.len()],
                                            self.lines, self.start_pos));
                    }
                }

            }
        }
        return Ok(());
    }
}
