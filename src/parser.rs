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
    Assignment(DefLet)
}

pub struct Parser {
    lexer:  Lexer
}

impl Parser {
    pub fn new() -> Parser {
        return Parser{lexer: Lexer::new()};
    }

    fn parse_atom(&mut self) -> Expr {
        if self.lexer.current_is_type(TokenType::INT) {
            let int = self.lexer.curr_value().parse::<i32>().unwrap();
            return Expr::Atom(Value::Int(int));
        }
        if self.lexer.current_is_type(TokenType::FLOAT) {
            let float = self.lexer.curr_value().parse::<f32>().unwrap();
            return Expr::Atom(Value::Float(float));
        }
        return Expr::Nil;
    }

    // TODO: fix broken binops *should be expr op expr, not atom op expr
    fn parse_binop(&mut self, e1: Expr) -> Expr {
        let op = Lexer::bin_op(&self.lexer.curr_value()[..]).unwrap();
        self.lexer.next_token();
        let bop = BinaryOp{l_expr: e1, op: op, r_expr: self.parse_expression()};
        return Expr::BinaryOperation(Box::new(bop));
    }

    fn parse_expression(&mut self) -> Expr {
        match *self.lexer.curr_type() {
            TokenType::INT | TokenType::FLOAT => {
                let e1 = self.parse_atom();
                if self.lexer.next_token() &&
                   self.lexer.current_is_type(TokenType::BINOP) {
                    return self.parse_binop(e1);
                }
                return e1;
            },
            TokenType::IDENTIFIER => {
                let e1 = Expr::GetName(self.lexer.curr_value());
                if self.lexer.next_token() &&
                   self.lexer.current_is_type(TokenType::BINOP) {
                    return self.parse_binop(e1);
                }
                return e1;
            },
            _ => return Expr::Nil
        }
    }

    fn parse_statement(&mut self) -> Statement {
        if self.lexer.tokens().len() > 0 {
            match *self.lexer.curr_type() {
                TokenType::LET => {
                   self.lexer.next_token();
                   self.lexer.match_token(TokenType::IDENTIFIER).unwrap();
                   let name = self.lexer.curr_value();
                   self.lexer.next_token();
                   self.lexer.match_token(TokenType::EQUALS).unwrap();
                   self.lexer.next_token();
                   let e = self.parse_expression();
                   return Statement::Assignment(DefLet{name: name, expr: e});

                },
                _ => return Statement::Expression(self.parse_expression()),
            }
        }
        return Statement::Expression(Expr::Nil);
    }

    fn parse_program(&mut self) -> Vec<Statement> {
        println!("Lexer: {:?}", self.lexer.tokens());
        let mut program = Vec::new();
        program.push(self.parse_statement());
        return program;
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
