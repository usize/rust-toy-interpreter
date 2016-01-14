#![allow(dead_code)]

use std::collections::HashMap;
use value::*;

#[derive(Debug)]
pub enum OpCode {
    VAL(Value), // stack.push(Value)
    ADD, // stack.pop() + stack.pop()
    SUB, // stack.pop() - stack.pop()
    MUL, // stack.pop() * stack.pop()
    DIV, // stack.pop() / stack.pop()
    DEF, // scopes[stack.pop()] = stack.pop()
    GETNAME(&'static str), // scopes[stack.pop()] = stack.pop()
}

pub struct VM {
    program:    Vec<OpCode>,
    //callstack:  Vec<usize>,
    stack:      Vec<Value>,
    ip:         usize,
    running:    bool,
    // TODO: Make real scopes instead of this travesty
    scopes:     HashMap<&'static str, Value>
}

impl VM {
    pub fn new() -> VM {
        return VM{
            program: Vec::new(),
            /*callstack: Vec::new(),*/
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
                OpCode::VAL(v)      => self.stack.push(v),
                OpCode::ADD         => self.add(),
                OpCode::SUB         => self.sub(),
                OpCode::MUL         => self.mul(),
                OpCode::DIV         => self.div(),
                OpCode::DEF         => {
                    match self.stack.pop().unwrap() {
                        Value::Str(name) => {
                            self.scopes.insert(name, self.stack.pop().unwrap());
                        },
                        _ => self.stack.push(Value::Error("invalid assignment")),
                    }
                },
                OpCode::GETNAME(n)  => self.stack.push(self.scopes.get(n).unwrap().clone()),
            }
            self.ip += 1;
        }
        self.running = false;
    }
}
