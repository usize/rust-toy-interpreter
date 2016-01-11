use vm::*;
use value::*;
use lexer::*;
use parser::*;

// Everything you need to run some code in the vm
pub struct Script {
    pub program: Vec<OpCode>,
    pub stack:   Vec<Value>
}

impl Script {
    pub fn new() -> Script {
        return Script{program: Vec::new(), stack: Vec::new()};
    }
}

fn compile_expression(script: &mut Script, expr: &Expr) {
    match expr {
        &Expr::Atom(v) => script.stack.push(v),
        &Expr::BinaryOperation(ref bop) => {
            compile_expression(script, &bop.r_expr);
            compile_expression(script, &bop.l_expr);
            match bop.op {
                BinOp::PLUS => script.program.push(OpCode::ADD),
                BinOp::MIN  => script.program.push(OpCode::SUB),
                BinOp::MUL  => script.program.push(OpCode::MUL),
                BinOp::DIV  => script.program.push(OpCode::DIV),
            }
        },
        &Expr::Nil => (),
    }
}

pub fn compile_script(statements: Vec<Statement>) -> Script {
    let mut script = Script::new();
    for statement in statements {
        match statement {
            Statement::Expression(s) => compile_expression(&mut script, &s),
            Statement::Nil => (),
        }
    }
    return script;
}
