use vm::*;
use lexer::*;
use parser::*;
use value::*;

fn compile_expression(script: &mut Script, expr: &Expr) {
    match expr {
        &Expr::Atom(v) => script.program.push(OpCode::Val(v)),
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
            script.strings.push(n.clone());
            script.program.push(OpCode::GetName(script.strings.len() - 1))
        },
        &Expr::Nil => (),
    }
}

fn compile_assignment(script: &mut Script, deflet: &DefLet) {
    compile_expression(script, &deflet.expr);
    script.strings.push(deflet.name.clone());
    script.program.push(OpCode::Val(Value::Str(script.strings.len() - 1)));
    script.program.push(OpCode::Def);
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
