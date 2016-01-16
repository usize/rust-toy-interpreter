use vm::*;
use lexer::*;
use parser::*;
use value::*;

// Everything you need to run some code in the vm
pub struct Script {
    pub program: Vec<OpCode>,
    pub strings: Vec<String>
}

impl Script {
    pub fn new() -> Script {
        return Script{program: Vec::new(), strings: Vec::new()};
    }
}

fn compile_expression(script: &mut Script, expr: &Expr) {
    match expr {
        &Expr::Atom(v) => script.program.push(OpCode::VAL(v)),
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
        &Expr::GetName(ref n) => {
            script.strings.push(n.clone());
            script.program.push(OpCode::GETNAME(script.strings.len() - 1))
        },
        &Expr::Nil => (),
    }
}

fn compile_assignment(script: &mut Script, deflet: &DefLet) {
    compile_expression(script, &deflet.expr);
    script.strings.push(deflet.name.clone());
    script.program.push(OpCode::VAL(Value::Str(script.strings.len() - 1)));
    script.program.push(OpCode::DEF);
}

pub fn compile_script(statements: Vec<Statement>) -> Script {
    let mut script = Script::new();
    for statement in statements {
        match statement {
            Statement::Expression(s) => compile_expression(&mut script, &s),
            Statement::Assignment(a) => compile_assignment(&mut script, &a),
        }
    }
    return script;
}
