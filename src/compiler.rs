use vm::Script;
use lexer::BinOp;
use parser::{Statement, DefLet, Expr};
use opcode::OpCode;
use value::Value;

fn compile_expression(script: &mut Script, expr: &Expr) {
    match expr {
        &Expr::Atom(ref v) => script.program.push(OpCode::Val(v.clone())),
        &Expr::BinaryOperation(ref bop) => {
            compile_expression(script, &bop.r_expr);
            compile_expression(script, &bop.l_expr);
            match bop.op {
                BinOp::Plus => script.program.push(OpCode::Add),
                BinOp::Min  => script.program.push(OpCode::Sub),
                BinOp::Mul  => script.program.push(OpCode::Mul),
                BinOp::Div  => script.program.push(OpCode::Div),
            }
        },
        &Expr::GetName(ref n) => {
            script.program.push(OpCode::GetName(n.clone()))
        },
        &Expr::Nil => ()
    }
}

fn compile_assignment(script: &mut Script, deflet: &DefLet) {
    compile_expression(script, &deflet.expr);
    script.program.push(OpCode::Val(Value::Str(deflet.name.clone())));
    script.program.push(OpCode::Def);
}

pub fn compile_script(statements: Vec<Statement>) -> Script {
    let mut script = Script::new();
    for statement in statements {
        match statement {
            Statement::Expression(s) => compile_expression(&mut script, &s),
            Statement::Assignment(a) => compile_assignment(&mut script, &a),
            Statement::Function(args, body)   => (),
        }
    }
    return script;
}
