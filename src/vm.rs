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

    pub fn run(&mut self, scopes: &mut HashMap<String, Value>) -> Result<Option<Value>, ()> {
        self.running = true;
        while self.running && self.ip < self.program.len() {
            match self.program[self.ip] {
                OpCode::Val(ref v)  => self.stack.push(v.clone()),
                OpCode::Add         => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a.add(b));
                },
                OpCode::Sub         => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a.sub(b));
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
                        _ => self.stack.push(Value::Error("invalid assignment")),
                    }
                },
                OpCode::GetName(ref n)  => {
                    match scopes.get(n) {
                        Some(v) => self.stack.push(v.clone()),
                        None => ()
                    }
                },
                OpCode::Ret => {
                    self.running = false;
                },
                OpCode::Call => {
                    let mut args_len = 0;
                    match self.stack.pop().unwrap() {
                        Value::Int(i) => args_len = i,
                        _ => ()
                    }
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
                        _ => self.stack.push(Value::Error("invalid call")),
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
