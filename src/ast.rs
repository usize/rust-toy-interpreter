use lexer::BinOp;
use value::Value;

#[derive(Debug, Clone)]
pub enum Expression {
    Atom(Value),
    BinaryOperation{l_expr: Box<Expression>, op: BinOp, r_expr: Box<Expression>},
    GetName(String),
    Function{name: Option<String>, args: Vec<String>, body: Vec<Statement>},
    Call(Vec<Expression>),
    Return(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expression),
    Assignment{name: String, expr: Expression},
    If{cond: Expression, body: Vec<Statement>},
    IfElse{cond: Expression, body: Vec<Statement>, else_body: Vec<Statement>},
    While{cond: Expression, body: Vec<Statement>}
}

