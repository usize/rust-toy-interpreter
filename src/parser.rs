use lexer::*;
use value::*;

#[derive(Debug)]
#[derive(Clone)]
pub struct BinaryOp {
    pub l_expr: Expr,
    pub op: BinOp,
    pub r_expr: Expr,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Expr {
    Atom(Value),
    BinaryOperation(Box<BinaryOp>),
    GetName(String),
    Function{name: Option<String>, args: Vec<String>, body: Vec<Statement>},
    Call(Vec<Expr>),
    Return(Box<Expr>),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct DefLet {
    pub name: String,
    pub expr: Expr
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Statement {
    Expression(Expr),
    Assignment(DefLet),
    If{cond: Expr, body: Vec<Statement>},
    IfElse{cond: Expr, body: Vec<Statement>, else_body: Vec<Statement>},
}

pub struct Parser {
    lexer:  Lexer
}

impl Parser {
    pub fn new() -> Parser {
        return Parser{lexer: Lexer::new()};
    }

    pub fn from(tokens: Vec<Token>) -> Parser {
        return Parser{lexer: Lexer::from(tokens)};
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        match *self.lexer.curr_type() {
            TokenType::Int => {
                let int = self.lexer.curr_value().parse::<i32>().unwrap();
                return Ok(Expr::Atom(Value::Int(int)));
            },
            TokenType::Float => {
                let float = self.lexer.curr_value().parse::<f32>().unwrap();
                return Ok(Expr::Atom(Value::Float(float)));
            },
            TokenType::Str => {
                return Ok(Expr::Atom(Value::Str(self.lexer.curr_value())));
            },
            TokenType::Identifier => {
                let e1 = Expr::GetName(self.lexer.curr_value());
                if self.lexer.next_token() {
                    match *self.lexer.curr_type() {
                        TokenType::BinOp => return Ok(try!(self.parse_binop(e1))),
                        TokenType::LPar  => {
                            let mut expr_stack = Vec::new();
                            expr_stack.push(e1);
                            self.lexer.next_token();
                            while !self.lexer.current_is_type(TokenType::RPar) {
                                expr_stack.push(try!(self.parse_expression()));
                                if self.lexer.current_is_type(TokenType::Comma) {
                                    self.lexer.next_token();
                                }
                            }
                            return Ok(Expr::Call(expr_stack));
                        },
                        _ => return Ok(e1)
                    }
                }
                return Ok(e1);
            },
            TokenType::LPar => {
                self.lexer.next_token();
                let e = self.parse_expression();
                try!(self.lexer.match_token(TokenType::RPar));
                return e;
            },
            TokenType::Function => {
                self.lexer.next_token();
                let name : Option<String>;
                if self.lexer.current_is_type(TokenType::Identifier) {
                    name = Some(self.lexer.curr_value());
                    self.lexer.next_token();
                } else {
                    name = None;
                }
                try!(self.lexer.match_token(TokenType::LPar));
                self.lexer.next_token();
                let mut args = Vec::new();
                while self.lexer.current_is_type(TokenType::Identifier) {
                    args.push(self.lexer.curr_value());
                     self.lexer.next_token();
                     if self.lexer.current_is_type(TokenType::Comma) {
                         self.lexer.next_token();
                     }
                }
                try!(self.lexer.match_token(TokenType::RPar));
                self.lexer.next_token();
                let body = try!(self.parse_block());
                self.lexer.next_token();
                return Ok(Expr::Function{name: name, args: args, body: body});
            },
            TokenType::Return => {
                self.lexer.next_token();
                let e = try!(self.parse_expression());
                return Ok(Expr::Return(Box::new(e)));
            },
            _ => Err(String::from("unrecognized expression"))
        }
    }

    // TODO: precedences ~!!@
    fn parse_binop(&mut self, e1: Expr) -> Result<Expr, String> {
        let (op, _) = Lexer::bin_op(&self.lexer.curr_value()[..]).unwrap();
        self.lexer.next_token();
        let r_expr = try!(self.parse_expression());
        let bop = BinaryOp{l_expr: e1, op: op, r_expr: r_expr};
        return Ok(Expr::BinaryOperation(Box::new(bop)));
    }

    // Wrap in a while loop
    fn parse_expression(&mut self) -> Result<Expr, String> {
        let e1 = try!(self.parse_term());
        self.lexer.next_token();
        if self.lexer.tokens_remaining() > 0 &&
           self.lexer.current_is_type(TokenType::BinOp) {
            return self.parse_binop(e1);
        }
        return Ok(e1);
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match *self.lexer.curr_type() {
            TokenType::Let => {
                self.lexer.next_token();
                try!(self.lexer.match_token(TokenType::Identifier));
                let name = self.lexer.curr_value();
                self.lexer.next_token();
                self.lexer.match_token(TokenType::Equals).unwrap();
                self.lexer.next_token();
                let e = self.parse_expression().unwrap();
                return Ok(Statement::Assignment(DefLet{name: name, expr: e}));
            },
            TokenType::If => {
                self.lexer.next_token();
                try!(self.lexer.match_token(TokenType::LPar));
                self.lexer.next_token();
                let cond = try!(self.parse_expression());
                try!(self.lexer.match_token(TokenType::RPar));
                self.lexer.next_token();
                let body = try!(self.parse_block());
                self.lexer.next_token();
                if self.lexer.current_is_type(TokenType::Else) {
                    self.lexer.next_token();
                    let else_body = try!(self.parse_block());
                    return Ok(Statement::IfElse{
                        cond: cond,
                        body: body,
                        else_body: else_body
                    });
                }
                return Ok(Statement::If{cond: cond, body: body});
            },
            _ => {
                let e = try!(self.parse_expression());
                return Ok(Statement::Expression(e));
            },
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        try!(self.lexer.match_token(TokenType::LCBrace));
        let mut block_tokens = Vec::new();
        let mut brace_count = 1;
        while self.lexer.next_token() {
            if self.lexer.current_is_type(TokenType::LCBrace) {
                brace_count += 1;
            }
            if self.lexer.current_is_type(TokenType::RCBrace) {
                brace_count -= 1;
            }
            if brace_count < 1 {
                break;
            }
            block_tokens.push(self.lexer.curr_token().clone());
        }
        try!(self.lexer.match_token(TokenType::RCBrace));
        return Parser::parse_from_tokens(block_tokens);
    }

    fn parse_from_tokens(tokens: Vec<Token>) -> Result<Vec<Statement>, String> {
        let mut block_parser = Parser::from(tokens);
        return block_parser.parse_program();
    }

    pub fn parse_program(&mut self) -> Result<Vec<Statement>, String> {
        println!("Lexer: \n\t{:?}", self.lexer.tokens());
        let mut program = Vec::new();
        if self.lexer.tokens().len() > 0 {
            loop {
                program.push(try!(self.parse_statement()));
                if self.lexer.tokens_remaining() < 1 {
                   return Ok(program);
                }
                self.lexer.next_token();
            }
        }
        return Ok(program);
    }

    pub fn parse_lines(&mut self, text: String) -> Result<Vec<Statement>, String> {
        try!(self.lexer.tokenize(text.clone()));
        let program = self.parse_program();
        self.lexer.reset();
        return program;
    }
}
