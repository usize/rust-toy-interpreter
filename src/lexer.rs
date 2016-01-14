#![allow(dead_code)]

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    INT,
    FLOAT,
    IDENTIFIER,
    BINOP
}

#[derive(Debug)]
pub enum BinOp {
    PLUS,
    MIN,
    MUL,
    DIV
}

#[derive(Debug)]
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
        while self.chr(line) == ' ' {
            self.cursor += 1;
            self.start_pos += 1;
        }
    }

    pub fn bin_op(v: &str) -> Option<BinOp> {
        match v {
            "+" => Some(BinOp::PLUS),
            "-" => Some(BinOp::MIN),
            "*" => Some(BinOp::MUL),
            "/" => Some(BinOp::DIV),
            _ => None
        }
    }

    fn is_binop(&mut self, line: &str) -> bool {
        match Lexer::bin_op(&line[self.start_pos..self.cursor + 1]) {
            Some(_) => true,
            None => false
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

    pub fn curr_token(&self) -> &Token {
        return &self.tokens[self.top];
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

                // IDENTIFIERS .. KEYWORDS
                if self.chr(line).is_alphabetic() {
                    while self.chr(line).is_alphabetic() {
                        self.cursor += 1;
                    }
                    self.add_token(TokenType::IDENTIFIER, line);
                    continue;
                }

                // INTS 'N FLOATS
                if self.chr(line).is_digit(10) {
                    while self.chr(line).is_digit(10) {
                        self.cursor += 1;
                    }
                    if self.chr(line) == '.' && self.peek(line).is_digit(10) {
                        self.cursor += 1;
                        while self.chr(line).is_digit(10) {
                            self.cursor += 1;
                        }
                        self.add_token(TokenType::FLOAT, line);
                        continue;
                    }
                    self.add_token(TokenType::INT, line);
                    continue;
                }

                // BINOPS || UNARYOPS
                if self.is_binop(line) {
                    self.cursor += 1;
                    self.add_token(TokenType::BINOP, line);
                    continue;
                }
                return Err(format!("unknown symbol: {} , line: {} column: {}",
                                    &line[self.start_pos..line.len()],
                                    self.lines, self.start_pos));
            }
        }
        return Ok(());
    }
}
