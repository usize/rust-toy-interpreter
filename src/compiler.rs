use lexer::BinOp;
use parser::{Statement, DefLet, Expr};
use opcode::OpCode;
use value::Value;
use object::Object;

fn compile_expression(script: &mut Vec<OpCode>, expr: &Expr) {
    match *expr {
        Expr::Atom(ref v) => script.push(OpCode::Val(v.clone())),
        Expr::BinaryOperation(ref bop) => {
            compile_expression(script, &bop.r_expr);
            compile_expression(script, &bop.l_expr);
            match bop.op {
                BinOp::Plus => script.push(OpCode::Add),
                BinOp::Min  => script.push(OpCode::Sub),
                BinOp::Mul  => script.push(OpCode::Mul),
                BinOp::Div  => script.push(OpCode::Div),
            }
        },
        Expr::GetName(ref n) => {
            script.push(OpCode::GetName(n.clone()))
        },
        Expr::Function{ref name, ref args, ref body} => {
            let s = compile_script(body.clone());
            let o = Object::Function{args: args.clone(), body: s};
            script.push(OpCode::Val(Value::Object(o)));
            match *name {
                Some(ref v) => {
                    script.push(OpCode::Val(Value::Str(v.clone())));
                    script.push(OpCode::Def);
                },
                None => ()
            }
        },
        Expr::Call(ref args) => {
            for e in args {
                compile_expression(script, e);
            }
            script.push(OpCode::Val(Value::Int((args.len() as i32) - 1)));
            script.push(OpCode::Call);
        },
        Expr::Return(ref e) => {
           compile_expression(script, e);
           script.push(OpCode::Ret);
        },
    }
}

fn compile_assignment(script: &mut Vec<OpCode>, deflet: &DefLet) {
    compile_expression(script, &deflet.expr);
    script.push(OpCode::Val(Value::Str(deflet.name.clone())));
    script.push(OpCode::Def);
}

pub fn compile_script(statements: Vec<Statement>) -> Vec<OpCode> {
    let mut script = Vec::new();
    for statement in statements {
        match statement {
            Statement::Expression(s) => compile_expression(&mut script, &s),
            Statement::Assignment(a) => compile_assignment(&mut script, &a),
        }
    }
    return script;
}
