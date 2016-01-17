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
    Function(Vec<String>, Vec<Statement>),
    Nil,
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

    fn parse_term(&mut self) -> Expr {
        match *self.lexer.curr_type() {
            TokenType::Int => {
                let int = self.lexer.curr_value().parse::<i32>().unwrap();
                return Expr::Atom(Value::Int(int));
            },
            TokenType::Float => {
                let float = self.lexer.curr_value().parse::<f32>().unwrap();
                return Expr::Atom(Value::Float(float));
            },
            TokenType::Str => {
                return Expr::Atom(Value::Str(self.lexer.curr_value()));
            },
            TokenType::Identifier => {
                let e1 = Expr::GetName(self.lexer.curr_value());
                if self.lexer.next_token() &&
                   self.lexer.current_is_type(TokenType::BinOp) {
                    return self.parse_binop(e1);
                }
                return e1;
            },
            TokenType::LPar => {
                self.lexer.next_token();
                let e = self.parse_expression();
                self.lexer.match_token(TokenType::RPar).unwrap();
                return e;
            },
            TokenType::Function => {
                self.lexer.next_token();
                self.lexer.match_token(TokenType::LPar).unwrap();
                self.lexer.next_token();
                let mut args = Vec::new();
                while self.lexer.current_is_type(TokenType::Identifier) {
                    args.push(self.lexer.curr_value());
                     self.lexer.next_token();
                     if self.lexer.current_is_type(TokenType::Comma) {
                         self.lexer.next_token();
                     }
                }
                self.lexer.match_token(TokenType::RPar).unwrap();
                self.lexer.next_token();
                let body = self.parse_block();
                self.lexer.next_token();
                return Expr::Function(args, body);
            },
            _ => Expr::Nil
        }
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
        if self.lexer.tokens_remaining() > 0 &&
           self.lexer.current_is_type(TokenType::BinOp) {
            return self.parse_binop(e1);
        }
        return e1;
    }

    fn parse_statement(&mut self) -> Statement {
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

    fn parse_block(&mut self) -> Vec<Statement> {
        self.lexer.match_token(TokenType::LCBrace).unwrap();
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
        self.lexer.match_token(TokenType::RCBrace).unwrap();
        return Parser::parse_from_tokens(block_tokens);
    }

    fn parse_from_tokens(tokens: Vec<Token>) -> Vec<Statement> {
        let mut block_parser = Parser::from(tokens);
        return block_parser.parse_program();
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        println!("Lexer: {:?}", self.lexer.tokens());
        let mut program = Vec::new();
        if self.lexer.tokens().len() > 0 {
            loop {
                program.push(self.parse_statement());
                if self.lexer.tokens_remaining() < 1 {
                   return program;
                }
                self.lexer.next_token();
            }
        }
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
