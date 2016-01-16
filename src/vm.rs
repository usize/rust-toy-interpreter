#![allow(dead_code)]

use std::collections::HashMap;
use value::*;

#[derive(Debug)]
pub enum OpCode {
    Val(Value), // stack.push(Value)
    Add, // stack.pop() + stack.pop()
    Sub, // stack.pop() - stack.pop()
    Mul, // stack.pop() * stack.pop()
    Div, // stack.pop() / stack.pop()
    Def, // scopes[stack.pop()] = stack.pop()
    GetName(usize), // scopes[stack.pop()] = stack.pop()
}

pub struct VM {
    program:    Vec<OpCode>,
    strings:    Vec<String>,
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
            strings: Vec::new(),
            stack: Vec::new(),
            ip: 0,
            running: false,
            scopes: HashMap::new()
        };
    }

    pub fn load(&mut self, program: Vec<OpCode>, strings: Vec<String>) {
        self.program = program;
        self.strings = strings;
        self.stack = Vec::new();
        self.ip = 0;
    }

    fn add(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a.add(b));
    }

    fn sub(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a.sub(b));
    }

    fn mul(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a.mul(b));
    }

    fn div(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a.div(b));
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
                OpCode::Val(v)      => self.stack.push(v),
                OpCode::Add         => self.add(),
                OpCode::Sub         => self.sub(),
                OpCode::Mul         => self.mul(),
                OpCode::Div         => self.div(),
                OpCode::Def         => {
                    match self.stack.pop().unwrap() {
                        Value::Str(n) => {
                            self.scopes.insert(self.strings[n].clone(), self.stack.pop().unwrap());
                        },
                        _ => self.stack.push(Value::Error("invalid assignment")),
                    }
                },
                OpCode::GetName(n)  => self.stack.push(self.scopes[&self.strings[n]].clone()),
            }
            self.ip += 1;
        }
        self.running = false;
    }
}
