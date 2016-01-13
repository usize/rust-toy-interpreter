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
    Nil,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expr),
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

    fn parse_expression(&mut self) -> Expr {
        if self.lexer.tokens().len() > 0 {
            let e1 = self.parse_atom();
            if self.lexer.next_token() &&
               self.lexer.current_is_type(TokenType::BINOP) {
                let op = Lexer::bin_op(&self.lexer.curr_value()[..]).unwrap();
                self.lexer.next_token();
                let bop = BinaryOp{l_expr: e1, op: op, r_expr: self.parse_expression()};
                return Expr::BinaryOperation(Box::new(bop));
            }
            return e1;
        }
        return Expr::Nil;
    }

    fn parse_statement(&mut self) -> Statement {
        return Statement::Expression(self.parse_expression());
    }

    fn parse_program(&mut self) -> Vec<Statement> {
        println!("Lexer: {:?}", self.lexer.tokens());
        let mut program = Vec::new();
        program.push(self.parse_statement());
        return program;
    }

    pub fn parse_lines(&mut self, text: String) -> Vec<Statement> {
        self.lexer.tokenize(text.clone());
        let program = self.parse_program();
        self.lexer.reset();
        return program;
    }
}
