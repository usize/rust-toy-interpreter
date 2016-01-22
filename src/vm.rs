#![allow(dead_code)]

use std::collections::HashMap;
use object::Object;
use opcode::OpCode;
use value::Value;

pub struct VM {
    program:    Vec<OpCode>,
    stack:      Vec<Value>,
    ip:         usize,
    running:    bool,
}

impl VM {
    pub fn new() -> VM {
        return VM{
            program: Vec::new(),
            stack: Vec::new(),
            ip: 0,
            running: false,
        };
    }

    pub fn load(&mut self, program: Vec<OpCode>) {
        self.program = program;
        self.stack = Vec::new();
        self.ip = 0;
    }

    pub fn stack(&self) -> &Vec<Value> {
        return &self.stack;
    }

    pub fn program(&self) -> &Vec<OpCode> {
        return &self.program;
    }

    pub fn run(&mut self, scopes: &mut HashMap<String, Value>) -> Result<Option<Value>, String> {
        self.running = true;
        while self.running && self.ip < self.program.len() {
            match self.program[self.ip] {
                OpCode::Val(ref v)  => self.stack.push(v.clone()),
                OpCode::Add         => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try!(a.add(b)));
                },
                OpCode::Sub         => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(try!(a.sub(b)));
                },
                OpCode::Mul         => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a.mul(b));
                },
                OpCode::Div         => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a.div(b));
                },
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
                    match self.stack.pop().unwrap().as_bool() {
                        Value::Bool(v) => {
                            if !v {
                                self.ip = (self.ip as i32 + *n) as usize;
                                continue;
                            }
                        },
                        _ => ()
                    }
                },
                OpCode::JumpIf(ref n) => {
                    match self.stack.pop().unwrap().as_bool() {
                        Value::Bool(v) => {
                            if v {
                                self.ip = (self.ip as i32  + *n) as usize;
                                continue;
                            }
                        },
                        _ => ()
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
                                    let mut frame = VM::new();
                                    for arg in args {
                                        if args_len > 0 {
                                            scopes.insert(arg.clone(), arg_values.pop().unwrap());
                                            args_len -= 1;
                                        } else {
                                            scopes.insert(arg.clone(), Value::Undefined);
                                        }
                                    }
                                    frame.load(body);
                                    match try!(frame.run(scopes)) {
                                        Some(result) => self.stack.push(result),
                                        None => ()
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
        match self.stack.get(0) {
            Some(result) => return Ok(Some(result.clone())),
            None => return Ok(None)
        }
    }
}
