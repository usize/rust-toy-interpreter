use lexer::BinOp;
use value::Value;

#[derive(Debug, Clone)]
pub enum Expr {
    Atom(Value),
    BinaryOperation{l_expr: Box<Expr>, op: BinOp, r_expr: Box<Expr>},
    GetName(String),
    Function{name: Option<String>, args: Vec<String>, body: Vec<Statement>},
    Call(Vec<Expr>),
    Return(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expr),
    Assignment{name: String, expr: Expr},
    If{cond: Expr, body: Vec<Statement>},
    IfElse{cond: Expr, body: Vec<Statement>, else_body: Vec<Statement>},
    While{cond: Expr, body: Vec<Statement>}
}

