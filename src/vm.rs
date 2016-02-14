use std::collections::HashMap;
use object::{Object, Native};
use opcode::OpCode;
use value::Value;

pub struct VM {
    program:    Vec<OpCode>,
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
    pub fn new(program: Vec<OpCode>) -> VM {
        return VM{
            program: program,
            stack: Vec::new(),
            ip: 0,
            running: false,
        };
    }

    pub fn run(&mut self, scopes: &mut HashMap<String, Value>) -> Result<Option<Value>, String> {
        self.running = true;
        while self.running && self.ip < self.program.len() {
            match self.program[self.ip] {
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
                    match self.stack.pop().unwrap() {
                        Value::Str(s) => {
                            scopes.insert(s, self.stack.pop().unwrap());
                        },
                        _ => return Err("invalid assignment".to_string()),
                    }
                },
                OpCode::GetName(ref n)  => {
                    match scopes.get(n) {
                        Some(v) => self.stack.push(v.clone()),
                        None => ()
                    }
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
                    let mut args_len =
                        match self.stack.pop().unwrap() {
                            Value::Number(n) => n as usize,
                            _ => panic!("bad bytecode")
                        };
                    let mut arg_values = Vec::new();
                    for _ in 0 .. args_len {
                        arg_values.push(self.stack.pop().unwrap());
                    }
                    match self.stack.pop().unwrap() {
                        Value::Object(o) => {
                            match o {
                                Object::Function{args, body} => {
                                    let mut frame = VM::new(body);
                                    for arg in args {
                                        if args_len > 0 {
                                            scopes.insert(arg.clone(), arg_values.pop().unwrap());
                                            args_len -= 1;
                                        } else {
                                            scopes.insert(arg.clone(), Value::Undefined);
                                        }
                                    }
                                    match try!(frame.run(scopes)) {
                                        Some(result) => self.stack.push(result),
                                        None => ()
                                    }
                                },
                                Object::Native(nobj) => {
                                   match nobj {
                                      Native::Function(f) => {
                                          // we won't be popping off the vector
                                          // so let's flip it as a convenience
                                          // (values will be in the true arg order)
                                          arg_values.reverse();
                                          self.stack.push(f(arg_values));
                                      }
                                   }
                                }
                            }
                        },
                        _ => return Err("invalid call".to_string()),
                    }
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
