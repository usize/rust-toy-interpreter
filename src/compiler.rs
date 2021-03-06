use lexer::BinOp;
use ast::{Statement, Expression};
use opcode::OpCode;
use value::Value;
use object::Object;

fn compile_expression(script: &mut Vec<OpCode>, expr: &Expression) {
    match *expr {
        Expression::Atom(ref v) => script.push(OpCode::Val(v.clone())),
        Expression::BinaryOperation{ref l_expr, ref op, ref r_expr} => {
            compile_expression(script, r_expr);
            compile_expression(script, l_expr);
            match *op {
                BinOp::Plus     => script.push(OpCode::Add),
                BinOp::Min      => script.push(OpCode::Sub),
                BinOp::Mul      => script.push(OpCode::Mul),
                BinOp::Div      => script.push(OpCode::Div),
                BinOp::EqEq     => script.push(OpCode::EqEq),
                BinOp::NotEq    => script.push(OpCode::NotEq),
                BinOp::Lt       => script.push(OpCode::Lt),
                BinOp::LtEq     => script.push(OpCode::LtEq),
                BinOp::Gt       => script.push(OpCode::Gt),
                BinOp::GtEq     => script.push(OpCode::GtEq),
            }
        },
        Expression::GetName(ref n) => {
            script.push(OpCode::GetName(n.clone()))
        },
        Expression::Function{ref name, ref args, ref body} => {
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
        Expression::Call(ref args) => {
            for e in args {
                compile_expression(script, e);
            }
            script.push(OpCode::Val(Value::Number((args.len() as f64) - 1.0)));
            script.push(OpCode::Call);
        },
        Expression::Return(ref e) => {
           compile_expression(script, e);
           script.push(OpCode::Ret);
        },
    }
}

fn compile_assignment(script: &mut Vec<OpCode>, name: &String, expr: &Expression) {
    compile_expression(script, expr);
    script.push(OpCode::Val(Value::Str(name.clone())));
    script.push(OpCode::Def);
}

pub fn compile_script(statements: Vec<Statement>) -> Vec<OpCode> {
    let mut script = Vec::new();
    for statement in statements {
        match statement {
            Statement::Expr(s) => compile_expression(&mut script, &s),
            Statement::Assignment{ref name, ref expr} => {
                compile_assignment(&mut script, name, expr);
            },
            Statement::If{cond, body} => {
                compile_expression(&mut script, &cond);
                let body = compile_script(body);
                script.push(OpCode::JumpIfNot(body.len() as i32 + 1));
                script.extend(body.iter().cloned());
            },
            Statement::IfElse{cond, body, else_body} => {
                compile_expression(&mut script, &cond);
                let body = compile_script(body);
                script.push(OpCode::JumpIfNot(body.len() as i32 + 1));
                script.extend(body.iter().cloned());
                compile_expression(&mut script, &cond);
                let else_body = compile_script(else_body);
                script.push(OpCode::JumpIf(else_body.len() as i32 + 1));
                script.extend(else_body.iter().cloned());
            },
            Statement::While{cond, body} => {
                let start_len = script.len() as i32;
                compile_expression(&mut script, &cond);
                let body = compile_script(body);
                script.push(OpCode::JumpIfNot(body.len() as i32 + 2));
                script.extend(body.iter().cloned());
                let end_len = script.len() as i32;
                script.push(OpCode::Jump(start_len - end_len));
            },
        }
    }
    return script;
}
