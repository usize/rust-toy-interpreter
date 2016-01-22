#![allow(non_snake_case)]

use std::collections::HashMap;
use std::io::{self, Write};

use compiler::*;
use parser::*;
use vm::*;
use value::Value;

mod compiler;
mod parser;
mod opcode;
mod object;
mod lexer;
mod value;
mod vm;

macro_rules! weak_try {
    ($func:expr) => {
        match $func {
            Ok(_) => (),
            Err(err) => println!("{:?}", err)
        };
    }
}

const VERSION: &'static str = "0.0.0";

fn main() {
    println!("Harvey {} (github.com/mrrrgn/harvey)", VERSION);

    let mut vm = VM::new();
    let mut parser = Parser::new();
    let mut scopes = HashMap::new();

    loop {
        let mut buffer = String::new();
        weak_try!(io::stdout().write(b"Harvey> "));
        weak_try!(io::stdout().flush());

        for _ in io::stdin().read_line(&mut buffer) {
            match parser.parse_lines(buffer.clone()) {
                Err(msg) => println!("{}", msg),
                Ok(statements) => {
                    println!("Parser: \n\t{:?}", &statements);
                    let script = compile_script(statements);
                    vm.load(script);
                    let result = vm.run(&mut scopes).unwrap();
                    println!("VM: \n\tstack: {:?}, \n\tprogram: {:?}\n", vm.stack(), vm.program());
                    match result {
                        Some(value) => println!("Harvey> {:?}", value),
                        None => ()
                    }
                }
            }
        }
    }
}

macro_rules! assert_ok {
    ($e: expr) => (
        match $e {
            Ok(x) => x,
            Err(err) => panic!("{:?}", err)
        }
    )
}

fn eval(code: &str) -> Value {
    let mut parser = Parser::new();
    let ast = assert_ok!(parser.parse_lines(code.to_string()));
    let script = compile_script(ast);

    let mut vm = VM::new();
    vm.load(script);

    let mut scopes = HashMap::new();
    assert_ok!(vm.run(&mut scopes)).expect("script did not produce a value")
}

#[test]
fn it_works() {
    assert_eq!(eval("2 + 2"), Value::Number(4.0));
    assert_eq!(eval("0 / 0"), Value::Number(std::f64::NAN));
}
