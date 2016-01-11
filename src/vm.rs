#![allow(dead_code)]

use value::*;

#[derive(Debug)]
pub enum OpCode {
    ADD, // stack.pop() + stack.pop()
    SUB, // stack.pop() - stack.pop()
    MUL, // stack.pop() * stack.pop()
    DIV, // stack.pop() / stack.pop()
}

pub struct VM {
    program:    Vec<OpCode>,
    //callstack:  Vec<usize>,
    stack:      Vec<Value>,
    ip:         usize,
    running:    bool
}

impl VM {
    pub fn new() -> VM {
        return VM{
            program: Vec::new(),
            /*callstack: Vec::new(),*/
            stack: Vec::new(),
            ip: 0,
            running: false
        };
    }

    pub fn load(&mut self, program: Vec<OpCode>, data: Vec<Value>) {
        self.program = program;
        self.stack = data;
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
                OpCode::ADD => self.add(),
                OpCode::SUB => self.sub(),
                OpCode::MUL => self.mul(),
                OpCode::DIV => self.div(),
            }
            self.ip += 1;
        }
        self.running = false;
    }
}
