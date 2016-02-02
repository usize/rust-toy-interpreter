use std::collections::HashMap;
use opcode::OpCode;
use value::Value;

pub struct VM {
    stack:      Vec<Value>,
    ip:         usize,
    running:    bool,
}

macro_rules! binary_op {
    ($vm_ref:expr, $a:ident, $b:ident, $r:expr) => {
        {
            let $b = $vm_ref.stack.pop().unwrap();
            let $a = $vm_ref.stack.pop().unwrap();
            $vm_ref.stack.push($r);
        }
    }
}

impl VM {
    pub fn new() -> VM {
        return VM{
            stack: Vec::new(),
            ip: 0,
            running: false,
        };
    }

    pub fn run(&mut self, program: &Vec<OpCode>) -> Result<Option<Value>, String> {
        self.running = true;
        while self.running && self.ip < program.len() {
            match program[self.ip] {
                OpCode::Val(ref v)  => self.stack.push(v.clone()),
                OpCode::Add         => binary_op!(self, a, b, try!(a.add(b))),
                OpCode::Sub         => binary_op!(self, a, b, try!(a.sub(b))),
                OpCode::Mul         => binary_op!(self, a, b, try!(a.mul(b))),
                OpCode::Div         => binary_op!(self, a, b, try!(a.div(b))),
                OpCode::EqEq        => binary_op!(self, a, b, Value::Bool(a == b)),
                OpCode::NotEq       => binary_op!(self, a, b, Value::Bool(a != b)),
                OpCode::Lt          => binary_op!(self, a, b, Value::Bool(a < b)),
                OpCode::LtEq        => binary_op!(self, a, b, Value::Bool(a <= b)),
                OpCode::Gt          => binary_op!(self, a, b, Value::Bool(a > b)),
                OpCode::GtEq        => binary_op!(self, a, b, Value::Bool(a >= b)),
                OpCode::Def         => {
                },
                OpCode::GetName(ref n)  => {
                },
                OpCode::JumpIfNot(ref n) => {
                    if !self.stack.pop().unwrap().to_boolean() {
                        self.ip = (self.ip as i32 + *n) as usize;
                        continue;
                    }
                },
                OpCode::JumpIf(ref n) => {
                    if self.stack.pop().unwrap().to_boolean() {
                        self.ip = (self.ip as i32 + *n) as usize;
                        continue;
                    }
                },
                OpCode::Jump(ref n) => {
                    self.ip = (self.ip as i32 + *n) as usize;
                    continue;
                },
                OpCode::Ret => {
                    self.running = false;
                },
                OpCode::Call => {
                },
            }
            self.ip += 1;
        }
        self.running = false;
        match self.stack.last() {
            Some(result) => return Ok(Some(result.clone())),
            None => return Ok(None)
        }
    }
}
