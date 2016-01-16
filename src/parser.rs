use lexer::*;
use value::*;

/**
 *
 * program ->
 *  [
 *      Statement ->
 *          Expression
 *              Expression ->
 *                  Atom |
 *                  BinOp
 *                      BinOp ->
 *                          Atom BinOp Expression
 *  ]
 *
 **/

#[derive(Debug)]
pub struct BinaryOp {
    pub l_expr: Expr,
    pub op: BinOp,
    pub r_expr: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Atom(Value),
    BinaryOperation(Box<BinaryOp>),
    GetName(String),
    Nil,
}

#[derive(Debug)]
pub struct DefLet {
    pub name: String,
    pub expr: Expr
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expr),
    Assignment(DefLet),
    Nil,
}

pub struct Parser {
    lexer:  Lexer
}

impl Parser {
    pub fn new() -> Parser {
        return Parser{lexer: Lexer::new()};
    }

    fn parse_term(&mut self) -> Expr {
        if self.lexer.current_is_type(TokenType::Int) {
            let int = self.lexer.curr_value().parse::<i32>().unwrap();
            return Expr::Atom(Value::Int(int));
        }
        if self.lexer.current_is_type(TokenType::Float) {
            let float = self.lexer.curr_value().parse::<f32>().unwrap();
            return Expr::Atom(Value::Float(float));
        }
        if self.lexer.current_is_type(TokenType::Identifier) {
            let e1 = Expr::GetName(self.lexer.curr_value());
            if self.lexer.next_token() &&
               self.lexer.current_is_type(TokenType::BinOp) {
                return self.parse_binop(e1);
            }
            return e1;
        }
        if self.lexer.current_is_type(TokenType::LPar) {
            self.lexer.next_token();
            let e = self.parse_expression();
            self.lexer.match_token(TokenType::RPar).unwrap();
            return e;
        }
        return Expr::Nil;
    }

    // TODO: precedences ~!!@
    fn parse_binop(&mut self, e1: Expr) -> Expr {
        let (op, _) = Lexer::bin_op(&self.lexer.curr_value()[..]).unwrap();
        self.lexer.next_token();
        let bop = BinaryOp{l_expr: e1, op: op, r_expr: self.parse_expression()};
        return Expr::BinaryOperation(Box::new(bop));
    }

    // Wrap in a while loop
    fn parse_expression(&mut self) -> Expr {
        let e1 = self.parse_term();
        if self.lexer.next_token() &&
           self.lexer.current_is_type(TokenType::BinOp) {
            return self.parse_binop(e1);
        }
        return e1;
    }

    fn parse_statement(&mut self) -> Statement {
        if self.lexer.tokens().len() > 0 {
            match *self.lexer.curr_type() {
                TokenType::Let => {
                   self.lexer.next_token();
                   self.lexer.match_token(TokenType::Identifier).unwrap();
                   let name = self.lexer.curr_value();
                   self.lexer.next_token();
                   self.lexer.match_token(TokenType::Equals).unwrap();
                   self.lexer.next_token();
                   let e = self.parse_expression();
                   return Statement::Assignment(DefLet{name: name, expr: e});

                },
                _ => return Statement::Expression(self.parse_expression()),
            }
        }
        return Statement::Nil;
    }

    fn parse_program(&mut self) -> Vec<Statement> {
        println!("Lexer: {:?}", self.lexer.tokens());
        let mut program = Vec::new();
        loop {
            program.push(self.parse_statement());
            if !self.lexer.next_token() {
               return program;
            }
        }
    }

    pub fn parse_lines(&mut self, text: String) -> Result<Vec<Statement>, String> {
        match self.lexer.tokenize(text.clone()) {
            Err(msg) => return Err(msg),
            _ => ()
        }
        let program = self.parse_program();
        self.lexer.reset();
        return Ok(program);
    }
}
