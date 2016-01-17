#![allow(dead_code)]

use std::collections::HashMap;
use opcode::OpCode;
use value::Value;

// Everything you need to run some code in the vm
pub struct Script {
    pub program: Vec<OpCode>,
}

impl Script {
    pub fn new() -> Script {
        return Script{program: Vec::new()};
    }
}

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

    pub fn load(&mut self, script: Script) {
        self.program = script.program;
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
                OpCode::Call => (),
            }
            self.ip += 1;
        }
        self.running = false;
    }
}
