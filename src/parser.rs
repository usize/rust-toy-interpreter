use lexer::*;
use value::*;
use ast::*;

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
                return Ok(Expr::Atom(Value::Number(int as f64)));
            },
            TokenType::Float => {
                let float = self.lexer.curr_value().parse::<f64>().unwrap();
                return Ok(Expr::Atom(Value::Number(float)));
            },
            TokenType::Str => {
                return Ok(Expr::Atom(Value::Str(self.lexer.curr_value())));
            },
            TokenType::True => {
                return Ok(Expr::Atom(Value::Bool(true)));
            },
            TokenType::False => {
                return Ok(Expr::Atom(Value::Bool(false)));
            },
            TokenType::Identifier => {
                let e1 = Expr::GetName(self.lexer.curr_value());
                if self.lexer.next_token() {
                    match *self.lexer.curr_type() {
                        TokenType::BinOp => return self.parse_binop(e1),
                        TokenType::LPar  => return self.parse_call(e1),
                        _ => {
                            self.lexer.prev_token();
                            return Ok(e1)
                        }
                    }
                }
                return Ok(e1);
            },
            TokenType::LPar => {
                self.lexer.next_token();
                let e = try!(self.parse_expression());
                self.lexer.next_token();
                try!(self.lexer.match_token(TokenType::RPar));
                self.lexer.next_token();
                if self.lexer.current_is_type(TokenType::LPar) {
                    return self.parse_call(e);
                }
                self.lexer.prev_token();
                return Ok(e);
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

    fn parse_call(&mut self, e1: Expr) -> Result<Expr, String> {
        let mut expr_stack = Vec::new();
        expr_stack.push(e1);
        self.lexer.next_token();
        while !self.lexer.current_is_type(TokenType::RPar) {
            expr_stack.push(try!(self.parse_expression()));
            self.lexer.next_token();
            if self.lexer.current_is_type(TokenType::Comma) {
                self.lexer.next_token();
            }
        }
        return Ok(Expr::Call(expr_stack));
    }

    fn parse_binop(&mut self, e1: Expr) -> Result<Expr, String> {
        let mut expr_list = vec!(e1);
        let mut op_list : Vec<(BinOp, u8)> = Vec::new();

        op_list.push(Lexer::bin_op(&self.lexer.curr_value()[..]).unwrap());
        self.lexer.next_token();
        expr_list.push(try!(self.parse_term()));

        let mut end = false;
        while expr_list.len() > 1 {
            if !end && self.lexer.next_token() {
                if !self.lexer.current_is_type(TokenType::BinOp) {
                    self.lexer.prev_token();
                    end = true;
                    continue;
                }

                let (op2, prec2) = Lexer::bin_op(&self.lexer.curr_value()[..]).unwrap();

                if prec2 > op_list.last().unwrap().1 {

                    let e1 = expr_list.pop().unwrap();
                    let e2 = expr_list.pop().unwrap();
                    expr_list.push(Expr::BinaryOperation{
                        l_expr: Box::new(e1),
                        op: op_list.pop().unwrap().0,
                        r_expr: Box::new(e2)
                    });

                    self.lexer.next_token();
                    expr_list.push(try!(self.parse_term()));
                    op_list.push((op2, prec2));

                    continue;
                }

                self.lexer.next_token();
                expr_list.push(try!(self.parse_term()));
                op_list.push((op2, prec2));
            }
            let bop = Expr::BinaryOperation{
                l_expr: Box::new(expr_list.pop().unwrap()),
                op: op_list.pop().unwrap().0,
                r_expr: Box::new(expr_list.pop().unwrap())
            };
            expr_list.push(bop);
        }
        return Ok(expr_list.pop().unwrap());
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        let e1 = try!(self.parse_term());
        self.lexer.next_token();
        if self.lexer.tokens_remaining() > 0 &&
           self.lexer.current_is_type(TokenType::BinOp) {
            return self.parse_binop(e1);
        }
        if self.lexer.tokens_remaining() > 0 {
            self.lexer.prev_token();
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
                try!(self.lexer.match_token(TokenType::Equals));
                self.lexer.next_token();
                let e = try!(self.parse_expression());
                return Ok(Statement::Assignment{name: name, expr: e});
            },
            TokenType::Identifier => {
                let name = self.lexer.curr_value();
                self.lexer.next_token();
                if !self.lexer.current_is_type(TokenType::Equals) {
                    // Whoops, this isn't an assignment
                    self.lexer.prev_token();
                    let e = try!(self.parse_expression());
                    return Ok(Statement::Expression(e));
                }
                self.lexer.next_token();
                let e = try!(self.parse_expression());
                return Ok(Statement::Assignment{name: name, expr: e});
            },
            TokenType::If => {
                self.lexer.next_token();
                try!(self.lexer.match_token(TokenType::LPar));
                self.lexer.next_token();
                let cond = try!(self.parse_expression());
                self.lexer.next_token();
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
            TokenType::While => {
                self.lexer.next_token();
                try!(self.lexer.match_token(TokenType::LPar));
                self.lexer.next_token();
                let cond = try!(self.parse_expression());
                self.lexer.next_token();
                try!(self.lexer.match_token(TokenType::RPar));
                self.lexer.next_token();
                let body = try!(self.parse_block());
                return Ok(Statement::While{cond: cond, body: body});
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
        let mut program = Vec::new();
        if self.lexer.tokens().len() > 0 {
            loop {
                if self.lexer.tokens_remaining() < 1 {
                    break;
                }
                program.push(try!(self.parse_statement()));
                self.lexer.next_token();
                if self.lexer.current_is_type(TokenType::Semicolon) {
                    self.lexer.next_token();
                }
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
