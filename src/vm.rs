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
    // TODO: Make real scopes instead of this travesty
    scopes:     HashMap<String, Value>
}

impl VM {
    pub fn new() -> VM {
        return VM{
            program: Vec::new(),
            stack: Vec::new(),
            ip: 0,
            running: false,
            scopes: HashMap::new()
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

    pub fn run(&mut self) {
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
                            self.scopes.insert(s, self.stack.pop().unwrap());
                        },
                        _ => self.stack.push(Value::Error("invalid assignment")),
                    }
                },
                OpCode::GetName(ref n)  => self.stack.push(self.scopes[n].clone()),
                OpCode::Call => {
                    let mut args_len = 0;
                    match self.stack.pop().unwrap() {
                        Value::Int(i) => args_len = i,
                        _ => ()
                    }
                    for _ in 0 .. args_len {
                        self.stack.pop();
                    }
                    match self.stack.pop().unwrap() {
                        Value::Object(o) => {
                            match o {
                                Object::Function{args:_, body} => {
                                    let mut frame = VM::new();
                                    frame.load(body);
                                    frame.run();
                                    self.stack.push(frame.stack()[0].clone());
                                }
                            }
                        },
                        _ => () // TODO: Error, should be using traits here I think
                    }
                },
            }
            self.ip += 1;
        }
        self.running = false;
    }
}
