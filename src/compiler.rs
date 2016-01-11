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

fn compile_expression(script: &mut Script, expr: &Expression) {
    match expr {
        &Expression::Atom(v) => script.stack.push(v),
        &Expression::BinaryOperation(ref bop) => {
            compile_expression(script, &bop.r_expr);
            compile_expression(script, &bop.l_expr);
            match bop.op {
                BinOp::PLUS => script.program.push(OpCode::ADD),
                BinOp::MIN  => script.program.push(OpCode::SUB),
                BinOp::MUL  => script.program.push(OpCode::MUL),
                BinOp::DIV  => script.program.push(OpCode::DIV),
            }
        },
        &Expression::Nil => (),
    }
}

pub fn compile_script(expressions: Vec<Expression>) -> Script {
    let mut script = Script::new();
    for expr in expressions {
        compile_expression(&mut script, &expr);
    }
    return script;
}
