use lexer::*;
use value::*;

/**
 *
 * program ->
 *  [
 *      Expression ->
 *          Atom |
 *          BinOp
 *              BinOp ->
 *                  Expression BinOp Expression
 *  ]
 *
 **/

#[derive(Debug)]
pub struct BinaryOp {
    pub l_expr: Expression,
    pub op: BinOp,
    pub r_expr: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Atom(Value),
    BinaryOperation(Box<BinaryOp>),
    Nil,
}

pub struct Parser {
    lexer:  Lexer
}

impl Parser {
    pub fn new() -> Parser {
        return Parser{lexer: Lexer::new()};
    }

    fn parse_atom(&mut self) -> Expression {
        if self.lexer.current_is_type(TokenType::INT) {
            let int = self.lexer.curr_value().parse::<i32>().unwrap();
            return Expression::Atom(Value::Int(int));
        }
        if self.lexer.current_is_type(TokenType::FLOAT) {
            let float = self.lexer.curr_value().parse::<f32>().unwrap();
            return Expression::Atom(Value::Float(float));
        }
        return Expression::Nil;
    }

    fn parse_expression(&mut self) -> Expression {
        if self.lexer.tokens().len() > 0 {
            let e1 = self.parse_atom();
            if self.lexer.next_token() &&
               self.lexer.current_is_type(TokenType::BINOP) {
                let op = Lexer::bin_op(&self.lexer.curr_value()[..]).unwrap();
                self.lexer.next_token();
                let bop = BinaryOp{l_expr: e1, op: op, r_expr: self.parse_expression()};
                return Expression::BinaryOperation(Box::new(bop));
            }
            return e1;
        }
        return Expression::Nil;
    }

    fn parse_program(&mut self) -> Vec<Expression> {
        println!("Lexer: {:?}", self.lexer.tokens());
        let mut program = Vec::new();
        program.push(self.parse_expression());
        return program;
    }

    pub fn parse_lines(&mut self, text: String) -> Vec<Expression> {
        self.lexer.tokenize(text.clone());
        let program = self.parse_program();
        self.lexer.reset();
        return program;
    }
}
